use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use plotters::style::RGBColor;
use plotters::series::Histogram;

fn main() -> Result<(), Box<dyn Error>> {
    // Read data from CSV file
    let file_path = "src/input.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut x = Vec::<f64>::new();

    for result in rdr.records() {
        let record = result?;

        if record.len() >= 1 {
            if let Some(x_str) = record.get(0) {
                if let Ok(x_val) = x_str.parse::<f64>() {
                    x.push(x_val);
                } else {
                    eprintln!("Error: Parsing data");
                }
            } else {
                eprintln!("Error: Missing columns");
            }
        } else {
            eprintln!("Error: Not enough columns");
        }
    }

    let max_score = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_score = x.iter().cloned().fold(f64::INFINITY, f64::min);
    
    // Determine an appropriate number of bins based on your data and distribution.
    let num_bins = 10; // Adjust this value as needed.

    let root = BitMapBackend::new("Histogram4.png", (800, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("class 6/8 final exam score", ("sans-serif", 50.0))
        .build_cartesian_2d((min_score as u32..max_score as u32).into_segmented(), 0u32..num_bins)?;

    chart
        .configure_mesh()
        // .disable_x_mesh()

        .x_desc("Score")
        .y_desc("Frequency")
        .bold_line_style(&WHITE.mix(0.3))
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .margin(0)
            .data(x.iter().map(|&value| (value as u32, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under the current dir");
    println!("Result has been saved to {}", "Histogram3.png");

    Ok(())
}

