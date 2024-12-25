use std::fs;
use itertools::Itertools;

fn main() {
    //solve("data/input");
    solve("data/test_input");
}

//#derive![Debug]
struct KeyData (bool,Vec<i32>);

fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let section_split=content.split("\n\n");
    //println!("Sections total {}",&section_split.count());

    let mut key_data:Vec<KeyData>=Vec::new();
    for (section_index,key_string_data) in section_split.enumerate() {
        let mut line_data=key_string_data.split("\n");

        let mut is_lock=false;
        let mut column_count:Vec<i32>=Vec::new();
        for (line_index,line) in line_data.enumerate() {
            let mut ignore_line=false;
            if line_index==0 {
                let line_length=line.len();
                column_count=vec![0;line_length];
                if line.chars().all(|line_char|line_char=='#') {is_lock=true;ignore_line=true;} // First line all # to be a lock)
            }
            if line_index==6 && !is_lock {ignore_line=true;} // Ignore key line
            //println!("Line index {} line {}",line_index,line);

            for (index,line_char) in line.chars().enumerate() {
                if line_char=='#' && !ignore_line {column_count[index]+=1;}
            }
        }
        println!("Section index {} line data {} is_lock {}",section_index,column_count.iter().join(","),is_lock);
    }

}
