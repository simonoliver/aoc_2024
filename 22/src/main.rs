use std::collections::HashMap;
use std::fs;

fn main() {
    solve("data/input");
    //solve("data/test_input");
}

fn calc_next_number(secret_number:i64) -> i64{
    let mix_value_1=secret_number*64;
    let mut new_secret_number=(secret_number ^ mix_value_1)%16777216;
    let mix_value_2 = ((new_secret_number as f64)/32.0).floor() as i64;
    new_secret_number=(new_secret_number ^ mix_value_2)%16777216;
    let mix_value_3=new_secret_number*2048;
    new_secret_number=(new_secret_number ^ mix_value_3)%16777216;
    new_secret_number
}

fn process_number_iterations(secret_number:i64, iter_count:u32) -> i64{
    let mut output_secret_number=secret_number;
    for i in 0..iter_count {
        output_secret_number=calc_next_number(output_secret_number)
    }
    output_secret_number
}


fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let input_numbers : Vec<i64> = content.split("\n").filter(|line|!line.is_empty()).map(|line|line.parse::<i64>().unwrap()).collect();
    let mut sum:i64=0;
    let iter_count=2000;
    // Pt1
    for input_number in &input_numbers {
        let processed_number=process_number_iterations(*input_number,iter_count);
        sum+=processed_number;
       // println!("Input {input_number} output {processed_number}");
    }
    println!("Pt1 - Sum is {sum}");

    // Pt2
    let mut add_banana_hashmap:HashMap<(i32,i32,i32,i32),i64>=HashMap::new();
    for input_number in &input_numbers {
        let mut output_secret_number = *input_number;
        let mut diff_queue: Vec<i32> = Vec::new();
        let mut recorded_tuples_for_number:HashMap<(i32,i32,i32,i32),bool>=HashMap::new(); // Only record the first
        for i in 0..iter_count {
            let last_digit = output_secret_number.to_string().chars().last().unwrap().to_digit(10).unwrap() as i32;
            output_secret_number = calc_next_number(output_secret_number);
            let new_digit = output_secret_number.to_string().chars().last().unwrap().to_digit(10).unwrap() as i32;
            let diff = new_digit - last_digit;
            if diff_queue.len() == 4 { diff_queue.remove(0); } // Ensure queue is 4 in length
            diff_queue.push(diff);
            if diff_queue.len() == 4 {
                let tuple=(diff_queue[0],diff_queue[1],diff_queue[2],diff_queue[3]);
                if !recorded_tuples_for_number.contains_key(&tuple) {
                    add_banana_hashmap.entry(tuple).and_modify(|val| { *val += new_digit as i64; }).or_insert(new_digit as i64);
                    recorded_tuples_for_number.entry(tuple).or_insert(true); // Ensure not added again for this number sequence
                }
            }
        }
    }
    let max_bananas_key=add_banana_hashmap.iter().max_by_key(|entry|entry.1).unwrap();
    let max_bananas=add_banana_hashmap.values().max().unwrap();
    println!("Pt2 - largest banana count is {} {:?}",max_bananas,max_bananas_key);
}
