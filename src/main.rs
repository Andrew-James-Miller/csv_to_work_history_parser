//! Work History CSV Parser and Formatter
//! 
//! Author  Andrew James Miller
//! Email   andrewmiller.professional+git@gmail.com
//! Website https://www.andrewjamesmiller.org/
//! Version 1.0.0 of 2025-09-12
//! 
//! This software is licensed under the MIT License, which permits use, modification, and distribution,
//! subject to the terms detailed in the LICENSE file.
//! See https://opensource.org/licenses/MIT for the full text of the MIT License.
//! 
//! (c) Copyright 2025 Andrew J. Miller. All rights reserved.
//! 
//! This program converts work history from CSV format into a formatted text output.
//! 
//! # Usage
//! ```bash
//! csv_to_work_history_parser <input.csv> [output.txt]
//! ```
//! If output path is not provided, the file will be created in the current directory
//! with the name "formatted_work_history.txt"
//! 
//! # Input CSV Format
//! The expected CSV format is:
//! ```text
//! Company,Job Title,Start Date,End Date,Address,Supervisor Name,Description,Reason
//! "Company Name",Position,MM/DD/YYYY,MM/DD/YYYY,"Address",Supervisor,"Description",Reason
//! ```
//! 
//! # Output Format
//! The program generates a text file with entries formatted as:
//! ```text
//! Work History N
//! Company: Company Name
//! Position: Job Title
//! Start Date: MM/YYYY
//! End Date: MM/YYYY
//! Location: City, State
//! Responsibilities: Description
//! ```
//! 
//! # Error Handling
//! The program will provide descriptive errors for:
//! - Invalid file paths
//! - Malformed CSV data
//! - Invalid date formats
//! 
//! # Example
//! ```bash
//! # With specific output path
//! csv_to_work_history_parser work_history.csv my_output.txt
//! 
//! # Output to current directory
//! csv_to_work_history_parser work_history.csv
//! ```

use anyhow::{Context, Result, anyhow};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::PathBuf;

/// Represents a single work history entry with parsed and formatted fields.
/// 
/// This struct contains the essential information extracted from a CSV record,
/// with dates parsed into `NaiveDate` for proper chronological sorting and
/// formatting.
#[derive(Debug)]
struct WorkHistory {
    /// Name of the employer/company
    company: String,
    /// Job title/position held
    position: String,
    /// Employment start date
    start_date: NaiveDate,
    /// Employment end date
    end_date: NaiveDate,
    /// Formatted location (City, State)
    location: String,
    /// Description of job responsibilities
    responsibilities: String,
}

/// Parses command line arguments and returns input and output file paths.
/// 
/// # Returns
/// * `Result<(PathBuf, PathBuf)>` - Tuple of (input_path, output_path)
/// 
/// # Errors
/// Returns an error if arguments are missing or invalid
fn parse_args() -> Result<(PathBuf, PathBuf)> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args.len() > 3 {
        return Err(anyhow!(
            "Usage: {} <input_csv_file> [output_txt_file]\n\
            Example: {} work_history.csv my_output.txt\n\
            If output file is not specified, 'formatted_work_history.txt' will be created in the current directory",
            args.get(0).unwrap_or(&String::from("program")),
            args.get(0).unwrap_or(&String::from("program"))
        ));
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = if args.len() == 3 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from("formatted_work_history.txt")
    };

    Ok((input_path, output_path))
}

/// Validates that the input file exists and output path is valid.
/// 
/// # Arguments
/// * `input_path` - Path to the input CSV file
/// * `output_path` - Path where the output file will be written
/// 
/// # Returns
/// * `Result<()>` - Ok if validation passes, Error otherwise
fn validate_paths(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    // Check input file exists
    if !input_path.exists() {
        return Err(anyhow!("Input file not found: {}", input_path.display()));
    }

    // If output path has a parent directory, check it exists
    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(anyhow!("Output directory not found: {}", parent.display()));
        }
    }

    Ok(())
}

/// Parses a date string in MM/DD/YYYY format into a NaiveDate.
/// 
/// # Arguments
/// * `date` - A string slice containing the date in MM/DD/YYYY format
/// 
/// # Returns
/// * `Result<NaiveDate>` - The parsed date or an error with context
fn parse_date(date: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date, "%m/%d/%Y")
        .with_context(|| format!("Failed to parse date: {}", date))
}

/// Extracts city and state from an address string.
/// 
/// # Arguments
/// * `address` - A string slice containing the full address
/// 
/// # Returns
/// * `String` - Formatted "City, State" or the original string if parsing fails
fn extract_location(address: &str) -> String {
    // If the address already looks like "City, State", return it as is
    if address.matches(',').count() == 1 {
        return address.to_string();
    }

    // Extract city and state from longer addresses
    let parts: Vec<&str> = address.split(',').collect();
    match parts.len() {
        0 => String::new(),
        1 => parts[0].trim().to_string(),
        2 => format!("{}", parts[1].trim()),
        _ => {
            let state_part = parts.get(2)
                .and_then(|s| s.trim().split_whitespace().next())
                .unwrap_or("");
            format!("{}, {}", parts[1].trim(), state_part)
        }
    }
}

/// Formats a NaiveDate into the MM/YYYY format.
/// 
/// # Arguments
/// * `date` - A NaiveDate to format
/// 
/// # Returns
/// * `String` - The date formatted as MM/YYYY
fn format_date(date: NaiveDate) -> String {
    date.format("%m/%Y").to_string()
}

/// Process the CSV file and write formatted output.
/// 
/// # Arguments
/// * `input_path` - Path to the input CSV file
/// * `output_path` - Path where the output file will be written
/// 
/// # Returns
/// * `Result<()>` - Ok if processing succeeds, Error otherwise
fn process_work_history(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    // Open and configure CSV reader
    let file = File::open(input_path)
        .with_context(|| format!("Failed to open input file: {}", input_path.display()))?;
    
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(file);

    let mut work_histories = Vec::new();

    // Parse CSV records
    for result in rdr.records() {
        let record = result.context("Failed to read CSV record")?;
        
        let work_history = WorkHistory {
            company: record[0].to_string(),
            position: record[1].to_string(),
            start_date: parse_date(&record[2])?,
            end_date: parse_date(&record[3])?,
            location: extract_location(&record[4]),
            responsibilities: record[6].to_string(),
        };
        
        work_histories.push(work_history);
    }

    // Sort work histories by end date (most recent first)
    work_histories.sort_by(|a, b| b.end_date.cmp(&a.end_date));

    // Create output file
    let mut output = File::create(output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

    // Write formatted work histories
    for (index, history) in work_histories.iter().enumerate() {
        writeln!(output, "Work History {}", index + 1)?;
        writeln!(output, "Company: {}", history.company)?;
        writeln!(output, "Position: {}", history.position)?;
        writeln!(output, "Start Date: {}", format_date(history.start_date))?;
        writeln!(output, "End Date: {}", format_date(history.end_date))?;
        writeln!(output, "Location: {}", history.location)?;
        writeln!(output, "Responsibilities: {}", history.responsibilities)?;
        writeln!(output)?; // Empty line between entries
    }

    Ok(())
}

fn main() -> Result<()> {
    // Parse command line arguments
    let (input_path, output_path) = parse_args()?;

    // Validate input/output paths
    validate_paths(&input_path, &output_path)?;

    // Process the work history
    process_work_history(&input_path, &output_path)?;

    println!("Successfully created {}", output_path.display());
    Ok(())
}