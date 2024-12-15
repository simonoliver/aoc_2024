use std::fs;
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

fn get_quadrant_index(position:(i64,i64),area_size: (i64, i64)) -> i64 {
    let mid_point=((area_size.0-1)/2,(area_size.1-1)/2);
    if position.0<mid_point.0 {
        if position.1<mid_point.1 {return 0;}
        if position.1>mid_point.1 {return 1;}
    }
    if position.0>mid_point.0 {
        if position.1<mid_point.1 {return 2;}
        if position.1>mid_point.1 {return 3;}
    }
    -1 // Not in a quadrant
}

fn count_robots(location_data: &Vec<((i64, i64), (i64, i64))>, area_size: (i64, i64)) -> i64 {
    let quadrants:Vec<i64>=
    location_data.iter().fold(vec!(0;4),
        |mut acc,location| {
            let quadrant=get_quadrant_index((location.0.0,location.0.1),area_size);
            if quadrant>=0 {acc[quadrant as usize]+=1;}
            acc
            //if quadrant<0 {acc}
            //else acc[quadrant]+=1;acc
            //if location.0.0==(area_size.0-1)/2 || location.0.1==(area_size.1-1)/2 {acc} else {acc+1}
        }
    );
    quadrants[0]*quadrants[1]*quadrants[2]*quadrants[3]
}

fn print_grid(location_data: &Vec<((i64,i64),(i64,i64))>,area_size:(i64,i64)) {
    let mut grid:Vec<Vec<i64>>=Vec::new();
    for _ in 0..area_size.1 {
        grid.push(vec!(0;area_size.0 as usize));
    }
    for (_,location_data) in location_data.iter().enumerate() {
       let location=location_data.0;
        grid[location.1 as usize][location.0 as usize]+=1; // Append count
    }

    for (row_index,row) in grid.iter().enumerate() {
        let mut string_line=String::new();
        for (column_index,value) in row.iter().enumerate() {
            if row_index==(area_size.1 as usize-1)/2 {string_line.push(' ')}
            else if column_index==(area_size.0 as usize-1)/2 {string_line.push(' ')}
            else if *value==0 {string_line.push('.');}
            else {string_line.push_str(value.to_string().as_str());}
        }
        println!("{string_line}");
    }
}

fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");
    let regex=Regex::new(r"p=(-*[0-9]+),(-*[0-9]+) v=(-*[0-9]+),(-*[0-9]+)").unwrap();
    let area_size:(i64,i64)= (11,7); //(101,103); //
    let mut location_data:Vec<((i64,i64),(i64,i64))>= regex.captures_iter(&content)
        .filter(|capture|capture.get(0).unwrap().as_str().len()>0)
        .map(|capture| {
            let data_row:Vec<i64>=capture.iter().skip(1).map(|entry|  entry.unwrap().as_str().parse::<i64>().unwrap()).collect();
            ((data_row[0],data_row[1]),(data_row[2],data_row[3]))
        }).collect();
    println!("Data size {}",location_data.len());

    process_movement(&mut location_data,area_size,100);
    print_grid(&location_data,area_size);
    println!("Pt1 - {}",count_robots(&location_data,area_size));
}



