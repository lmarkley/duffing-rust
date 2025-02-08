use std::f64::consts::PI as PI;
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "duffing.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MASS : f64 = 1.0;
    const VISCOSITY : f64 = 0.15;
    const SPRING1 : f64 = 0.5;
    const SPRING3 : f64 = 0.5;

    let mut acceleration : f64 = 0.0;
    let mut velocity : f64 = 1.0;
    let mut position : f64 = 1.0;
    let mut time : f64 = 0.0;
    let frequency : f64 = 0.833; 
    let period : f64 = (2.0 * PI)/frequency;
    let amplitude : f64 = 0.19;
    const SAMPLE_POINTS : usize = 2000;
    let points_per_period : usize = 1000;

    let dt : f64 = period/(points_per_period as f64);


    let mut position_array: [f64; SAMPLE_POINTS] = [0.0; SAMPLE_POINTS];
    let mut velocity_array: [f64; SAMPLE_POINTS] = [0.0; SAMPLE_POINTS];

    for j in 0..points_per_period {
        position_array[j] = position;
        velocity_array[j] = velocity;
        for _i in 0..SAMPLE_POINTS {
            
            acceleration = (-VISCOSITY * velocity + SPRING1 * position - SPRING3*position*position*position + amplitude * (frequency*time).cos())/MASS;
            velocity += acceleration * dt;
            position += velocity * dt;
            time += dt;
        }
    }

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut scatter = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .caption("Duffing Oscillator", ("sans-serif", 30.0).into_font())
        .build_cartesian_2d(-1.5f64..1.5f64,-0.5f64..0.8f64)?;

    scatter.configure_mesh()
        .x_desc("Position")
        .y_desc("Velocity")
        .draw()?;

    scatter.draw_series(
        position_array.iter()
        .zip(velocity_array.iter())
        .map(|(x,y)| Circle::new((*x,*y), 2, BLUE.filled())),
        )?;


    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);


    Ok(())

}
