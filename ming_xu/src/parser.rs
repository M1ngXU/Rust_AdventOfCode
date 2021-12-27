use std::fs;

pub fn get_input(year: usize, day: usize) -> String {
    let path = format!("./input/{}/{}{}", year, if day < 10 { "0" } else { "" }, day);
    match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read file {} because of {}.", path, e)
    }
}

pub fn get_nested_string_array(year: usize, day: usize, inline_seperator: &str, general_seperator: &str) -> Vec<Vec<Vec<String>>> {
    get_input(year, day)
        .lines().map(| s | s.split(inline_seperator)
        .map(| s | s.split(general_seperator)
            .map(| s | s.to_string()).collect()
        ).collect()
    ).collect()
}

pub fn get_nested_integer_array(year: usize, day: usize, inline_seperator: &str, general_seperator: &str) -> Vec<Vec<Vec<i64>>> {
    get_nested_string_array(year, day, inline_seperator, general_seperator).iter()
        .map(| v | v.iter()
            .map(| s | s.iter()
                .map(| s | s.parse().unwrap()).collect()
            ).collect()
        ).collect()
}