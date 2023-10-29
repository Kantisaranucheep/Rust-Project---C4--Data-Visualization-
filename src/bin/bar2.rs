use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the output file name
    const OUT_FILE_NAME: &str = "Bar_chart14.png";

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

    // Read the column headers
    let headers = rdr.headers()?.clone();

    // Create a mapping from column headers (years) to numerical values
    let mapping: HashMap<String, i32> = headers
        .iter()
        .enumerate()
        .skip(2) // Skip the first two columns (year, region)
        .map(|(index, year)| (year.to_string(), index as i32 - 2)) // Adjust the index
        .collect();

    println!("Mapping: {:?}", mapping);

    // Create a vector to store data for each year
    let mut year_data: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let region_name = record[1].to_string();
        for (i, value) in record.iter().enumerate().skip(2) {
            if let Ok(density) = value.parse::<f64>() {
                if let Some(year) = headers.get(i) {
                    if let Some(&year_index) = mapping.get(year) {
                        year_data
                            .entry(region_name.clone())
                            .or_insert(HashMap::new())
                            .insert(year.to_string(), density);
                    }
                }
            } else {
                eprintln!("Error: Invalid data in column {}", i);
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
            0.0..region.len() as f64 * 2.0, // Double the range to accommodate two bars per region
            // Define the range for the y-axis
            0.0..5400.0,
        )?;

    let custom_x_label_formatter = |x: &f64| -> String {
        if *x >= 0.0 && *x < region.len() as f64 * 2.0 {
            let position = (*x / 2.0) as usize;
            region[position].to_string()
        } else {
            "Unknown".to_string()
        }
    };

    // Configure chart mesh as before
    chart.configure_mesh()
        .x_labels(region.len() as usize * 2)
        .x_label_formatter(&custom_x_label_formatter)
        .y_labels(27)
        .y_desc("Population Density (Per square kilometer)")
        .x_desc("Region in Thailand")
        .axis_desc_style(TextStyle::from(("sans-serif", 30)))
        .draw()?;

        for (i, &region_name) in region.iter().enumerate() {
            if let Some(data) = year_data.get(&region_name.to_string()) {
                let x_pos = i as f64 * 2.0;
                
                if let Some(density_2543) = data.get("2543").cloned() {
                    let bar_2543 = Rectangle::new(
                        [(x_pos, 0.0), (x_pos + 0.40, density_2543)],
                        BLUE.filled(),
                    );
                    let _ = chart.draw_series(std::iter::once(bar_2543));
                } else {
                    eprintln!("Error: Data for year 2543 not found for region '{}'", region_name);
                }
        
                if let Some(density_2553) = data.get("2553").cloned() {
                    // Add a second bar for the year "2553"
                    let x_pos_next_year = x_pos + 0.5;
                    let bar_2553 = Rectangle::new(
                        [(x_pos_next_year, 0.0), (x_pos_next_year + 0.40, density_2553)],
                        RED.filled(),
                    );
                    let _ = chart.draw_series(std::iter::once(bar_2553));
                } else {
                    eprintln!("Error: Data for year 2553 not found for region '{}'", region_name);
                }
            }
        }
    

    // Save the bar chart as an image
    root.present()?;

    Ok(())
}

