use std::collections::HashMap;
use std::fs;

fn process_stones(stone_data : &mut Vec<i64>) {
    let mut process_index=0;
    let mut reached_end=false;
    while !reached_end {
        let stone_value=stone_data[process_index];
        if stone_value==0 {stone_data[process_index]=1;}
        else if stone_value.to_string().len()%2==0 {
            let stone_value_string=stone_value.to_string();
            let (stone_string_left,stone_string_right)=stone_value_string.split_at(stone_value_string.len()/2);
            stone_data[process_index]=stone_string_left.parse::<i64>().unwrap();
            process_index+=1;
            stone_data.insert(process_index,stone_string_right.parse::<i64>().unwrap());
        }
        else {stone_data[process_index]*=2024};
        process_index+=1;
        if process_index>=stone_data.len() {reached_end=true;}
    }
}

fn process_stone_single_iter(stone_value : i64) -> Vec<i64> {
    if stone_value==0 {return vec!{1}};
    if stone_value.to_string().len()%2==0 {
        let stone_value_string = stone_value.to_string();
        let (stone_string_left, stone_string_right) = stone_value_string.split_at(stone_value_string.len() / 2);
        return vec! {stone_string_left.parse::<i64>().unwrap(), stone_string_right.parse::<i64>().unwrap()};
    }
    vec!{stone_value*2024}
}

fn recurse_stone_iter(stone_value : i64, iter_count : i64, cached_values : &mut HashMap<i64,Vec<i64>>) -> i64 {
    let mut stone_count=0;

    // We want a sum of all expanded values

    // How to do rust recursive function mutable reference
    let expanded_values=cached_values.entry(stone_value).or_insert(process_stone_single_iter(stone_value)).clone();  // Have to clone here for local use
    //println!("Cached expanded values for {stone_value} size {} contents {:?}",expanded_values.len(),expanded_values);

    if iter_count>0 { // At least one iteration remaining
        for expanded_stone_value in expanded_values {
            // Add the sub of the expansion of each stone value
            stone_count+=recurse_stone_iter(expanded_stone_value,iter_count-1,cached_values);
        }
    } else {
        stone_count+=expanded_values.len() as i64; // End of iteration
        //println!("Final contents {:?}",expanded_values);
    }
    stone_count
}




fn main() {
    let content=fs::read_to_string("data/input").expect("Expected to read the file");
    let mut stone_data:Vec<i64>=content.split("\n").next().unwrap().split(" ")
        .filter(|value_string| match value_string.parse::<i64>() {
            Ok(_) => {true}
            Err(_) => {false}
        })
        .map(|value_string|
            value_string.parse::<i64>().unwrap()
        ).collect();

    let pt2_stone_data=stone_data.clone();

    for _ in 0..25 {
        process_stones(&mut stone_data)
    }
    println!("Pt1 - Stone count {}",stone_data.len());

    let mut expanded_stone_map : HashMap<i64,Vec<i64>> = HashMap::new();
    let mut stone_count=0;
    let iter_count=25;
    for stone_value in pt2_stone_data.iter() {

        let stone_count_inc=recurse_stone_iter(*stone_value,iter_count-1,&mut expanded_stone_map);
        println!("Processing value for {stone_value} stone_count {stone_count_inc}");
        stone_count+=stone_count_inc;
    }
    println!("Pt2 - Stone count {}",stone_count);

}
