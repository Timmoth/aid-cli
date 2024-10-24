use std::collections::BTreeMap;
use std::env;
use std::env::consts::OS;

use regex::Regex;

pub fn print_env_vars(key_filter: Option<String>, value_filter: Option<String>) {
    let mut env_vars: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let separator = if OS == "windows" { ';' } else { ':' };

    let key_regex = key_filter.map(|kf| Regex::new(&kf).unwrap());
    let value_regex = value_filter.map(|vf| Regex::new(&vf).unwrap());

    for (key, value) in env::vars() {
        if let Some(ref kr) = key_regex {
            if !kr.is_match(&key.to_lowercase()) {
                continue;
            }
        }

        let split_values: Vec<String> = value
            .split(separator)
            .map(|s| s.trim().to_string())
            .filter(|v| !v.is_empty()) // Filter out empty values
            .collect();

        let filtered_values: Vec<String> = if let Some(ref vr) = value_regex {
            split_values
                .into_iter()
                .filter(|v| vr.is_match(&v.to_lowercase()))
                .collect()
        } else {
            split_values
        };

        if !filtered_values.is_empty() {
            env_vars.insert(key, filtered_values);
        }
    }

    for (key, values) in env_vars {
        println!("{}:", key);
        for value in values {
            println!("\t{}", value);
        }
    }
}
