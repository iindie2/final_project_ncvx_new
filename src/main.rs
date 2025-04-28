use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

// Define Record struct (matches your dataset format)
#[derive(Debug)]
struct Record {
    age: i32,
    sex: i32, // 1 for male, 2 for female
    income: f32,
    reported: i32, // 1 for reported, 2 for not reported
}

// Function to load and clean the data
fn load_and_clean_data(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut records = Vec::new();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();  // Adjusted to CSV

        if fields.len() < 4 { continue; }

        let record = Record {
            age: fields[0].parse().unwrap_or_default(),
            sex: fields[1].parse().unwrap_or_default(),
            income: fields[2].parse().unwrap_or_default(),
            reported: fields[3].parse().unwrap_or_default(),
        };

        records.push(record);
    }

    // Clean the data
    Ok(records.into_iter().filter(|r| r.income > 0.0 && r.age > 0).collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/final_project_data.csv"; // Ensure this file path is correct
    let clean_data = load_and_clean_data(file_path)?;

    // Step 1: Print total clean rows
    if clean_data.is_empty() {
        println!("No valid data found.");
        return Ok(());
    }

    println!("✅ Total clean rows: {}", clean_data.len());

    // Step 2: Calculate and print Average Income
    let total_income: f32 = clean_data.iter().map(|record| record.income).sum();
    let count = clean_data.len();
    let average_income = total_income / count as f32;
    println!("💵 Average income: {:.2}", average_income);

    // Step 3: Calculate and print Min and Max Income
    let min_income = clean_data.iter().map(|record| record.income).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
    let max_income = clean_data.iter().map(|record| record.income).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);

    println!("💵 Min income: {}", min_income);
    println!("💵 Max income: {}", max_income);

    // Step 4: Calculate and print Reporting Rate by Sex
    let male_reports = clean_data.iter().filter(|&record| record.sex == 1 && record.reported == 2).count();
    let female_reports = clean_data.iter().filter(|&record| record.sex == 2 && record.reported == 2).count();
    let male_report_rate = male_reports as f32 / clean_data.len() as f32 * 100.0;
    let female_report_rate = female_reports as f32 / clean_data.len() as f32 * 100.0;

    println!("📊 Reporting rate by sex:");
    println!("♂️ Male reporting rate: {:.2}%", male_report_rate);
    println!("♀️ Female reporting rate: {:.2}%", female_report_rate);

    Ok(())
}
