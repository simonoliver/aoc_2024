use std::fs;
use std::cmp;
use regex::Regex;

fn find_lowest_win_combo(a_button:(i64,i64),b_button:(i64,i64),prize_pos:(i64,i64)) -> (bool,i64) {
    println!("AButton {:?} BButton {:?} Prize is {:?}",a_button,b_button,prize_pos);
    // We want to find the solution where a_count*a_button+b_count*b_button=prize_pos and
    // A is 3 tokens, B is 1. So we want the smallest value of B while still satisfying it
    let max_b= cmp::max(prize_pos.0/b_button.0,prize_pos.1/b_button.1); // max B before exceeding
    for b_count in 0..max_b {
        let mut exceeded=false;
        let mut a_count=0;
        while !exceeded {
            let total=(b_button.0*b_count+a_button.0*a_count,b_button.1*b_count+a_button.1*a_count);
            if total==prize_pos {
                return (true,a_count*3+b_count);
            }
            if total.0>prize_pos.0 || total.1>prize_pos.1 {exceeded=true;}
            a_count+=1;
        }
    }
    (false,0)
}

fn get_data(contents:&str,regex_formula:&str) -> Vec<(i64,i64)>
{
    let mut found_data:Vec<(i64,i64)> = Vec::new();
    let regex = Regex::new(regex_formula).unwrap();
    for capture in regex.captures_iter(contents) {
        let arg0=capture.get(1).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
        let arg1=capture.get(2).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
        found_data.push((arg0,arg1));
    }
    found_data
}

fn main() {
    let content = fs::read_to_string("data/input").expect("Expected to read the file");

    let button_a_entries:Vec<(i64,i64)>=get_data(&content,r"Button A: X\+([0-9]+), Y\+([0-9]+)");
    let button_b_entries:Vec<(i64,i64)>=get_data(&content,r"Button B: X\+([0-9]+), Y\+([0-9]+)");
    let prize_entries:Vec<(i64,i64)>=get_data(&content,r"Prize: X\=([0-9]+), Y=+([0-9]+)");

    println!("Entries: BA {} BB {} PR {}",button_a_entries.len(),button_b_entries.len(),prize_entries.len());
    let mut total_token_count=0;
    for i in 0..button_a_entries.len() {
        let (can_win,token_count) = find_lowest_win_combo(button_a_entries[i],button_b_entries[i],prize_entries[i]);
        if can_win {total_token_count+=token_count};
    }
    println!("Pt1 - Total token count {total_token_count}");
    let mut total_token_count=0;
    for i in 0..button_a_entries.len() {
        let (can_win,token_count) = find_lowest_win_combo(button_a_entries[i],button_b_entries[i],(prize_entries[i].0+10000000000000,prize_entries[i].1+10000000000000));
        if can_win {total_token_count+=token_count};
    }
    println!("Pt2 - Total token count {total_token_count}");
}


