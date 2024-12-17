use std::fs;
use colored::*;

#[derive(Clone, Debug)]
struct MachineState {
    r_a:i32, r_b:i32, r_c:i32,
    program:Vec<i32>,
}

fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");
    let lines:Vec<&str>=content.split("\n").filter(|line|line.len()>0).collect();
    let mut machine_state:MachineState=MachineState{r_a:0,r_b:0,r_c:0,program:Vec::new()};
    let mut program:Vec<i32>=Vec::new();
    for line in lines {
        if line.contains("Register A: ") { machine_state.r_a =line[12..].parse::<i32>().unwrap();}
        if line.contains("Register B: ") { machine_state.r_b =line[12..].parse::<i32>().unwrap();}
        if line.contains("Register C: ") { machine_state.r_c =line[12..].parse::<i32>().unwrap();}
        if line.contains("Program: ") { let program_values:Vec<i32>=line[9..].split(',')
                                                           .map(|value_str| value_str.parse::<i32>().unwrap())
                                                           .collect();machine_state.program.extend(program_values);};
    }
    println!("MachineState {:?}",machine_state);
}
