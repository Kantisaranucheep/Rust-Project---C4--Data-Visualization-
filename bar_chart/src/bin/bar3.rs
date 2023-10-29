use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::collections::HashMap;

#[derive(Debug)]
struct RegionData {
    year_2543: f64,
    year_2553: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define the output file name
    const OUT_FILE_NAME: &str = "Bar_chart15.png";

    let region = vec![
        "whole country", "Bangkok", "Central", "North", "North-East", "South", 
    ];

    // Create a drawing area
    let root = BitMapBackend::new(OUT_FILE_NAME, (1600, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Read data from your CSV file
    let file_path = "src/density2.csv"; // Replace with your data file path
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Create a vector to store data for each region
    let mut year_data: HashMap<String, RegionData> = HashMap::new();

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        let region_name = record[1].to_string();
        if i % 2 == 0 {
            if let Ok(year_2543) = record[2].parse::<f64>() {
                let entry = year_data
                    .entry(region_name.clone())
                    .or_insert(RegionData {
                        year_2543,
                        year_2553: 0.0, // Initialize year_2553 to 0.0
                    });
            } else {
                eprintln!("Error: Invalid data in the 2543 column for region '{}'", region_name);
            }
        } else {
            if let Ok(year_2553) = record[2].parse::<f64>() {
                if let Some(entry) = year_data.get_mut(&region_name) {
                    entry.year_2553 = year_2553;
                }
            } else {
                eprintln!("Error: Invalid data in the 2553 column for region '{}'", region_name);
            }
        }
    }
    

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Population Density by Region in Thailand in year 2543 and 2553",
            ("sans-serif", 60),
        )
        .x_label_area_size(60)
        .y_label_area_size(100)
        .margin(20)
        .build_cartesian_2d(
            // Define the range for x-axis
            0.0..region.len() as f64,
            // Define the range for the y-axis
            0.0..5400.0,
        )?;

    let custom_x_label_formatter = |x: &f64| -> String {
        if *x >= 0.0 && *x < region.len() as f64 {
            let position = *x as usize;
            region[position].to_string()
        } else {
            "".to_string()
        }
    };

    // Configure chart mesh as before
    chart.configure_mesh()
        .x_labels(12)
        .x_label_formatter(&custom_x_label_formatter)
        .y_labels(54)
        .y_desc("Population Density (Per square kilometer)")
        .x_desc("Region in Thailand")
        .axis_desc_style(TextStyle::from(("sans-serif", 30)))
        .draw()?;

    for (i, &region_name) in region.iter().enumerate() {
        if let Some(data) = year_data.get(&region_name.to_string()) {
            let x_pos = i as f64;

            let bar_2543 = Rectangle::new(
                [(x_pos, 0.0), (x_pos + 0.4, data.year_2543)],
                BLUE.filled(), // Make the 2543 bars blue
            );

            let x_pos_2553 = x_pos + 0.5; // Adjust the offset for the next year
            let bar_2553 = Rectangle::new(
                [(x_pos_2553, 0.0), (x_pos_2553 + 0.4, data.year_2553)],
                RED.filled(), // Make the 2553 bars red
            );

            let _ = chart.draw_series(vec![bar_2543, bar_2553]);
        }
    }

    // Save the bar chart as an image
    root.present()?;

    Ok(())
}

