
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the output file name
    const OUT_FILE_NAME: &str = "Bar_chart8.png";

    // Define months at a higher scope
    let years = vec![
        "2561", "2562", "2563", "2564", "2565", "2566",
    ];

    // Create a drawing area
    let root = BitMapBackend::new(OUT_FILE_NAME, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Read data from your CSV file
    let file_path = "src/bts20182023.csv"; // Replace with your data file path
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Read the column headers
    let headers = rdr.headers()?.clone();

    // Create a mapping from column headers (years) to numerical values
    let years_mapping: HashMap<String, i32> = headers.iter()
        .enumerate()
        .skip(2) // Skip the first two columns (no, month)
        .map(|(index, year)| (year.to_string(), index as i32 - 2)) // Adjust the index
        .collect();

    // Create a vector to store data for each year
    let mut year_data: HashMap<String, Vec<(f64, f64)>> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        // Start processing data from the third column (index 2)
        for (i, value) in record.iter().enumerate().skip(2) {
            if let Ok(passengers) = value.parse::<f64>() {
                if let Some(&year_index) = years_mapping.get(&headers[i]) {
                    year_data
                        .entry(headers[i].to_string()) // Use the header as the year
                        .or_insert(Vec::new())
                        .push((year_index as f64, passengers));
                }
            } else {
                eprintln!("Error: Invalid data in column {}", i);
            }
        }
    }

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption("BTS passengers 2018-2023", ("sans-serif", 60))
        .x_label_area_size(60)
        .y_label_area_size(100)
        .margin(20)
        .build_cartesian_2d(0.0..6.0, 0.0..26000000.0)?; // Adjust the X-axis range

    // Define a custom X-axis label formatter
    let custom_x_label_formatter = |x: &f64| -> String {
        if let Some(year) = years.get(*x as usize) {
            year.to_string()
        } else {
            " ".to_string()
        }
    };

    // Configure chart mesh
    chart.configure_mesh()
        .x_labels(7)
        .x_label_formatter(&custom_x_label_formatter)
        .y_labels(26)
        .x_desc("Years")
        .y_desc("Passengers")
        .axis_desc_style(TextStyle::from(("sans-serif", 40))) // Apply the custom style to the X-axis description
        .draw()?;

    // Draw the bar chart
    for (_year, data) in &year_data {
        let _ = chart.draw_series(data.iter().map(|(x, y)| {
            let x_f64 = *x; // Keep x as f64
            let y_f64 = *y; // Keep y as f64
            let mut bar = Rectangle::new(
                [(x_f64, 0.0), (x_f64 + 0.6, y_f64)],
                CYAN.filled(),
            );
            bar.set_margin(0, 0, 5, 5);
            bar
        }));
    }

    Ok(())
}

