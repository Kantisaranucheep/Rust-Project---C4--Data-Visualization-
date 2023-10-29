use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::collections::HashMap;
use plotters::style::text_anchor::Pos;
use plotters::style::text_anchor::HPos;
use plotters::style::text_anchor::VPos;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the output file name
    const OUT_FILE_NAME: &str = "line_chart16.png";

    // Define months at a higher scope
    let months = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];

    // Create a drawing area
    let root = BitMapBackend::new(OUT_FILE_NAME, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart context with a custom X-axis label
    let mut chart = ChartBuilder::on(&root)
        .caption("Monthly Bangkok Mass Transit system(BTS) passengers from 2018-2023", ("sans-serif", 60))
        .x_label_area_size(100)
        .y_label_area_size(100)
        .margin(55)
        .build_cartesian_2d(0..11, 0..25000000)?; // Adjust the X-axis range

    let custom_x_label_formatter = |x: &i32| -> String {
        // Your custom formatting logic here
        if let Some(month) = months.get(*x as usize) {
            month.to_string()
        } else {
            "Unknown".to_string() // Handle out-of-range values gracefully
        }
    };

    let _pos = Pos::new(HPos::Left, VPos::Top);
    let label_style = TextStyle::from(("sans-serif", 40));
    let x_desc_style = TextStyle::from(("sans-serif", 40)); // Set the desired font size for X-axis description

    chart.configure_mesh()
        .x_labels(11)
        .x_label_formatter(&custom_x_label_formatter)
        .y_labels(25)
        .x_desc("Month")
        .y_desc("Passengers")
        .axis_desc_style(label_style) // Apply the custom style to the X-axis description
        .draw()?;

    // Read data from your CSV file
    let file_path = "src/csv (1).csv"; // Replace with your data file path
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Read the column headers to determine the number of lines
    let headers = rdr.headers()?.clone();
    let num_lines = headers.len() - 1; // Excluding the second column (X-axis)

    // Create a mapping from month names to numerical values
    let month_mapping: HashMap<String, i32> = months.iter().enumerate()
        .map(|(index, month)| (month.to_string(), index as i32))
        .collect();

    // Define the X-axis range based on the unique months
    let x_range: std::ops::Range<i32> = 0..11;

    for line_index in 0..num_lines {
        let mut series_data = Vec::new();
        let mut point_data = Vec::new(); // Data for points

        // Clone the file handle to reuse it
        let file_clone = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file_clone);

        for result in rdr.records() {
            let record = result?;

            // Extract and transform data from your CSV record as needed
            let x_month = &record[1]; // Month name
            let x: i32 = *month_mapping.get(x_month).unwrap(); // Map month name to numerical value
            let y: i32 = record[line_index + 2].parse()?; // Adjust column index

            series_data.push((x, y));

            // Add point data
            point_data.push((x, y));
        }

        let color = Palette99::pick(line_index); // Use different colors for each line
        let series_style = ShapeStyle::from(&color).filled().stroke_width(2);

        chart
            .draw_series(LineSeries::new(series_data, series_style))?
            .label(&headers[line_index + 2]); // Use column headers as series labels

        // Add points to the line
        let point_style = ShapeStyle::from(&color).filled();

        chart
            .draw_series(
                point_data
                    .into_iter()
                    .map(|(x, y)| Circle::new((x, y), 5, point_style.clone()))
                    .into_iter(),
            )?;
            
        chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .position(SeriesLabelPosition::LowerRight) // Adjust the legend position as needed
        .label_font(("sans-serif", 20))
        .border_style(&BLACK)
        .draw()?;
                
    }

    // Add labels under the X-axis for line descriptions
    let label_areas: Vec<(i32, &str)> = vec![
        (0, "2018"), (1, "2019"), (2, "2020"), (3, "2021"), (4, "2022"), (5, "2023")
    ];

// Add labels under the X-axis for line descriptions
    let label_data: Vec<(i32, i32)> = label_areas.iter().map(|(x, _)| (*x, 0)).collect();

    for (x, label) in label_areas {
        chart.draw_series(
            label_data.iter().map(|(lx, _)| {
                plotters::element::Text::new(label, (*lx, 0), TextStyle::from(("sans-serif", 20)).color(&BLACK))
            }),
        )?;
    }


    // Save the chart as an image
    root.present()?;

    Ok(())
}

