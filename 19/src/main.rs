use std::fmt::Debug;
use std::fs;
use memoize::memoize;

fn main() {
    //solve("data/input");
    solve("data/test_input");
}

//#[memoize]
fn find_section_combos_in_pattern (remaining_design:String,pattern_sections_string:String) -> i32 {
    let pattern_sections=pattern_sections_string.split(",");
    let mut found_combos=0;
    for mut pattern in pattern_sections {
        pattern=pattern.trim();
        println!("Checking pattern '{}'",pattern);
        let pattern_len=pattern.len();
        // If first section matches
        if remaining_design.len()>=pattern.len() && remaining_design[0..pattern_len].contains(pattern) {
            if remaining_design.len()==pattern.len() {
                found_combos+=1; // We've found the end. Whole pattern matches
            } else {
                // Recurse on remaining section
                let pattern_sections_string_clone=pattern_sections_string.clone();
                found_combos+=find_section_combos_in_pattern(remaining_design[pattern_len..].to_string(),pattern_sections_string_clone);
            }
        }
    }
    found_combos
}

fn solve(input_path:&str) {

   let content=fs::read_to_string(input_path).expect("Expect to load file");
    let lines=content.split("\n").filter(|line|!line.is_empty());
    let mut pattern_sections:Vec<&str>=Vec::new();
    let mut target_designs:Vec<&str>=Vec::new();
    let mut pattern_sections_string=String::new();
    for (index,line) in lines.enumerate() {
        if index==0 { pattern_sections.extend(line.split(","));pattern_sections_string.push_str(line); } // Add patterns
        else {target_designs.push(line);}
    }
    println!("Sections {} Designs {}",pattern_sections.len(),target_designs.len());
    let mut total_combos=0;
    let mut valid_designs=0;
    for target_design in target_designs {
        let mut target_string=String::new();
        target_string.push_str(target_design);
        let mut cloned_pattern_string=pattern_sections_string.clone();
        let combos=find_section_combos_in_pattern(target_string,cloned_pattern_string);
        total_combos+=combos;
        if combos>0 {valid_designs+=1};
    }
    println!("Pt1 - Total combos {} valid_designs {}",total_combos,valid_designs);
 }
