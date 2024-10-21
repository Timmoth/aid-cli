extern crate csv;
extern crate nom;
extern crate serde;

use std::error::Error;
use std::fs::File;

use csv::{ReaderBuilder, StringRecord};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::{alphanumeric1, char, digit1, one_of, space0, space1},
    combinator::{map, opt, value},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use regex::Regex;

#[derive(Debug)]
pub struct SQLQuery {
    pub columns: Vec<String>,
    pub table: String,
    pub condition: Option<Condition>,
    pub order_by: Option<OrderBy>,
    pub distinct: bool,
    pub group_by: Option<Vec<String>>,
    pub aggregate_functions: Vec<AggregateFunction>,
}

#[derive(Debug, PartialEq)]
pub enum AggregateFunction {
    Count(String),
    Min(String),
    Max(String),
    Sum(String),
    Avg(String),
}

#[derive(Debug, PartialEq)]
pub struct OrderBy {
    pub column: String,
    pub direction: SortDirection,
}

#[derive(Debug, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub enum Condition {
    GreaterThan(String, i32),
    GreaterThanEqualTo(String, i32),
    LessThan(String, i32),
    LessThanEqualTo(String, i32),
    Equal(String, String),
    Like(String, regex::Regex),
    Between(String, i32, i32),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

pub fn load_csv(file_path: &str) -> Result<(Vec<String>, Vec<StringRecord>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);

    // Read headers from the CSV
    let headers = reader
        .headers()?
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<String>>();

    // Read all records into a vector
    let records: Vec<StringRecord> = reader.records().collect::<Result<Vec<_>, _>>()?;

    Ok((headers, records))
}

pub fn apply_query(
    headers: &Vec<String>,
    records: Vec<StringRecord>,
    query: &SQLQuery,
) -> (Vec<String>, Vec<Vec<String>>) {
    // First, filter the records based on the WHERE clause
    let filtered_records: Vec<StringRecord> = records
        .into_iter()
        .filter(|row| {
            if let Some(condition) = &query.condition {
                evaluate_condition(condition, row, headers)
            } else {
                true // No conditions means all rows pass
            }
        })
        .collect();

    // Create a new header order for the result
    let mut result_headers: Vec<String> = Vec::new();

    // Add headers
    for h in &query.columns {
        if h == &"*" {
            result_headers.extend(headers.clone());
        } else {
            result_headers.push(h.to_string());
        }
    }

    // Group records if GROUP BY is specified
    let grouped_records: Vec<(String, Vec<&StringRecord>)> = if let Some(group_by) = &query.group_by
    {
        let mut groups: std::collections::HashMap<String, Vec<&StringRecord>> =
            std::collections::HashMap::new();

        for record in filtered_records.iter() {
            let group_key = group_by
                .iter()
                .map(|col| {
                    let index = headers.iter().position(|h| h == col).unwrap();
                    record.get(index).unwrap_or("").to_string()
                })
                .collect::<Vec<String>>()
                .join(",");

            groups
                .entry(group_key)
                .or_insert_with(Vec::new)
                .push(record);
        }

        groups.into_iter().collect() // Convert the HashMap to a Vec
    } else {
        // No grouping, create a single group with all filtered records
        vec![("*".to_string(), filtered_records.iter().collect())] // Use a default key for no grouping
    };

    // Prepare the result rows
    let mut result_rows = Vec::new();

    // If there's an aggregate function, process it
    if !query.aggregate_functions.is_empty() {
        for (group_key, records) in &grouped_records {
            let mut row = Vec::new();

            for column_name in &query.columns {
                let column_value =
                    if let Some(index) = headers.iter().position(|h| h == column_name) {
                        // Collect the values for the column across all records in the group
                        records
                            .iter()
                            .map(|record| record.get(index).unwrap_or("").to_string())
                            .next() // Take the first value (or aggregate if needed later)
                    } else {
                        None
                    };
                row.push(column_value.unwrap_or_default());
            }

            // Now, calculate the aggregate functions for the group
            for aggregate_function in &query.aggregate_functions {
                match aggregate_function {
                    // COUNT function
                    AggregateFunction::Count(column_name) => {
                        if let Some(column_index) = headers.iter().position(|h| h == column_name) {
                            let count = records
                                .iter()
                                .filter(|record| {
                                    record
                                        .get(column_index)
                                        .map_or(false, |value| !value.is_empty())
                                })
                                .count();
                            row.push(count.to_string()); // Add the count result to the row
                        } else if column_name == "*" {
                            row.push(records.len().to_string()); // Special case: count all records in the group
                        } else {
                            println!("Column '{}' not found in headers", column_name);
                            row.push("0".to_string()); // Default to 0 if column not found
                        }
                    }

                    // MIN function
                    AggregateFunction::Min(column_name) => {
                        if let Some(column_index) = headers.iter().position(|h| h == column_name) {
                            if let Some(min_value) = records
                                .iter()
                                .filter_map(|record| record.get(column_index))
                                .filter(|value| !value.is_empty())
                                .min()
                            {
                                row.push(min_value.to_string());
                            } else {
                                row.push("NULL".to_string()); // Handle empty or missing values
                            }
                        } else {
                            println!("Column '{}' not found in headers", column_name);
                            row.push("NULL".to_string());
                        }
                    }

                    // MAX function
                    AggregateFunction::Max(column_name) => {
                        if let Some(column_index) = headers.iter().position(|h| h == column_name) {
                            if let Some(max_value) = records
                                .iter()
                                .filter_map(|record| record.get(column_index))
                                .filter(|value| !value.is_empty())
                                .max()
                            {
                                row.push(max_value.to_string());
                            } else {
                                row.push("NULL".to_string());
                            }
                        } else {
                            println!("Column '{}' not found in headers", column_name);
                            row.push("NULL".to_string());
                        }
                    }

                    // SUM function
                    AggregateFunction::Sum(column_name) => {
                        if let Some(column_index) = headers.iter().position(|h| h == column_name) {
                            let sum: f64 = records
                                .iter()
                                .filter_map(|record| record.get(column_index))
                                .filter(|value| !value.is_empty())
                                .filter_map(|value| value.parse::<f64>().ok())
                                .sum();
                            row.push(sum.to_string()); // Add the sum result to the row
                        } else {
                            println!("Column '{}' not found in headers", column_name);
                            row.push("0".to_string());
                        }
                    }

                    // AVG function
                    AggregateFunction::Avg(column_name) => {
                        if let Some(column_index) = headers.iter().position(|h| h == column_name) {
                            let sum: f64 = records
                                .iter()
                                .filter_map(|record| record.get(column_index))
                                .filter(|value| !value.is_empty())
                                .filter_map(|value| value.parse::<f64>().ok())
                                .sum();

                            let count: usize = records
                                .iter()
                                .filter(|record| {
                                    record
                                        .get(column_index)
                                        .map_or(false, |value| !value.is_empty())
                                })
                                .count();

                            if count > 0 {
                                let avg = sum / (count as f64);
                                row.push(avg.to_string());
                            } else {
                                row.push("NULL".to_string()); // Handle division by zero case
                            }
                        } else {
                            println!("Column '{}' not found in headers", column_name);
                            row.push("NULL".to_string());
                        }
                    }
                }
            }
            // After processing both columns and aggregates, push the row
            result_rows.push(row);
        }

        // Update result_headers to include the group key and aggregate function columns
        for aggregate_function in &query.aggregate_functions {
            match aggregate_function {
                AggregateFunction::Count(column_name) => {
                    result_headers.push(format!("COUNT({})", column_name));
                }
                AggregateFunction::Min(column_name) => {
                    result_headers.push(format!("MIN({})", column_name));
                }
                AggregateFunction::Max(column_name) => {
                    result_headers.push(format!("MAX({})", column_name));
                }
                AggregateFunction::Sum(column_name) => {
                    result_headers.push(format!("SUM({})", column_name));
                }
                AggregateFunction::Avg(column_name) => {
                    result_headers.push(format!("AVG({})", column_name));
                }
            }
        }
    } else {
        if query.distinct {
            let mut distinct_records = std::collections::HashSet::new();
            for record in filtered_records {
                // Create a unique key for the record based on the selected columns
                let key = if query.columns.contains(&"*".to_string()) {
                    // Create a unique key using all fields in the record
                    record.iter().collect::<Vec<&str>>().join(",")
                } else {
                    query
                        .columns
                        .iter()
                        .filter_map(|header| {
                            headers
                                .iter()
                                .position(|h| h == header)
                                .and_then(|index| record.get(index))
                        })
                        .map(|value| value.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                };

                // Insert the key into the HashSet to ensure uniqueness
                distinct_records.insert(key);
            }

            // Collect distinct results
            for key in distinct_records {
                // Create a row based on the unique key
                let row: Vec<String> = key.split(',').map(|s| s.to_string()).collect();
                result_rows.push(row);
            }
        } else {
            // Process records based on specified columns
            for record in filtered_records {
                let row: Vec<String> = if query.columns.contains(&"*".to_string()) {
                    record.iter().map(|value| value.to_string()).collect()
                } else {
                    query
                        .columns
                        .iter()
                        .filter_map(|header| {
                            if let Some(index) = headers.iter().position(|h| h == header) {
                                Some(record.get(index).unwrap_or("").to_string())
                            } else {
                                None
                            }
                        })
                        .collect()
                };
                result_rows.push(row);
            }
        }
    }

    // Create a mapping of column names to their respective indices
    let column_index_map: std::collections::HashMap<_, _> = result_headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.clone(), i))
        .collect();

    // Sort the results if ORDER BY is specified after aggregation
    if let Some(order_by) = &query.order_by {
        if let Some(column_index) = column_index_map.get(&order_by.column) {
            result_rows.sort_by(|a, b| {
                let default = String::from("");
                let a_value = a.get(*column_index).unwrap_or(&default);
                let b_value = b.get(*column_index).unwrap_or(&default);

                // Try to parse the values as numbers first
                let a_parsed = a_value.parse::<f64>();
                let b_parsed = b_value.parse::<f64>();

                match (a_parsed, b_parsed) {
                    (Ok(a_num), Ok(b_num)) => {
                        // Both values are numeric
                        match order_by.direction {
                            SortDirection::Ascending => a_num.partial_cmp(&b_num).unwrap(),
                            SortDirection::Descending => b_num.partial_cmp(&a_num).unwrap(),
                        }
                    }
                    _ => {
                        // Fallback to string comparison if either value is not a valid number
                        match order_by.direction {
                            SortDirection::Ascending => a_value.cmp(b_value),
                            SortDirection::Descending => b_value.cmp(a_value),
                        }
                    }
                }
            });
        }
    }

    (result_headers, result_rows)
}

fn evaluate_condition(condition: &Condition, row: &StringRecord, headers: &[String]) -> bool {
    match condition {
        Condition::GreaterThan(col, threshold) => {
            if let Some(index) = headers.iter().position(|h| h == col) {
                if let Some(value) = row.get(index) {
                    if let Ok(num) = value.parse::<i32>() {
                        return num > *threshold;
                    }
                }
            }
            false
        }
        Condition::GreaterThanEqualTo(col, threshold) => {
            if let Some(index) = headers.iter().position(|h| h == col) {
                if let Some(value) = row.get(index) {
                    if let Ok(num) = value.parse::<i32>() {
                        return num >= *threshold;
                    }
                }
            }
            false
        }
        Condition::LessThan(col, threshold) => {
            if let Some(index) = headers.iter().position(|h| h == col) {
                if let Some(value) = row.get(index) {
                    if let Ok(num) = value.parse::<i32>() {
                        return num < *threshold;
                    }
                }
            }
            false
        }
        Condition::LessThanEqualTo(col, threshold) => {
            if let Some(index) = headers.iter().position(|h| h == col) {
                if let Some(value) = row.get(index) {
                    if let Ok(num) = value.parse::<i32>() {
                        return num <= *threshold;
                    }
                }
            }
            false
        }
        Condition::Equal(col, expected_value) => {
            if let Some(index) = headers.iter().position(|h| h == col) {
                if let Some(value) = row.get(index) {
                    return value == expected_value;
                }
            }
            false
        }
        Condition::Between(col, lower, upper) => {
            if let Some(index) = headers.iter().position(|h| h == col) {
                if let Some(value) = row.get(index) {
                    if let Ok(num) = value.parse::<i32>() {
                        return num >= *lower && num <= *upper; // Check if the value is within the range
                    }
                }
            }
            false
        }
        Condition::And(cond1, cond2) => {
            evaluate_condition(cond1, row, headers) && evaluate_condition(cond2, row, headers)
        }
        Condition::Or(cond1, cond2) => {
            evaluate_condition(cond1, row, headers) || evaluate_condition(cond2, row, headers)
        }
        Condition::Not(cond1) => !evaluate_condition(cond1, row, headers),
        Condition::Like(col, pattern) => {
            if let Some(index) = headers.iter().position(|h| h == col) {
                if let Some(value) = row.get(index) {
                    return pattern.is_match(value);
                }
            }
            false
        }
    }
}

// Function to parse comma followed by optional spaces
fn comma_space(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?; // Consume any leading whitespace
    let (input, _) = tag(",")(input)?; // Consume the comma
    let (input, _) = space0(input)?; // Consume any trailing whitespace
    Ok((input, ())) // Return remaining input and unit value
}

fn parse_group_by(input: &str) -> IResult<&str, Vec<String>> {
    preceded(
        tuple((space0, tag("GROUP BY"), space1)),
        separated_list1(comma_space, parse_column), // Parse column names
    )(input)
}

// Function to parse aggregate functions
fn parse_aggregate_function(input: &str) -> IResult<&str, AggregateFunction> {
    alt((
        // Parser for COUNT function
        preceded(
            tuple((space0, tag("COUNT"))),
            map(delimited(char('('), parse_column, char(')')), |col| {
                AggregateFunction::Count(col)
            }),
        ),
        preceded(
            tuple((space0, tag("COUNT"))),
            map(delimited(char('('), char('*'), char(')')), |col| {
                AggregateFunction::Count(String::from("(*)"))
            }),
        ),
        // Parser for MIN function
        preceded(
            tuple((space0, tag("MIN"))),
            map(delimited(char('('), parse_column, char(')')), |col| {
                AggregateFunction::Min(col)
            }),
        ),
        preceded(
            tuple((space0, tag("Min"))),
            map(delimited(char('('), char('*'), char(')')), |col| {
                AggregateFunction::Min(String::from("(*)"))
            }),
        ),
        // Parser for MAX function
        preceded(
            tuple((space0, tag("MAX"))),
            map(delimited(char('('), parse_column, char(')')), |col| {
                AggregateFunction::Max(col)
            }),
        ),
        preceded(
            tuple((space0, tag("Max"))),
            map(delimited(char('('), char('*'), char(')')), |col| {
                AggregateFunction::Max(String::from("(*)"))
            }),
        ),
        // Parser for SUM function
        preceded(
            tuple((space0, tag("SUM"))),
            map(delimited(char('('), parse_column, char(')')), |col| {
                AggregateFunction::Sum(col)
            }),
        ),
        preceded(
            tuple((space0, tag("Sum"))),
            map(delimited(char('('), char('*'), char(')')), |col| {
                AggregateFunction::Sum(String::from("(*)"))
            }),
        ),
        // Parser for AVG function
        preceded(
            tuple((space0, tag("AVG"))),
            map(delimited(char('('), parse_column, char(')')), |col| {
                AggregateFunction::Avg(col)
            }),
        ),
        preceded(
            tuple((space0, tag("Avg"))),
            map(delimited(char('('), char('*'), char(')')), |col| {
                AggregateFunction::Avg(String::from("(*)"))
            }),
        ),
    ))(input)
}

fn parse_wildcard(input: &str) -> IResult<&str, String> {
    map(tag("*"), |_| "*".to_string())(input)
}

// Parser for quoted strings (including spaces)
fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(char('\''), is_not("\'"), char('\''))(input)
        .map(|(next_input, result)| (next_input, result.to_string()))
}

// Parser for unquoted identifiers (alphanumeric)
fn parse_identifier(input: &str) -> IResult<&str, String> {
    // Allow identifiers to consist of letters, digits, and underscores
    map(
        many1(one_of(
            "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        )),
        |s: Vec<char>| s.into_iter().collect(), // Collect characters into a String
    )(input)
}

fn parse_aggregate_function_name(input: &str) -> IResult<&str, String> {
    let (input, func_name) =
        alt((tag_no_case("COUNT"), tag_no_case("SUM"), tag_no_case("AVG")))(input)?;
    let (input, _) = tuple((char('('), space0))(input)?;
    let (input, column) = parse_column(input)?;
    let (input, _) = tuple((space0, char(')')))(input)?;
    Ok((input, format!("{}({})", func_name.to_uppercase(), column)))
}

// Combined parser for column names (either quoted strings or identifiers)
fn parse_column(input: &str) -> IResult<&str, String> {
    alt((parse_quoted_string, parse_identifier, parse_wildcard))(input)
}

fn parse_order_by_column(input: &str) -> IResult<&str, String> {
    alt((
        parse_quoted_string,
        parse_aggregate_function_name,
        parse_identifier,
        parse_wildcard,
    ))(input)
}

// Parser for the SELECT columns
fn parse_columns(input: &str) -> IResult<&str, (bool, Vec<AggregateFunction>, Vec<String>)> {
    // First, parse the SELECT keyword
    let (input, _) = preceded(space0, tuple((tag("SELECT"), space1)))(input)?;

    // Check for DISTINCT (optional)
    let (input, distinct) = opt(tuple((tag("DISTINCT"), space1)))(input)?;
    let distinct_flag = distinct.is_some();

    // Initialize the vectors to hold aggregate functions and columns
    let mut aggregate_functions = Vec::new();
    let mut columns = Vec::new();

    // Function to parse the first item, either an aggregate function or a column
    let (input, first_item) = alt((
        map(parse_aggregate_function, |agg| {
            aggregate_functions.push(agg);
            input // Return the remaining input after parsing
        }),
        map(parse_column, |col| {
            columns.push(col);
            input // Return the remaining input after parsing
        }),
    ))(input)?;

    // Parse any additional aggregate functions or columns, separated by commas
    let (input, additional_items) = opt(preceded(
        comma_space,
        separated_list1(
            comma_space,
            alt((
                map(parse_aggregate_function, |agg| {
                    aggregate_functions.push(agg);
                    input // Return the remaining input after parsing
                }),
                map(parse_column, |col| {
                    columns.push(col);
                    input // Return the remaining input after parsing
                }),
            )),
        ),
    ))(input)?;

    // Process additional items if they were found
    if let Some(items) = additional_items {
        for item in items {
            // Since we pushed the items during parsing, we don't need to do anything else
        }
    }

    Ok((input, (distinct_flag, aggregate_functions, columns)))
}

// Function to parse a file path
fn parse_file(input: &str) -> IResult<&str, String> {
    // Define a valid character for file names (excluding newline)
    let valid_filename_char = is_not("/\\ \n");

    // Define a parser for a directory or filename
    let component = map(valid_filename_char, |s: &str| s.to_string());

    // Define a parser for a single directory component, allowing for optional leading "./" or "../"
    let directory_component = preceded(
        many0(alt((
            map(char('.'), |_| ""),                         // Handle '.'
            map(char('/'), |_| "/"),                        // Handle '/'
            map(preceded(char('.'), char('.')), |_| "../"), // Handle '..'
        ))),
        component,
    );

    // Combine the components into a full path
    let mut path_parser = map(many0(directory_component), |components: Vec<String>| {
        components.join("/") // Join components with '/'
    });

    path_parser(input)
}

fn parse_table(input: &str) -> IResult<&str, String> {
    preceded(tuple((space0, tag("FROM"), space1)), parse_file)(input)
}

// Combine both parsers to accept either an identifier or a quoted string
fn parse_value(input: &str) -> IResult<&str, String> {
    alt((parse_quoted_string, parse_identifier))(input)
}

// Update parse_single_condition to use parse_value
fn parse_single_condition(input: &str) -> IResult<&str, Condition> {
    alt((
        map(
            tuple((parse_column, space0, tag(">="), space0, digit1)),
            |(col, _, _, _, value): (String, &str, &str, &str, &str)| {
                Condition::GreaterThanEqualTo(col, value.parse::<i32>().unwrap())
            },
        ),
        map(
            tuple((parse_column, space0, tag(">"), space0, digit1)),
            |(col, _, _, _, value): (String, &str, &str, &str, &str)| {
                Condition::GreaterThan(col, value.parse::<i32>().unwrap())
            },
        ),
        map(
            tuple((parse_column, space0, tag("<="), space0, digit1)),
            |(col, _, _, _, value): (String, &str, &str, &str, &str)| {
                Condition::LessThanEqualTo(col, value.parse::<i32>().unwrap())
            },
        ),
        map(
            tuple((parse_column, space0, tag("<"), space0, digit1)),
            |(col, _, _, _, value): (String, &str, &str, &str, &str)| {
                Condition::LessThan(col, value.parse::<i32>().unwrap())
            },
        ),
        map(
            tuple((parse_column, space0, tag("="), space0, parse_value)),
            |(col, _, _, _, value): (String, &str, &str, &str, String)| {
                Condition::Equal(col, value)
            },
        ),
        map(
            tuple((parse_column, space0, tag("LIKE"), space0, parse_value)),
            |(col, _, _, _, pattern): (String, &str, &str, &str, String)| {
                // Convert the LIKE pattern into a regex
                let regex_pattern = like_pattern_to_regex(&pattern); // Anchor the pattern

                // Try to compile the regex
                match Regex::new(&regex_pattern) {
                    Ok(compiled_regex) => Condition::Like(col, compiled_regex),
                    Err(_) => panic!("Failed to compile regex for LIKE condition '{}'.", pattern), // In production, handle this more gracefully
                }
            },
        ),
        map(
            tuple((
                parse_column,
                space0,
                tag("BETWEEN"),
                space0,
                digit1,
                space0,
                tag("AND"),
                space0,
                digit1,
            )),
            |(col, _, _, _, lower_bound, _, _, _, upper_bound): (
                String,
                &str,
                &str,
                &str,
                &str,
                &str,
                &str,
                &str,
                &str,
            )| {
                Condition::Between(
                    col,
                    lower_bound.parse::<i32>().unwrap(),
                    upper_bound.parse::<i32>().unwrap(),
                )
            },
        ),
    ))(input)
}

fn like_pattern_to_regex(pattern: &str) -> String {
    let regex_pattern = regex::escape(pattern).replace("%", ".*");
    format!("^{}$", regex_pattern) // Anchoring the pattern
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    preceded(
        tuple((space0, tag("WHERE"), space1)),
        parse_or_condition, // Start with OR conditions (lowest precedence)
    )(input)
}

// Parse OR conditions (OR has lower precedence than AND)
fn parse_or_condition(input: &str) -> IResult<&str, Condition> {
    let (input, first_condition) = parse_and_condition(input)?; // Parse first AND condition
    let (input, rest_conditions) = many0(preceded(
        tuple((space0, tag("OR"), space1)),
        parse_and_condition, // Parse more AND conditions separated by OR
    ))(input)?;

    // Combine multiple OR conditions into a single condition
    let combined_condition = rest_conditions
        .into_iter()
        .fold(first_condition, |acc, cond| {
            Condition::Or(Box::new(acc), Box::new(cond))
        });

    Ok((input, combined_condition))
}

// Parse AND conditions (AND has higher precedence than OR)
fn parse_and_condition(input: &str) -> IResult<&str, Condition> {
    let (input, first_condition) = parse_not_condition(input)?; // Parse first NOT condition
    let (input, rest_conditions) = many0(preceded(
        tuple((space0, tag("AND"), space1)),
        parse_not_condition, // Parse more NOT conditions separated by AND
    ))(input)?;

    // Combine multiple AND conditions into a single condition
    let combined_condition = rest_conditions
        .into_iter()
        .fold(first_condition, |acc, cond| {
            Condition::And(Box::new(acc), Box::new(cond))
        });

    Ok((input, combined_condition))
}

fn parse_not_condition(input: &str) -> IResult<&str, Condition> {
    // Check if the input starts with "NOT"
    let (input, negated) = opt(preceded(
        tuple((tag("NOT"), space1)),
        parse_primary_condition,
    ))(input)?;

    // If there's a NOT condition, return it wrapped in a Condition::Not
    if let Some(condition) = negated {
        Ok((input, Condition::Not(Box::new(condition))))
    } else {
        // Otherwise, fall back to parsing a primary condition (no NOT)
        parse_primary_condition(input)
    }
}

// Primary condition parser, handles individual conditions and grouped expressions in parentheses
fn parse_primary_condition(input: &str) -> IResult<&str, Condition> {
    alt((
        // Parse a grouped condition inside parentheses
        delimited(
            tuple((char('('), space0)),
            parse_or_condition, // Recursively parse OR conditions inside parentheses
            tuple((space0, char(')'))),
        ),
        // Parse a single condition (e.g., age > 30)
        parse_single_condition, // Your existing single condition parser
    ))(input)
}
fn parse_order_by(input: &str) -> IResult<&str, OrderBy> {
    preceded(
        tuple((space0, tag("ORDER BY"), space1)),
        // Parse the column name and the direction (ASC or DESC)
        map(
            tuple((
                parse_order_by_column,
                // Optional space followed by sort direction
                opt(preceded(
                    space0,
                    alt((
                        map(tag("ASC"), |_| SortDirection::Ascending),
                        map(tag("DESC"), |_| SortDirection::Descending),
                    )),
                )),
            )),
            |(column, direction)| OrderBy {
                column,
                direction: direction.unwrap_or(SortDirection::Ascending), // Default to ascending if no direction specified
            },
        ),
    )(input)
}

pub fn parse_sql(input: &str) -> IResult<&str, SQLQuery> {
    // Call parse_columns to get the distinct flag, aggregate function, and columns
    let (input, (distinct, aggregate_functions, columns)) = parse_columns(input)?;

    // Parse the rest of the SQL query
    let (input, table) = parse_table(input)?;
    let (input, condition) = opt(parse_condition)(input)?;
    let (input, group_by) = opt(parse_group_by)(input)?;
    let (input, order_by) = opt(parse_order_by)(input)?;

    Ok((
        input,
        SQLQuery {
            columns,
            table,
            condition,
            order_by,
            group_by,
            aggregate_functions,
            distinct,
        },
    ))
}
