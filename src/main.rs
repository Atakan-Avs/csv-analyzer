use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;

const USAGE: &str = "Usage: csv-analyzer <file_path> [day]";

#[derive(Deserialize)]
struct TipRecord {
    total_bill: f64,
    tip: f64,
    sex: String,
    smoker: String,
    day: String,
    time: String,
    size: u8,
}

struct CsvData {
    headers: csv::StringRecord,
    rows: BTreeMap<String, csv::StringRecord>,
}

fn read_rows_by_id(path: &str) -> Result<CsvData, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();
    let id_column = headers
        .iter()
        .position(|header| header == "id")
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Missing 'id' column in '{path}'"),
            )
        })?;

    let mut rows = BTreeMap::new();

    for result in reader.records() {
        let row = result?;
        let Some(id) = row.get(id_column).filter(|id| !id.trim().is_empty()) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Empty id in '{path}'"),
            )
            .into());
        };

        if rows.contains_key(id) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Duplicate id '{id}' in '{path}'"),
            )
            .into());
        }

        rows.insert(id.to_owned(), row);
    }

    Ok(CsvData { headers, rows })
}

fn main() -> Result<(), csv::Error> {
    let mut args = std::env::args().skip(1);

    let Some(file_path) = args.next() else {
        eprintln!("{USAGE}");
        std::process::exit(1);
    };

    if file_path == "--help" {
        println!("{USAGE}");
        println!("Example: csv-analyzer data/tips.csv Sun");
        return Ok(());
    }

    if file_path == "diff" {
        let (Some(before_path), Some(after_path), None) = (args.next(), args.next(), args.next())
        else {
            eprintln!("Usage: csv-analyzer diff <before.csv> <after.csv>");
            std::process::exit(1);
        };

        let before = read_rows_by_id(&before_path)?;
        let after = read_rows_by_id(&after_path)?;

        if before.headers != after.headers {
            eprintln!("CSV files must have the same columns in the same order.");
            std::process::exit(1);
        }

        for id in after.rows.keys() {
            if !before.rows.contains_key(id) {
                println!("Added: id={id}");
            }
        }

        for id in before.rows.keys() {
            if !after.rows.contains_key(id) {
                println!("Removed: id={id}");
            }
        }

        for (id, before_row) in &before.rows {
            let Some(after_row) = after.rows.get(id) else {
                continue;
            };

            for (column, (old, new)) in before
                .headers
                .iter()
                .zip(before_row.iter().zip(after_row.iter()))
            {
                if old != new {
                    println!("Changed: id={id}, {column}: {old} -> {new}");
                }
            }
        }

        return Ok(());
    }

    let day_filter = args.next();

    if args.next().is_some() {
        eprintln!("Too many arguments.");
        eprintln!("{USAGE}");
        std::process::exit(1);
    }

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open CSV file '{file_path}': {error}");
            std::process::exit(1);
        }
    };
    let mut reader = csv::Reader::from_reader(file);

    let headers = reader.headers()?;
    println!("Headers: {headers:?}");

    let mut record_count: u32 = 0;
    let mut total_bill = 0.0;
    let mut total_tip = 0.0;
    let mut max_tip = 0.0;

    for result in reader.deserialize::<TipRecord>() {
        let record = match result {
            Ok(record) => record,
            Err(error) => {
                eprintln!("Failed to read a CSV record from '{file_path}': {error}");
                std::process::exit(1);
            }
        };
        if let Some(day) = &day_filter
            && record.day != *day
        {
            continue;
        }
        record_count += 1;
        total_bill += record.total_bill;
        total_tip += record.tip;
        if record_count == 1 || record.tip > max_tip {
            max_tip = record.tip;
        }

        if record_count == 1 {
            println!(
                "First record: total_bill={}, tip={}, sex={}, smoker={}, day={}, time={}, size={}",
                record.total_bill,
                record.tip,
                record.sex,
                record.smoker,
                record.day,
                record.time,
                record.size,
            );
        }
    }

    println!("Parsed {record_count} records from {file_path}");

    if record_count == 0 {
        println!("No records to analyze.");
        return Ok(());
    }

    println!("Total bill: {total_bill:.2}");
    println!("Total tips: {total_tip:.2}");

    let average_tip = total_tip / f64::from(record_count);
    println!("Average tip: {average_tip:.2}");
    println!("Highest tip: {max_tip:.2}");

    Ok(())
}
