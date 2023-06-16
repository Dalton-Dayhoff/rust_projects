use std::{fs::File, collections::HashMap, io::Read, vec, f64::consts::TAU};
use toml::{self, Table, de::Error as TomlError};
use serde::Deserialize;

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
#[derive(Deserialize, Debug)]
struct OrbitalElements {
    semimajor_axis: f64, // km
    eccentricity: f64, // none
    inclination: f64,  // radians
    mean_anomoly: f64, // radians
    argument_of_parigee: f64, // radians
    mass: f64, // kg 
    mu: Option<f64>, // km^3/s^2
    h: Option<f64> // km^2/s
}

impl OrbitalElements {
    fn new(data: [f64; 6]) -> Self{
        Self{
            semimajor_axis: data[0],
            eccentricity: data[1],
            inclination: wrap_angle(data[2].to_radians()),
            mean_anomoly: wrap_angle(data[3].to_radians() % (2.0 * std::f64::consts::PI)),
            argument_of_parigee: wrap_angle(data[4].to_radians() % (2.0 * std::f64::consts::PI)),
            mass: data[5],
            mu: None,
            h: None
        }
    }
}


/// In my simulation, there are two types of bodies
#[derive(Deserialize, Debug)]
enum BodyType {
    Planet,
    Satellite
}

/// Data for a body, includes a reference to requisite orbital data
#[derive(Deserialize, Debug)]
struct Body {
    coords: Vec<[f64; 3]>,
    vel: Vec<[f64; 3]>,
    radius: i32,
    orbit_data: OrbitalElements,
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
        let mean_anomaly = data[3] - data[4];
        Self {
            coords: vec![[data[0], 0.0, 0.0]],
            vel: vec![[0.0; 3]],
            radius: data[7] as i32,
            moons: None,
            orbit_data: OrbitalElements::new([data[0], 
                data[1], 
                data[2], 
                mean_anomaly, 
                arg_of_perigee, 
                data[7]]),
            importance: BodyType::Planet
        }
    }

    /// Same as but withwith moons/satellites
    fn with_moons(data: Vec<f64>, moons_: HashMap<String, Body>) -> Self{
        let arg_of_perigee = data[4] - data[5];
        let mean_anomaly = data[3] - data[4];
        Self {
            coords: vec![[data[0], 0.0, 0.0]],
            vel: vec![[0.0; 3]],
            radius: data[7] as i32,
            moons: Some(moons_),
            orbit_data: OrbitalElements::new([data[0], 
                data[1], 
                data[2], 
                mean_anomaly, 
                arg_of_perigee, 
                data[7]]),
            importance: BodyType::Planet
        }
    }

    /// Same as normal new function with exception that it is used for satellites/moons
    fn new_satellite(data: Vec<f64>) -> Self {
        let arg_of_perigee = data[4] - data[5];
        let mean_anomaly = data[3] - data[4];
        Self {
            coords: vec![[data[0], 0.0, 0.0]],
            vel: vec![[0.0; 3]],
            radius: data[7] as i32,
            moons: None,
            orbit_data: OrbitalElements::new([data[0], 
                data[1], 
                data[2], 
                mean_anomaly, 
                arg_of_perigee, 
                data[7]]),
            importance: BodyType::Satellite
        }
    }
}

/// Struct holding the hashmap of all bodies, 
/// it is a struct because I may add more elements in the future (such as epoch)
#[derive(Deserialize)]
pub struct SolarSystem{
    bodies: HashMap<String, Body>
}

impl SolarSystem{
    fn new(bodies_in_system: HashMap<String, Body>) -> Self{
        Self{
            bodies: bodies_in_system
        }
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
    let mut number_of_bodies = 0;
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
            SolarSystem::new(system)
        }
        Err(e) => panic!("Could not read Toml {}", e)
    }
}




