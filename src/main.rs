use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

#[derive(Debug)]
struct CleanRecord {
    age: u32,
    sex: u32,
    income: u32,
    reported: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let records: Vec<CleanRecord> = read_records("data/final_project_data.csv")?;
    println!("✅ Total clean rows: {}", records.len());
    println!();

    println!("📊 Reporting rate by sex:");
    report_rate_by_sex(&records);
    println!();

    println!("💵 Average income:");
    average_income(&records);
    println!();

    Ok(())
}

fn read_records(filename: &str) -> Result<Vec<CleanRecord>, Box<dyn Error>> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut records: Vec<CleanRecord> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line: String = line?;
        if i == 0 {
            continue; // skip header
        }

        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 4 {
            continue;
        }

        let record = CleanRecord {
            age: fields[0].parse().unwrap_or(0),
            sex: fields[1].parse().unwrap_or(0),
            income: fields[2].parse().unwrap_or(0),
            reported: fields[3].parse().unwrap_or(0),
        };
        records.push(record);
    }

    Ok(records)
}

fn report_rate_by_sex(records: &[CleanRecord]) {
    let mut male_total = 0;
    let mut male_reported = 0;
    let mut female_total = 0;
    let mut female_reported = 0;

    for r in records {
        if r.sex == 1 {
            male_total += 1;
            if r.reported == 1 {
                male_reported += 1;
            }
        } else if r.sex == 2 {
            female_total += 1;
            if r.reported == 1 {
                female_reported += 1;
            }
        }
    }

    if male_total > 0 {
        println!("♂️ Male reporting rate: {:.2}%", male_reported as f64 / male_total as f64 * 100.0);
    }
    if female_total > 0 {
        println!("♀️ Female reporting rate: {:.2}%", female_reported as f64 / female_total as f64 * 100.0);
    }
}

fn average_income(records: &[CleanRecord]) {
    let total_income: u32 = records.iter().map(|r| r.income).sum();
    let count: u32 = records.len() as u32;

    if count > 0 {
        println!("💵 Average income: {:.2}", total_income as f64 / count as f64);
    }
}
