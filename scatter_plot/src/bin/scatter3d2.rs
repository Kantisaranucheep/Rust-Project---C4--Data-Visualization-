use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

struct ScatterPlot {
    x_data: Vec<f64>,
    y_data: Vec<f64>,
    z_data: Vec<f64>,
}

impl ScatterPlot {
    fn new(x_data: Vec<f64>, y_data: Vec<f64>, z_data: Vec<f64>) -> Self {
        ScatterPlot { x_data, y_data, z_data }
    }

    fn generate_plot(&self, output_path: &str) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::gif(output_path, (1000, 800),100)?.into_drawing_area();
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption("salary, age and years of experience",("sans-serif",40))
            .margin(20)
            .build_cartesian_3d(0.0..12.0, 0.0..120000.0, 0.0..40.0,)?;

        chart.with_projection(|mut pb| {
            pb.pitch = 0.15;
            pb.yaw = 0.35;
            pb.scale = 0.8;
            pb.into_matrix()
        });
        chart.configure_axes()
        .x_labels(10)
        .y_labels(10)
        .z_labels(10) // Add labels for the Z-axis
        .draw()?;

        let points = self.x_data
            .iter()
            .zip(self.y_data.iter())
            .zip(self.z_data.iter())
            .map(|((&x, &y), &z)| (x, y, z));

        chart.draw_series(
            points
                .map(|(x, y, z)| {
                    Circle::new((x, y, z), 3, RED.filled())
                })
        )?;

        

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/Salary_Data.csv"; 
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut x_data = Vec::<f64>::new();
    let mut y_data = Vec::<f64>::new();
    let mut z_data = Vec::<f64>::new();

    for result in rdr.records() {
        let record = result?;

        if record.len() >= 3 {
            if let (Some(x_str), Some(y_str), Some(z_str)) =
                (record.get(0), record.get(2), record.get(1))
            {
                if let (Ok(x_val), Ok(y_val), Ok(z_val)) =
                    (x_str.parse::<f64>(), y_str.parse::<f64>(), z_str.parse::<f64>())
                {
                    x_data.push(x_val);
                    y_data.push(y_val);
                    z_data.push(z_val);
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

    let scatter_plot = ScatterPlot::new(x_data, y_data, z_data);
    scatter_plot.generate_plot("scatter3d5_matrix2.gif")?;

    Ok(())
}
