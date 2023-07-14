use std::{fs::File, collections::HashMap, io::Read, vec, f64::consts::{TAU, PI, }};
use toml::{self, Table, de::Error as TomlError};
use serde::Deserialize;
use ndarray::{array, Array1, Array2};

const GRAVITATIONAL_CONSTANT: f64 = 6.6743E-11;
const MASS_OF_SUN: f64 = 1.99E30;
const DELTA: f64 = 1.0E-10;

/// Takes an angle in radians and wraps it between 0 and TAUT (2*pi)
fn wrap_angle(angle: f64) -> f64 {
    let wrapped_angle = angle % TAU;
    if wrapped_angle < 0.0 {
        wrapped_angle + TAU
    }
    else {
        wrapped_angle
    }
}

/// base elements required to from an orbit
#[derive(Deserialize, Debug, Clone)]
pub struct OrbitalElements {
    semimajor_axis: f64, // km
    eccentricity: f64, // none
    inclination: f64,  // radians
    raan: f64, // radians, right ascension of ascending node
    argument_of_parigee: f64, // radians
    mass: f64, // kg 
    pub orbital_period: f64, // period of the orbit
    pub other: DerivedElements // struct of elements to derive
}

#[derive(Deserialize, Debug, Clone)]
pub struct DerivedElements {
    mu: f64, // km^3/s^2
    angular_momentum: f64, // km^2/s
    pariapse: f64, // km
    apoapse: f64, // km
    pub true_anomaly: f64, // radians
    mean_anomaly: f64, // radians
    eccentric_anomaly: f64, // radians
    rot_mat: Array2<f64>, // no units, converts perifocal frame to heliocentric/frame of planet for sat/moons
    pub propogation_period: f64 // period between orbit propogation points
}

impl DerivedElements {
    fn new() -> Self {
        Self {
            mu: 0.0,
            angular_momentum: 0.0,
            pariapse: 0.0,
            apoapse: 0.0,
            true_anomaly: 0.0,
            mean_anomaly: 0.0,
            eccentric_anomaly: 0.0,
            rot_mat: Array2::zeros((3, 3)),
            propogation_period: 0.0
        }
    }
}

impl OrbitalElements {
    fn new(data: [f64; 6]) -> Self{
        Self{
            semimajor_axis: data[0],
            eccentricity: data[1],
            inclination: wrap_angle(data[2].to_radians()),
            raan: wrap_angle(data[3].to_radians() % (2.0 * std::f64::consts::PI)),
            argument_of_parigee: wrap_angle(data[4].to_radians() % (2.0 * std::f64::consts::PI)),
            mass: data[5],
            other: DerivedElements::new(),
            orbital_period: 0.0,
        }
    }
}


/// In my simulation, there are two types of bodies
#[derive(Deserialize, Debug, Clone)]
enum BodyType {
    Planet,
    Satellite
}

/// Data for a body, includes a reference to requisite orbital data
#[derive(Deserialize, Debug, Clone)]
pub struct Body {
    pub coords: Vec<Array1<f64>>,
    pub vel: Vec<Array1<f64>>,
    radius: i32,
    pub orbit_data: OrbitalElements,
    moons: Option<HashMap<String, Body>>,
    importance: BodyType,
}

impl Body{
    /// The creation of a now body.
    /// data: semi major axis [0]
    ///       eccentricity [1]
    ///       inclination [2] 
    ///       mean_longitude [3]
    ///       longitude of perigee [4]
    ///       longitude of ascending node [5]
    ///       radius of body [6]
    ///       mass of body [7]
    fn new(data: Vec<f64>) -> Self{
        let arg_of_perigee = data[4] - data[5];
        Self {
            coords: vec![Array1::zeros(3)],
            vel: vec![Array1::zeros(3)],
            radius: data[7] as i32,
            moons: None,
            orbit_data: OrbitalElements::new([data[0], 
                data[1], 
                wrap_angle(data[2].to_radians()), 
                wrap_angle(data[5].to_radians()), 
                wrap_angle(arg_of_perigee.to_radians()), 
                data[7]]),
            importance: BodyType::Planet
        }
    }

    /// Same as but withwith moons/satellites
    fn with_moons(data: Vec<f64>, moons_: HashMap<String, Body>) -> Self{
        let arg_of_perigee = data[4] - data[5];
        Self {
            coords: vec![Array1::zeros(3)],
            vel: vec![Array1::zeros(3)],
            radius: data[7] as i32,
            moons: Some(moons_),
            orbit_data: OrbitalElements::new([data[0], 
                data[1], 
                data[2].to_radians(), 
                data[5].to_radians(), 
                arg_of_perigee.to_radians(), 
                data[7]]),
            importance: BodyType::Planet
        }
    }
    /// Same as normal new function with exception that it is used for satellites/moons
    fn new_satellite(data: Vec<f64>) -> Self {
        let arg_of_perigee = data[4] - data[5];
        Self {
            coords: vec![Array1::zeros(3)],
            vel: vec![Array1::zeros(3)],
            radius: data[7] as i32,
            moons: None,
            orbit_data: OrbitalElements::new([data[0], 
                data[1], 
                data[2].to_radians(), 
                data[5].to_radians(), 
                arg_of_perigee.to_radians(), 
                data[7]]),
            importance: BodyType::Satellite
        }
    }
    
    fn setup_orbit_data(&mut self, major_body: Option<&Body>) {
        // Define mu
        match major_body {
            Some(major) => self.orbit_data.other.mu = GRAVITATIONAL_CONSTANT*(
                self.orbit_data.mass as f64 + major.orbit_data.mass as f64),
            _ => self.orbit_data.other.mu = GRAVITATIONAL_CONSTANT*(self.orbit_data.mass as f64 + MASS_OF_SUN),
        }

        // Define apsis
        self.orbit_data.other.pariapse = self.orbit_data.semimajor_axis * (1.0 - self.orbit_data.eccentricity);
        self.orbit_data.other.apoapse = self.orbit_data.semimajor_axis * (1.0 + self.orbit_data.eccentricity);

        // Define specific angular momentum
        self.orbit_data.other.angular_momentum = (self.orbit_data.other.mu * 
            self.orbit_data.other.pariapse * (1.0 + self.orbit_data.eccentricity)).sqrt();
        
        // Define orbital period
        self.orbit_data.orbital_period =2.0 * PI * ((self.orbit_data.semimajor_axis * 10.0_f64.powf(3.0)).powf(1.5) / self.orbit_data.other.mu.sqrt());
        // Using period, find time period to propagate over
        // Each orbit uses 1000 segmants
        self.orbit_data.other.propogation_period = self.orbit_data.orbital_period/1000.0;

        // Set initial position
        // We start with a true anomaly of 0, which means we can set the perifocal coords easily
        // p_hat direction
        let pf_coords = array![self.orbit_data.other.pariapse, 0.0, 0.0];
        // velocity in pf frame, q_hat direction
        let pf_vel = array![0.0, self.orbit_data.other.angular_momentum/self.orbit_data.other.pariapse, 0.0];

        // Find rotation matrix from pf to heliocentric frame
        let w = self.orbit_data.argument_of_parigee;
        let inc = self.orbit_data.inclination;
        let raan = self.orbit_data.raan;
        let rot_w = array![[w.cos(), -w.sin(), 0.0],
                [w.sin(), w.cos(), 0.0],
                [0.0, 0.0, 1.0]];
        let rot_i = array![[1.0, 0.0, 0.0],
                [0.0, inc.cos(), inc.sin()],
                [0.0, -inc.sin(), inc.cos()]];
        let rot_raan = array![[raan.cos(), -raan.sin(), 0.0],
                [raan.sin(), raan.cos(), 0.0],
                [0.0, 0.0, 1.0]];
        self.orbit_data.other.rot_mat = rot_raan.dot(&rot_i.dot(&rot_w));
        self.coords[0] = self.orbit_data.other.rot_mat.dot(&pf_coords);
        self.vel[0] = self.orbit_data.other.rot_mat.dot(&pf_vel);
    }


    /// Find orbital data at some given time from periapse
    pub fn propagate(&mut self, time: f64){
        self.orbit_data.other.mean_anomaly = wrap_angle((2.0 * PI / self.orbit_data.orbital_period )* time);
        let initial_ee = if self.orbit_data.other.mean_anomaly < PI 
            {self.orbit_data.other.mean_anomaly + self.orbit_data.eccentricity/2.0} 
            else {self.orbit_data.other.mean_anomaly - self.orbit_data.eccentricity/2.0};
        self.orbit_data.other.eccentric_anomaly = wrap_angle(find_eccentric_anomaly(
            self.orbit_data.other.mean_anomaly.clone(), 
            self.orbit_data.eccentricity.clone(), 
            initial_ee));
        let coefficient = (1.0 + self.orbit_data.eccentricity)/(1.0 - self.orbit_data.eccentricity);
        self.orbit_data.other.true_anomaly = wrap_angle(2.0*(coefficient*(self.orbit_data.other.eccentric_anomaly/2.0).tan()).atan());
        let r_mag = (self.orbit_data.other.angular_momentum.powi(2)/self.orbit_data.other.mu) 
            * 1.0/(1.0 + self.orbit_data.eccentricity*self.orbit_data.other.true_anomaly.cos());
        let r_pf = array![
            r_mag*self.orbit_data.other.true_anomaly.cos(), 
            r_mag*self.orbit_data.other.true_anomaly.sin(), 
            0.0];
        let v_pf = (self.orbit_data.other.mu/self.orbit_data.other.angular_momentum)*array![
            -self.orbit_data.other.true_anomaly.sin(), 
            self.orbit_data.eccentricity + self.orbit_data.other.true_anomaly.cos(), 
            0.0];
        self.coords.push(self.orbit_data.other.rot_mat.dot(&r_pf));
        self.vel.push(self.orbit_data.other.rot_mat.dot(&v_pf));
    }



}

fn find_eccentric_anomaly(mean_anomaly: f64, eccentricity: f64, eccentric_anomaly: f64) -> f64{
    let numerator = eccentric_anomaly - eccentricity* eccentric_anomaly.sin() - mean_anomaly;
    let denomonator = 1.0 - eccentricity*eccentric_anomaly.cos();
    let new_eccentric_anomaly = eccentric_anomaly - numerator/denomonator;
    if (new_eccentric_anomaly - eccentric_anomaly).abs() < DELTA {
        return new_eccentric_anomaly;
    }
    find_eccentric_anomaly(mean_anomaly, eccentricity, new_eccentric_anomaly)
}

/// Struct holding the hashmap of all bodies, 
/// it is a struct because I may add more elements in the future (such as epoch)
#[derive(Deserialize, Clone)]
pub struct SolarSystem{
    pub bodies: HashMap<String, Body>,
    pub number_of_bodies: i32
}

impl SolarSystem{
    fn new(bodies_in_system: HashMap<String, Body>, bodies: i32) -> Self{
        Self{
            bodies: bodies_in_system,
            number_of_bodies: bodies
        }
    }

    pub fn setup_system(&mut self) {
        let mut bodies: Vec<(&String, &mut Body)> = self.bodies.iter_mut().collect();
        let mut updated_bodies = HashMap::new();
        for (name, value) in bodies.iter_mut(){
            let body = value.clone();
            if let Some(moons) = &mut value.moons{
                let mut updated_moons = HashMap::new();
                for (m_name, moon) in moons.iter_mut(){
                    let mut moon_copy = moon.clone();
                    moon_copy.setup_orbit_data(Some(&body));
                    updated_moons.insert(m_name.clone(), moon_copy);
                }
                *moons = updated_moons;
            }
            value.setup_orbit_data(None);
            updated_bodies.insert(name.clone(), value.clone());
        }
        // Reset the struct
        self.bodies = updated_bodies; 
    }
}

/// More comments throughout but reades the toml of data for bodies in the system, packs the structs,
/// and returns a full instance of SolarSystem
pub fn setup_from_toml() -> SolarSystem{
    // Read the TOML file
    let mut file = File::open("../data/celestial_bodies_data.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse the TOML into a struct
    let config: Result<Table, TomlError> = toml::from_str(&contents);
    let number_of_bodies: i32;
    let mut system = HashMap::new();
    match config {
        Ok(config) => {
            // Set number of bodies
            number_of_bodies = config.get("number_of_bodies").and_then(toml::Value::as_integer).expect("Could not find the number of bodies").try_into().unwrap();
            // Iterate through bodies in the solar system, as specified in data.toml
            for (name, body) in config.get("SolarSystem").and_then(|s| s.as_table()).expect("SolarSystem not found in TOML").iter(){
                if let Some(moons) = body.get("moons").and_then(toml::Value::as_table){
                    let mut map_o_moons = HashMap::new();
                    // Creating moons
                    // could be satellites
                    for (moon_name, moon) in moons{
                        map_o_moons.insert(moon_name.clone(), 
                        // Getting each value from the toml is very similiar
                        // 1. check for value .get()
                        // 2. convert it from a toml value to its requisite type .and_then
                        // 3. panic and give a message if data doesn't exist .expect()
                        Body::new_satellite(vec![moon.get("semi_major_axis_km").and_then(toml::Value::as_float).expect("Couldn't find semimajor axis of a moon"), 
                        moon.get("eccentricity").and_then(toml::Value::as_float).expect("Couldn't find eccentricity of a moon") , 
                        moon.get("inclination_degrees").and_then(toml::Value::as_float).expect("Couldn't find inclination of a moon") ,
                        moon.get("mean_longitude_degrees").and_then(toml::Value::as_float).expect("Couldn't find mean longitude of a moon") ,
                        moon.get("longitude_of_perhelion_degrees").and_then(toml::Value::as_float).expect("Couldn't find longitude of perigee of a moon") ,
                        moon.get("longitude_of_the_ascending_node_degrees").and_then(toml::Value::as_float).expect("Couldn't find ascending node of a moon") ,
                        moon.get("meanradius_km").and_then(toml::Value::as_float).expect("Couldn't find radius of a moon") ,
                        moon.get("mass_kg").and_then(toml::Value::as_float).expect("Couldn't find mass of a moon") 
                    ]));
                    }
                    // Moons/satellites are created, create the body
                    system.insert(name.clone(), 
                        Body::with_moons(vec![body.get("semi_major_axis_km").and_then(toml::Value::as_float).expect("Couldn't find semimajor axis of a planet"), 
                        body.get("eccentricity").and_then(toml::Value::as_float).expect("Couldn't find eccentricity of a planet") , 
                        body.get("inclination_degrees").and_then(toml::Value::as_float).expect("Couldn't find inclination of a planet") ,
                        body.get("mean_longitude_degrees").and_then(toml::Value::as_float).expect("Couldn't find mean longitude of a plaent") ,
                        body.get("longitude_of_perhelion_degrees").and_then(toml::Value::as_float).expect("Couldn't find longitude of perigee of a planet") ,
                        body.get("longitude_of_the_ascending_node_degrees").and_then(toml::Value::as_float).expect("Couldn't find ascending node of a planet") ,
                        body.get("meanradius_km").and_then(toml::Value::as_float).expect("Couldn't find radius of a planet") ,
                        body.get("mass_kg").and_then(toml::Value::as_float).expect("Couldn't find mass of a planet") 
                    ], map_o_moons));
                }
                else {
                    // Create a body without any moons/satellites
                    system.insert(name.clone(), 
                        Body::new(vec![body.get("semi_major_axis_km").and_then(toml::Value::as_float).expect("Couldn't find semimajor axis of a planet"), 
                        body.get("eccentricity").and_then(toml::Value::as_float).expect("Couldn't find eccentricity of a planet") , 
                        body.get("inclination_degrees").and_then(toml::Value::as_float).expect("Couldn't find inclination of a planet") ,
                        body.get("mean_longitude_degrees").and_then(toml::Value::as_float).expect("Couldn't find mean longitude of a plaent") ,
                        body.get("longitude_of_perihelion_degrees").and_then(toml::Value::as_float).expect("Couldn't find longitude of perigee of a planet") ,
                        body.get("longitude_of_the_ascending_node_degrees").and_then(toml::Value::as_float).expect("Couldn't find ascending node of a planet") ,
                        body.get("meanradius_km").and_then(toml::Value::as_float).expect("Couldn't find radius of a planet") ,
                        body.get("mass_kg").and_then(toml::Value::as_float).expect("Couldn't find mass of a planet") 
                    ]));
                 }
            }
            // Pack and return the solar system
            SolarSystem::new(system, number_of_bodies)
        }
        Err(e) => panic!("Could not read Toml {}", e)
    }
}




