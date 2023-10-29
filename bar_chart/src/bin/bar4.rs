
use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

struct BarChart {
    x_data: Vec<String>,
    y_data: Vec<f64>,
}

impl BarChart {
    fn new(x_data: Vec<String>, y_data: Vec<f64>) -> Self {
        BarChart { x_data, y_data }
    }

    fn generate_chart(&self, output_path: &str) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(output_path, (2500, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let x_labels: Vec<String> = self.x_data.clone(); // Clone city names
        let mut chart = ChartBuilder::on(&root)
            .caption("Average Life Expectancy in each city ", ("sans-serif", 40))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Right, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .margin(20)
            .build_cartesian_2d(0.0..self.x_data.len() as f32, 0.0..90.0)?;

        chart.configure_mesh()
            .x_labels(45)
            .y_labels(20)
            
            .x_label_formatter(&|x| {
                // Customize x-labels to use city names
                if (0.0..x_labels.len() as f32).contains(x) {
                    return x_labels[*x as usize].clone(); // Clone the label
                }
                "".to_string()
            })
            .x_desc("Cities")
            .y_desc("Life Expectancy (years)")
            .axis_desc_style(TextStyle::from(("sans-serif", 20)))
            .draw()?;

        let bars = self.x_data.iter().enumerate().map(|(i, _)| {
            let x = i as f32;
            let y = self.y_data[i];
            (x, y)
        });

        let mut blue_color = true; // Flag to alternate colors

        chart.draw_series(
            bars.map(|(x, y)| {
                let color = if blue_color {
                    blue_color = false;
                    BLUE.mix(0.7)
                } else {
                    blue_color = true;
                    CYAN.mix(0.7)
                };

                Rectangle::new([(x, 0.0), (x + 0.6, y)], color.filled())
            }),
        )?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/healthy_lifestyle_city_2021.csv"; // Replace with your CSV file path
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut x_data = Vec::<String>::new();
    let mut y_data = Vec::<f64>::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(x_str) = record.get(0) {
            // Use column 6 (city names)
            x_data.push(x_str.to_string());
        }
        if let Some(y_str) = record.get(5) {
            if let Ok(y_val) = y_str.parse::<f64>() {
                y_data.push(y_val);
            }
        }
    }

    let bar_chart = BarChart::new(x_data, y_data);
    bar_chart.generate_chart("bar_chart25.png")?;

    Ok(())
}

