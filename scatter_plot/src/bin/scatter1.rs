use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

struct ScatterPlot {
    x_data: Vec<f64>,
    y_data: Vec<f64>,
    gender_data: Vec<String>, 
}

impl ScatterPlot {
    fn new(x_data: Vec<f64>, y_data: Vec<f64>, gender_data: Vec<String>) -> Self {
        ScatterPlot {
            x_data,
            y_data,
            gender_data,
        }
    }

    fn generate_plot(&self, output_path: &str) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(output_path, (1000, 600)).into_drawing_area();
        root.fill(&WHITE)?;
    
        let mut chart = ChartBuilder::on(&root)
            .caption("salary and years of experience", ("sans-serif", 40))
            .x_label_area_size(50)
            .y_label_area_size(70)
            .margin(20)
            .build_cartesian_2d(0.0..15.0, 0.0..200000.0)?;
    
        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(15)
            .x_desc("experiences in years")
            .y_desc("salary in THB")
            .axis_desc_style(TextStyle::from(("sans-serif", 20)))
            
            .draw()?;
    
        chart.draw_series(
            self.x_data
                .iter()
                .zip(self.y_data.iter())
                .zip(self.gender_data.iter())
                
                .map(|((&x, &y), gender)| {
                    if gender == "male" {
                        Circle::new((x, y), 5, RGBColor(0, 0, 255).filled())
                    } else if gender == "female" {
                        Circle::new((x, y), 5, RGBColor(255, 0, 0).filled())
                    } else {
                        Circle::new((x, y), 5, RGBColor(148,0,211).filled()) // Default color for other genders
                    }
                }),
                
        )?;

// Add legends as colored circles
chart.draw_series((0..=2).map(|i| {
    Circle::new(
        (13.5, (i as f64 * 13000.0) + 11000.0), // Adjust the position
        5,
        if i == 0 {
            RGBColor(0, 0, 255).filled()
        } else if i == 1 {
            RGBColor(255, 0, 0).filled()
        } else {
            RGBColor(128, 0, 128).filled()
        },
    )
}))?;

// Add legend labels
chart.draw_series((0..=2).map(|i| {
    Text::new(
        if i == 0 {
            "Male"
        } else if i == 1 {
            "Female"
        } else {
            "Other"
        },
        (13.8, (i as f64 * 13000.0) + 13000.0), // Adjust the position
        TextStyle::from(("sans-serif", 20)),
    )
}))?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/Salary.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut age_data = Vec::<f64>::new();
    let mut tip_data = Vec::<f64>::new();
    let mut gender_data = Vec::<String>::new(); // New vector for gender data

    for result in rdr.records() {
        let record = result?;

        if record.len() >= 3 {
            if let (Some(age_str), Some(tip_str), Some(gender)) =
                (record.get(0), record.get(1), record.get(2))
            {
                if let (Ok(age_val), Ok(tip_val)) = (age_str.parse::<f64>(), tip_str.parse::<f64>()) {
                    age_data.push(age_val);
                    tip_data.push(tip_val);
                    gender_data.push(gender.to_string());
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

    let scatter_plot = ScatterPlot::new(age_data, tip_data, gender_data);
    scatter_plot.generate_plot("scatter_plot8.png")?;

    Ok(())
}
