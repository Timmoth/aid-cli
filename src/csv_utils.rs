extern crate csv;
extern crate serde;
extern crate nom;

use std::error::Error;
use std::fs::File;

use csv::{ReaderBuilder, StringRecord};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, digit1, space0, space1, char},
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct SQLQuery {
    pub columns: Vec<String>,
    pub table: String,
    pub condition: Option<Condition>,
    pub order_by: Option<OrderBy>, // New field for ORDER BY clause
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

#[derive(Debug, PartialEq)]
pub enum Condition {
    GreaterThan(String, i32),
    GreaterThanEqualTo(String, i32),
    LessThan(String, i32),
    LessThanEqualTo(String, i32),
    Equal(String, String),
    And(Box<Condition>, Box<Condition>)
}

pub fn load_csv(file_path: &str) -> Result<(Vec<String>, Vec<StringRecord>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);

    // Read headers from the CSV
    let headers = reader.headers()?.iter().map(|h| h.to_string()).collect::<Vec<String>>();

    // Read all records into a vector
    let records: Vec<StringRecord> = reader.records().collect::<Result<Vec<_>, _>>()?;

    Ok((headers, records))
}

pub fn apply_query(
    headers: &Vec<String>,
    records: Vec<StringRecord>,
    query: &SQLQuery,
) -> Vec<Vec<String>> {
    // Filter the records
    let mut filtered_records: Vec<StringRecord> = records
        .into_iter()
        .filter(|row| {
            if let Some(condition) = &query.condition {
                evaluate_condition(condition, row, &headers)
            } else {
                true // No conditions means all rows pass
            }
        })
        .collect();

    // Sort the filtered results
    if let Some(order_by) = &query.order_by {
        let column_index = headers.iter().position(|h| h == &order_by.column).unwrap();

        filtered_records.sort_by(|a, b| {
            let a_value = a.get(column_index).unwrap_or("");
            let b_value = b.get(column_index).unwrap_or("");

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
                },
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

    // selecting only the requested columns
    filtered_records
        .into_iter()
        .map(|row| {
            query.columns.iter()
                .filter_map(|header| {
                    if let Some(index) = headers.iter().position(|h| h == header) {
                        Some(row.get(index).unwrap_or("").to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect()
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
        Condition::And(cond1, cond2) => {
            evaluate_condition(cond1, row, headers) && evaluate_condition(cond2, row, headers)
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

// Parser for quoted strings (including spaces)
fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(char('\''), is_not("\'"), char('\''))(input)
        .map(|(next_input, result)| (next_input, result.to_string()))
}

// Parser for unquoted identifiers (alphanumeric)
fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(alphanumeric1, |s: &str| s.to_string())(input)
}

// Combined parser for column names (either quoted strings or identifiers)
fn parse_column(input: &str) -> IResult<&str, String> {
    alt((parse_quoted_string, parse_identifier))(input)
}

// Parser for the SELECT columns
fn parse_columns(input: &str) -> IResult<&str, Vec<String>> {
    preceded(
        tuple((tag("SELECT"), space1)),
        separated_list1(comma_space, parse_column), // Use the new column parser
    )(input)
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
            map(char('.'), |_| ""), // Handle '.'
            map(char('/'), |_| "/"), // Handle '/'
            map(preceded(char('.'), char('.')), |_| "../"), // Handle '..'
        ))),
        component,
    );

    // Combine the components into a full path
    let mut path_parser = map(
        many0(directory_component),
        |components: Vec<String>| {
            components.join("/") // Join components with '/'
        },
    );

    path_parser(input)
}


fn parse_table(input: &str) -> IResult<&str, String> {
    preceded(
        tuple((space0, tag("FROM"), space1)),
        parse_file
    )(input)
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
    ))(input)
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    preceded(
        tuple((space0, tag("WHERE"), space1)),
        separated_list1(
            tuple((space0, tag("AND"), space1)),
            parse_single_condition,
        ),
    )(input)
    .map(|(next_input, conditions)| {
        // Combine all conditions into a single AND condition
        let combined = conditions.into_iter().reduce(|acc, cond| {
            Condition::And(Box::new(acc), Box::new(cond))
        }).expect("Expected at least one condition"); // Will panic if conditions is empty

        (next_input, combined)
    })
}

fn parse_order_by(input: &str) -> IResult<&str, OrderBy> {
    preceded(
        tuple((space0, tag("ORDER BY"), space1)),
        // Parse the column name and the direction (ASC or DESC)
        map(
            tuple((
                parse_column,
                // Optional space followed by sort direction
                opt(preceded(space0, alt((map(tag("ASC"), |_| SortDirection::Ascending), map(tag("DESC"), |_| SortDirection::Descending))))),
            )),
            |(column, direction)| OrderBy {
                column,
                direction: direction.unwrap_or(SortDirection::Ascending), // Default to ascending if no direction specified
            },
        ),
    )(input)
}

pub fn parse_sql(input: &str) -> IResult<&str, SQLQuery> {
    let (input, columns) = parse_columns(input)?;
    let (input, table) = parse_table(input)?;
    let (input, condition) = opt(parse_condition)(input)?;
    let (input, order_by) = opt(parse_order_by)(input)?; // Add ORDER BY parsing

    Ok((input, SQLQuery { columns, table, condition, order_by }))
}