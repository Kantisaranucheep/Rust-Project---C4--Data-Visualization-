use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

struct BarChart {
    x_data: Vec<String>,
    y_data: Vec<i32>,
}

impl BarChart {
    fn new(x_data: Vec<String>, y_data: Vec<i32>) -> Self {
        BarChart { x_data, y_data }
    }

    fn generate_chart(&self, output_path: &str) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(output_path, (2400, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let x_labels: Vec<String> = self.x_data.clone(); 
        let mut chart = ChartBuilder::on(&root)
            .caption("Average attendance in each premier league stadium (season 2023/2024) ", ("sans-serif", 40))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Right, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .margin(20)
            .build_cartesian_2d(0.0..self.x_data.len() as f32, 0..80000)?;

        chart.configure_mesh()
            .x_labels(21)
            .y_labels(20)
            
            .x_label_formatter(&|x| {
                // Customize x-labels to use city names
                if (0.0..x_labels.len() as f32).contains(x) {
                    return x_labels[*x as usize].clone(); 
                }
                "".to_string()
            })
            .x_desc("Teams")
            .y_desc("Average Attendances")
            .axis_desc_style(TextStyle::from(("sans-serif", 20)))
            .draw()?;

        let bars = self.x_data.iter().enumerate().map(|(i, _)| {
            let x = i as f32;
            let y = self.y_data[i] as i32;
            (x, y)
        });

        let mut green_color = true; 

        chart.draw_series(
            bars.map(|(x, y)| {
                let color = if green_color {
                    green_color = false;
                    RGBColor(34,139,34).mix(0.7)
                } else {
                    green_color = true;
                    RGBColor(154,205,50).mix(0.7)
                };

                Rectangle::new([(x, 0), (x + 0.7, y)], color.filled())
                // Text::new(format!("{:?}", c), (x, 0), ("sans-serif", 10).into_font());
            }),
        )?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/export.csv"; 
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut x_data = Vec::<String>::new();
    let mut y_data = Vec::<i32>::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(x_str) = record.get(1) {

            x_data.push(x_str.to_string());
        }
        if let Some(y_str) = record.get(2) {
            if let Ok(y_val) = y_str.parse::<i32>() {
                y_data.push(y_val);
            }
        }
    }

    let bar_chart = BarChart::new(x_data, y_data);
    bar_chart.generate_chart("bar_chart23.png")?;

    Ok(())
}
