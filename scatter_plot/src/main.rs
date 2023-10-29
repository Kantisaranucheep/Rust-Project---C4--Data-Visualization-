use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

struct ScatterPlot {
    x_data: Vec<f64>,
    y_data: Vec<f64>,
}

impl ScatterPlot {
    fn new(x_data: Vec<f64>, y_data: Vec<f64>) -> Self {
        ScatterPlot { x_data, y_data }
    }

    fn generate_plot(&self, output_path: &str) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(output_path, (1000, 600)).into_drawing_area();
        root.fill(&WHITE);

        let mut chart = ChartBuilder::on(&root)
            .caption("salary and years of experience",("sans-serif",40))
            .x_label_area_size(50)
            .y_label_area_size(70)
            .margin(20)
            .build_cartesian_2d(0.0..15.0, 0.0..200000.0)?;

        chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("experiences in years")
        .y_desc("salary in THB")
        .axis_desc_style(TextStyle::from(("sans-serif", 20)))
        .draw()?;

        chart.draw_series(
            self.x_data
                .iter()
                .zip(self.y_data.iter())
                .map(|(&x, &y)| Circle::new((x, y), 5, BLUE.filled())),
        )?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/Salary.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut age_data = Vec::<f64>::new();
    let mut tip_data = Vec::<f64>::new();

    for result in rdr.records() {
        let record = result?;

        if record.len() >= 2 {
            if let (Some(age_str), Some(tip_str)) = (record.get(0), record.get(1)) {
                if let (Ok(age_val), Ok(tip_val)) = (age_str.parse::<f64>(), tip_str.parse::<f64>()) {
                    age_data.push(age_val);
                    tip_data.push(tip_val);
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

    let scatter_plot = ScatterPlot::new(age_data, tip_data);
    scatter_plot.generate_plot("scatter_plot4.png")?;

    Ok(())
}


