use std::{fs::File, time::Instant};

use csv::Writer;

use crate::csv_utils;

pub async fn sql_search(sql: String, output_path: Option<String>) {

    let start = Instant::now();
    
    // Parse the SQL query
    let parsed = match csv_utils::parse_sql(&sql) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to parse SQL query: {}", e);
            return; // Early exit on error
        }
    };
    
    let parse_time = start.elapsed();
    let start = Instant::now();

    let query = parsed.1;

    // Load the CSV file
    let (headers, records) = match csv_utils::load_csv(&query.table) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to load CSV file '{}': {}", query.table, e);
            return; // Early exit on error
        }
    };

    let rows = records.len();
    let load_time = start.elapsed();
    let start = Instant::now();

    // Apply the query
    let results = csv_utils::apply_query(&headers, records, &query);
    let query_time = start.elapsed();
    let result_rows = results.len();

    output_results(results, &query.columns, &output_path);

    if output_path.is_some() {
        // Log the query statistics
        println!("Parsed {:?} in {:?}", query, parse_time);
        println!("Loaded {} rows in {:?}", rows, load_time);
        println!("Found {} rows in {:?}", result_rows, query_time);
    }
}


fn output_results(results: Vec<Vec<String>>, headers: &Vec<String>, output_path: &Option<String>) {
    if results.is_empty() {
        println!("No results found.");
        return;
    }

    match output_path {
        Some(path) => {
            match File::create(&path) {
                Ok(file) => {
                    let mut wtr = Writer::from_writer(file);

                    // Write headers to the CSV
                    if let Err(e) = wtr.write_record(headers) {
                        eprintln!("Failed to write headers to file: {}", e);
                        return;
                    }

                    // Write each row of results to the CSV
                    for row in results.iter() {
                        if let Err(e) = wtr.write_record(row) {

                            println!("{:?}", row);
                            eprintln!("Failed to write row to file: {}", e);
                            return;
                        }
                    }

                    if let Err(e) = wtr.flush() {
                        eprintln!("Failed to flush writer: {}", e);
                    }

                    println!("Results successfully written to {}", path);
                }
                Err(e) => {
                    eprintln!("Failed to create file {}: {}", path, e);
                }
            }
        }

        None => {
            println!("{:?}", headers);
            for row in results.iter() {
                println!("{}", row.join(","));
            }
        }
    }
}
