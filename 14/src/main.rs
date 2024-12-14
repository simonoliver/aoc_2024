use std::fs;
use regex;
use regex::Regex;

fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");
    let regex=Regex::new(r"p=(-*[0-9]+),(-*[0-9]+) v=(-*[0-9]+),(-*[0-9]+)").unwrap();
    let data:Vec<Vec<i64>>= regex.captures_iter(&content)
        .filter(|capture|capture.get(0).unwrap().as_str().len()>0)
        .map(|capture| {
            capture.iter().skip(1).map(|entry|entry.unwrap().as_str().parse::<i64>().unwrap()).collect()
        }).collect();

    println!("Data size {},{}",data[0].len(),data.len());
}


