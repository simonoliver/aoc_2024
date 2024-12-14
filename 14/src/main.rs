use std::fs;
use regex;
use regex::Regex;

fn process_movement (location_data: &mut Vec<((i64,i64),(i64,i64))>,area_size:(i64,i64), count : i64) {
    for _ in 0..count { // Iteration
        let loc_data_size=location_data.len();
        for loc_index in 0..loc_data_size {
            //for mut location in location_data {
            let (mut pos,vel)=location_data[loc_index];//location;
            pos=((pos.0+vel.0).rem_euclid(area_size.0),(pos.1+vel.1).rem_euclid(area_size.1));
            location_data[loc_index]=(pos,vel);
        }
    }
}

fn count_robots(location_data: &Vec<((i64, i64), (i64, i64))>, area_size: (i64, i64)) -> i64 {
    location_data.iter().fold(0,
        |acc,location| {
            if location.0.0==(area_size.0-1)/2 || location.0.1==(area_size.1-1)/2 {acc} else {acc+1}
        }
    )
}


fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");
    let regex=Regex::new(r"p=(-*[0-9]+),(-*[0-9]+) v=(-*[0-9]+),(-*[0-9]+)").unwrap();
    let area_size:(i64,i64)=(11,7); // (101,103)
    let mut location_data:Vec<((i64,i64),(i64,i64))>= regex.captures_iter(&content)
        .filter(|capture|capture.get(0).unwrap().as_str().len()>0)
        .map(|capture| {
            let data_row:Vec<i64>=capture.iter().skip(1).map(|entry|  entry.unwrap().as_str().parse::<i64>().unwrap()).collect();
            ((data_row[0],data_row[1]),(data_row[2],data_row[3]))
        }).collect();
    println!("Data size {}",location_data.len());

    process_movement(&mut location_data,area_size,100);
    println!("Pt1 - {}",count_robots(&location_data,area_size))
}



