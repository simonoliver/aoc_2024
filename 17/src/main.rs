use std::fs;
use std::process::exit;
use colored::*;

#[derive(Clone, Debug)]
struct MachineState {
    r_a:i32, r_b:i32, r_c:i32,ptr:i32,
    program:Vec<i32>,
}

fn get_combo_operand(machine_state:&MachineState,combo_operand:i32) -> i32{
    match combo_operand {
        1|2|3 => combo_operand,
        4 => machine_state.r_a,
        5 => machine_state.r_b,
        6 => machine_state.r_c,
        _ => {panic!("Unknown combo operand");0}
    }
}

fn process_step(machine_state:&mut MachineState) -> bool {
    let (instruction,operand)=(machine_state.program[machine_state.ptr as usize],machine_state.program[(machine_state.ptr+1) as usize]);
    match instruction {
        0 => {let output=(machine_state.r_a as f64)/((2 as i32).pow(get_combo_operand(&machine_state,operand) as u32) as f64);machine_state.r_a=output.floor() as i32} // adv
        1 => {let output= machine_state.r_b^operand;machine_state.r_b=output;} // bxl
        2 => {machine_state.r_b=get_combo_operand(machine_state,operand)%8;} // bst
        3 => {if machine_state.r_a!=0 {machine_state.ptr=operand;}} // jnz
        4 => {machine_state.r_b=machine_state.r_b ^ machine_state.r_c;} // bxc
        5 => {let output=get_combo_operand(machine_state,operand)%8;println!("output {}",output);} // out
        6 => {let output=(machine_state.r_a as f64)/((2 as i32).pow(get_combo_operand(&machine_state,operand) as u32) as f64);machine_state.r_b=output.floor() as i32} // bdv
        7 => {let output=(machine_state.r_a as f64)/((2 as i32).pow(get_combo_operand(&machine_state,operand) as u32) as f64);machine_state.r_c=output.floor() as i32} // cdv
        _ => {panic!("Bad instruction")}
    }
    if instruction!=3 { machine_state.ptr+=2;} // Progress instruction pointer
    machine_state.ptr<0 || machine_state.ptr>=machine_state.program.len() as i32 // Halt when exit bounds
}

fn main() {
    let content = fs::read_to_string("data/test_input").expect("Expected to read the file");
    let lines:Vec<&str>=content.split("\n").filter(|line|line.len()>0).collect();
    let mut machine_state:MachineState=MachineState{r_a:0,r_b:0,r_c:0,ptr:0,program:Vec::new()};
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
    let mut halt=false;
    while !halt {
        halt=process_step(&mut machine_state);
        //println!("Pos {}",machine_state.)
    }
    println!("Done");
}
