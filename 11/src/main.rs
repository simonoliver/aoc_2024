use memoize::memoize;
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
#[memoize]
fn recurse_stone_iter(stone_value : i64, iter_count : i64, sum: i64) -> i64 {
    if iter_count==0 {return sum;}
    if stone_value==0 {return recurse_stone_iter(1, iter_count-1, sum);}
    if stone_value.to_string().len()%2==0 {
        let stone_value_string = stone_value.to_string();
        let (stone_string_left, stone_string_right) = stone_value_string.split_at(stone_value_string.len() / 2);
        let (stone_left,stone_right) = (stone_string_left.parse::<i64>().unwrap(),stone_string_right.parse::<i64>().unwrap());
        return recurse_stone_iter(stone_left,iter_count-1,sum)+recurse_stone_iter(stone_right,iter_count-1,sum);
    }
    recurse_stone_iter(stone_value*2024,iter_count-1,sum)
}

fn main() {
    let content = fs::read_to_string("data/input").expect("Expected to read the file");
    let mut stone_data: Vec<i64> = content.split("\n").next().unwrap().split(" ")
        .filter(|value_string| match value_string.parse::<i64>() {
            Ok(_) => { true }
            Err(_) => { false }
        })
        .map(|value_string|
            value_string.parse::<i64>().unwrap()
        ).collect();

    let pt2_stone_data=stone_data.clone();


    for _ in 0..25 {
        process_stones(&mut stone_data)
    }
    println!("Pt1 - Stone count {}",stone_data.len());

    let mut stone_count=0;
    for stone_value in pt2_stone_data.iter() {

        let stone_count_inc=recurse_stone_iter(*stone_value,75,1);
        //println!("Processing value for {stone_value} stone_count {stone_count_inc}");
        stone_count+=stone_count_inc;
    }
    println!("Pt2 - Stone count {}",stone_count);
}


