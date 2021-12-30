use std::fs;
use super::matrix::Matrix;

fn day_to_string(day: usize) -> String {
    format!("{}{}", if day < 10 { "0" } else { "" }, day)
}

pub fn get_input(year: usize, day: usize) -> String {
    let path = format!("./input/{}/{}", year, day_to_string(day));
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

pub fn get_string_array(year: usize, day: usize, inline_seperator: &str) -> Vec<Vec<String>> {
    get_input(year, day).lines().map(| s | s.split(inline_seperator).map(| s | s.to_string()).collect()).collect()
}

pub fn get_nested_integer_array(year: usize, day: usize, inline_seperator: &str, general_seperator: &str) -> Vec<Vec<Vec<i64>>> {
    get_nested_string_array(year, day, inline_seperator, general_seperator).iter()
        .map(| v | v.iter()
            .map(| s | s.iter()
                .map(| s | s.parse().unwrap()).collect()
            ).collect()
        ).collect()
}

pub fn write_output_as_html(year: usize, day: usize, output: &str) {
    let path = format!("./output/{}/{}.html", year, day_to_string(day));
    match fs::write(&path, format!("<html><body style=\"font-family: monospace;width: 100vw; height: 100vh; overflow-x: hidden\">{}</body></html>", output.lines().collect::<Vec<&str>>().join("<br>"))) {
        Err(e) => panic!("Failed to write into file {} because of {}.", path, e),
        _ => {}
    };
}

pub fn get_matrix(year: usize, day: usize, element_seperator: &str) -> Matrix<i64> {
    let input = get_input(year, day).lines().map(| l | l.split(element_seperator).filter_map(| s | (s.len() > 0).then_some(s.to_string())).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    let mut matrix = Matrix::<i64>::new_with_default(vec![input[0].len(), input.len()]);
    for (y, r) in input.into_iter().enumerate() {
        for (x, v) in r.into_iter().enumerate() {
            *matrix.get_mut(&vec![x, y]).unwrap() = v.parse().unwrap_or(0) as i64;
        }
    }
    matrix
}