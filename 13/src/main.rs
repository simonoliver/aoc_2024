use std::fs;
use regex::Regex;

fn find_lowest_win_combo(a_button:(i32,i32),b_button:(i32,i32),prize_pos:(i32,i32)) -> (bool,i32) {
    println!("AButton {:?} BButton {:?} Prize is {:?}",a_button,b_button,prize_pos);
    (false,0)
}

fn get_data(contents:&str,regex_formula:&str) -> Vec<(i32,i32)>
{
    let mut found_data:Vec<(i32,i32)> = Vec::new();
    let regex = Regex::new(regex_formula).unwrap();
    for capture in regex.captures_iter(contents) {
        let arg0=capture.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let arg1=capture.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        found_data.push((arg0,arg1));
    }
    found_data
}

fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");

    let button_a_entries:Vec<(i32,i32)>=get_data(&content,r"Button A: X\+([0-9]+), Y\+([0-9]+)");
    let button_b_entries:Vec<(i32,i32)>=get_data(&content,r"Button B: X\+([0-9]+), Y\+([0-9]+)");
    let prize_entries:Vec<(i32,i32)>=get_data(&content,r"Prize: X\=([0-9]+), Y=+([0-9]+)");

    println!("Entries: BA {} BB {} PR {}",button_a_entries.len(),button_b_entries.len(),prize_entries.len());
    let mut total_token_count=0;
    for i in 0..button_a_entries.len() {
        let (can_win,token_count) = find_lowest_win_combo(button_a_entries[i],button_b_entries[i],prize_entries[i]);
        if can_win {total_token_count+=token_count};
    }
    println!("Pt1 - Total token count {total_token_count}")
}


