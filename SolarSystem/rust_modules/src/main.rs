mod planet;
use planet::{setup_from_toml, SolarSystem};
use plotters::{prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, PathElement}, series::LineSeries, style::{RED, WHITE, Color, RGBColor}};
use rand::Rng;



fn main()-> Result<(), Box<dyn std::error::Error>> {
    let mut system: SolarSystem = setup_from_toml();
    system.setup_system();
    let planet_names:Vec<String> = system.bodies.keys().cloned().collect();
    for i in 0..planet_names.len(){
        let mut total_time = 0.0;
        while total_time < system.bodies[&planet_names[i]].orbit_data.orbital_period{
            total_time += system.bodies[&planet_names[i]].orbit_data.other.propogation_period;
            system.bodies.get_mut(&planet_names[i]).map(|val| val.propagate(total_time));
        }
    }
    // Create a new drawing area
    let root = BitMapBackend::new("orbit.png", (800, 600)).into_drawing_area();

    // Set up a 3D chart context
    let coords = system.bodies["Neptune"].coords.clone(); // Clone the coordinates
    let y_data: Vec<f64> = coords.iter().map(|coord| coord[1]).collect();
    let value = y_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let limit = 10.0_f64.powf(value.log10().ceil() - 0.35);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_3d(-limit..limit, -limit..limit, -limit..800000000.0)?;
    // Plot the orbits
    
    const NUM_COLORS: usize = 5; // Set the desired number of colors
    const MAX_COLOR_VALUE: u8 = 255; // Set the maximum value for each color channel

    let mut rng = rand::thread_rng();
    let mut colors = vec![];
    for i in 0..system.number_of_bodies{
        colors.push(RGBColor(rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255)));
    }
    for (i, name) in planet_names.iter().enumerate() {
        let coords = system.bodies[name].coords.clone();
        let x_data: Vec<f64> = coords.iter().map(|coord| coord[0]).collect();
        let y_data: Vec<f64> = coords.iter().map(|coord| coord[1]).collect();
        let z_data: Vec<f64> = coords.iter().map(|coord| coord[2]).collect();

        let color = colors[i];

        chart
            .draw_series(LineSeries::new(
                (x_data.iter().zip(y_data.iter()).zip(z_data.iter()))
                    .map(|((&x, &y), &z)| (x, y, z)),
                &color,
            ))?
            .label(name)
            .legend(move |(x, y)| {
                let color = color.clone(); // Clone the color within the closure
                PathElement::new(vec![(x, y), (x + 20, y)], &color)
            });
    }
    
    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;
    root.present()?;

    Ok(())
}
