use aid::csv_utils;
use std::io;

pub fn load_and_query(sql: &str) -> Result<(Vec<String>, Vec<Vec<String>>), io::Error> {
    // Use the input SQL parameter instead of hardcoded SQL
    let parsed = match csv_utils::parse_sql(sql) {
        Ok(result) => result,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse SQL query: {}", e),
            ))
        }
    };

    let query = parsed.1;

    // Load the CSV file
    let (headers, records) = match csv_utils::load_csv(&query.table) {
        Ok(result) => result,
        Err(e) => {
            eprintln!();
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to load CSV file '{}': {}", query.table, e),
            ));
        }
    };

    // Apply the query
    Ok(csv_utils::apply_query(&headers, records, &query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_all() {
        let result = load_and_query("SELECT * FROM ./vgsales.csv");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(
                vec![
                    "Rank",
                    "Name",
                    "Platform",
                    "Year",
                    "Genre",
                    "Publisher",
                    "NA_Sales",
                    "EU_Sales",
                    "JP_Sales",
                    "Other_Sales",
                    "Global_Sales"
                ],
                headers
            );
            assert_eq!(
                vec![
                    "1",
                    "Wii Sports",
                    "Wii",
                    "2006",
                    "Sports",
                    "Nintendo",
                    "41.49",
                    "29.02",
                    "3.77",
                    "8.46",
                    "82.74"
                ],
                data[0]
            );
            assert_eq!(
                vec![
                    "16600",
                    "Spirits & Spells",
                    "GBA",
                    "2003",
                    "Platform",
                    "Wanadoo",
                    "0.01",
                    "0",
                    "0",
                    "0",
                    "0.01"
                ],
                data[16597]
            );
            assert_eq!(16598, data.len());
        }
    }

    #[test]
    fn count_all() {
        let result = load_and_query("SELECT COUNT(*) FROM ./vgsales.csv");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["COUNT(*)"], headers);
            assert_eq!(vec!["16598"], data[0]);
            assert_eq!(1, data.len());
        }
    }

    #[test]
    fn select_multi_column() {
        let result = load_and_query(
            "SELECT Year,Name FROM ./vgsales.csv WHERE Platform = 'Wii' AND Year = 2015",
        );

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["Year", "Name"], headers);
            assert_eq!(vec!["2015", "Just Dance 2016"], data[0]);
            assert_eq!(vec!["2015", "Monster High: New Ghoul in School"], data[3]);
            assert_eq!(4, data.len());
        }
    }

    #[test]
    fn select_distinct() {
        let result = load_and_query("SELECT DISTINCT Platform FROM vgsales.csv WHERE Publisher LIKE 'Nin%' ORDER BY Platform DESC");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["Platform"], headers);
            assert_eq!(vec!["WiiU"], data[0]);
            assert_eq!(vec!["3DS"], data[9]);
            assert_eq!(10, data.len());
        }
    }

    #[test]
    fn select_between() {
        let result = load_and_query("SELECT DISTINCT Platform FROM vgsales.csv WHERE Year BETWEEN 2005 AND 2006 ORDER BY Platform ASC");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["Platform"], headers);
            assert_eq!(vec!["DS"], data[0]);
            assert_eq!(vec!["XB"], data[9]);
            assert_eq!(10, data.len());
        }
    }

    #[test]
    fn select_filter_or() {
        let result = load_and_query("SELECT DISTINCT Publisher FROM vgsales.csv WHERE Name LIKE 'Call of Duty%' OR Name LIKE 'Battlefield%' ORDER BY Publisher ASC");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["Publisher"], headers);
            assert_eq!(vec!["Activision"], data[0]);
            assert_eq!(vec!["Electronic Arts"], data[1]);
            assert_eq!(2, data.len());
        }
    }

    #[test]
    fn select_filter_and_or() {
        let result = load_and_query("SELECT Name FROM vgsales.csv WHERE Year = 1984 AND (Genre = 'Puzzle' OR Genre = 'Racing') ORDER BY Year DESC");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["Name"], headers);
            assert_eq!(vec!["Excitebike"], data[0]);
            assert_eq!(vec!["Beamrider"], data[5]);
            assert_eq!(6, data.len());
        }
    }
    #[test]
    fn select_filter_not() {
        let result = load_and_query("SELECT DISTINCT Platform FROM vgsales.csv WHERE Name LIKE 'Call of Duty%' AND NOT (Platform = 'PC' OR Platform = 'DS') ORDER BY Platform DESC");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["Platform"], headers);
            assert_eq!(vec!["XOne"], data[0]);
            assert_eq!(vec!["GC"], data[10]);
            assert_eq!(11, data.len());
        }
    }

    #[test]
    fn select_group_by() {
        let result = load_and_query(
            "SELECT Genre, COUNT(Name) FROM vgsales.csv GROUP BY Genre ORDER BY COUNT(Name) DESC",
        );

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(vec!["Genre", "COUNT(Name)"], headers);
            assert_eq!(vec!["Action", "3316"], data[0]);
            assert_eq!(vec!["Puzzle", "582"], data[11]);
            assert_eq!(12, data.len());
        }
    }

    #[test]
    fn select_aggregate() {
        let result = load_and_query("SELECT Genre,COUNT(Name),SUM(Global_Sales),MIN(Global_Sales),MAX(Global_Sales),AVG(Global_Sales) FROM vgsales.csv GROUP BY Genre ORDER BY SUM(Global_Sales) DESC");

        if let Err(e) = &result {
            eprint!("{}", e);
            assert!(!result.is_err());
        }

        if let Ok((headers, data)) = result {
            assert_eq!(
                vec![
                    "Genre",
                    "COUNT(Name)",
                    "SUM(Global_Sales)",
                    "MIN(Global_Sales)",
                    "MAX(Global_Sales)",
                    "AVG(Global_Sales)"
                ],
                headers
            );
            assert_eq!(
                vec![
                    "Action",
                    "3316",
                    "1751.1799999999691",
                    "0.01",
                    "8.24",
                    "0.5281001206272524"
                ],
                data[0]
            );
            assert_eq!(
                vec![
                    "Strategy",
                    "681",
                    "175.1200000000004",
                    "0.01",
                    "5.45",
                    "0.2571512481644646"
                ],
                data[11]
            );
            assert_eq!(12, data.len());
        }
    }
}
