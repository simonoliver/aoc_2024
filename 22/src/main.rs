use std::fs;

fn main() {
    //solve("data/input");
    solve("data/test_input");
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
    for input_number in &input_numbers {
        let processed_number=process_number_iterations(*input_number,2000);
        sum+=processed_number;
        println!("Input {input_number} output {processed_number}");
    }
    println!("Pt1 - Sum is {sum}");
}
