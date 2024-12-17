use std::fs;

#[derive(Clone, Debug)]
struct MachineState {
    r_a:i64, r_b:i64, r_c:i64,ptr:i64,
    program:Vec<i64>,
}

fn get_combo_operand(machine_state:&MachineState,combo_operand:i64) -> i64{
    match combo_operand {
        1|2|3 => combo_operand,
        4 => machine_state.r_a,
        5 => machine_state.r_b,
        6 => machine_state.r_c,
        _ => {panic!("Unknown combo operand");}
    }
}

fn process_step(machine_state:&mut MachineState,output_data:&mut Vec<i64>) -> bool {
    let (instruction,operand)=(machine_state.program[machine_state.ptr as usize],machine_state.program[(machine_state.ptr+1) as usize]);
    machine_state.ptr+=2; // Progress instruction pointer for next run. Allow modification with jump
    match instruction {
        0 => {let output=(machine_state.r_a as f64)/((2 as i64).pow(get_combo_operand(&machine_state,operand) as u32) as f64);machine_state.r_a=output.floor() as i64} // adv
        1 => {let output= machine_state.r_b^operand;machine_state.r_b=output;} // bxl
        2 => {machine_state.r_b=get_combo_operand(machine_state,operand)%8;} // bst
        3 => {if machine_state.r_a!=0 {machine_state.ptr=operand;}} // jnz
        4 => {machine_state.r_b=machine_state.r_b ^ machine_state.r_c;} // bxc
        5 => {let output=get_combo_operand(machine_state,operand)%8;output_data.push(output);} // out
        6 => {let output=(machine_state.r_a as f64)/((2 as i64).pow(get_combo_operand(&machine_state,operand) as u32) as f64);machine_state.r_b=output.floor() as i64} // bdv
        7 => {let output=(machine_state.r_a as f64)/((2 as i64).pow(get_combo_operand(&machine_state,operand) as u32) as f64);machine_state.r_c=output.floor() as i64} // cdv
        _ => {panic!("Bad instruction")}
    }
    machine_state.ptr<0 || machine_state.ptr>=machine_state.program.len() as i64 // Halt when exit bounds
}

fn get_output_string(output:&Vec<i64>) -> String {
    output.iter().map(|entry|entry.to_string()).collect::<Vec<_>>().join(",")
}

fn main() {
    let content = fs::read_to_string("data/input").expect("Expected to read the file");
    let lines:Vec<&str>=content.split("\n").filter(|line|line.len()>0).collect();
    let mut machine_state:MachineState=MachineState{r_a:0,r_b:0,r_c:0,ptr:0,program:Vec::new()};

    for line in lines {
        if line.contains("Register A: ") { machine_state.r_a =line[12..].parse::<i64>().unwrap();}
        if line.contains("Register B: ") { machine_state.r_b =line[12..].parse::<i64>().unwrap();}
        if line.contains("Register C: ") { machine_state.r_c =line[12..].parse::<i64>().unwrap();}
        if line.contains("Program: ") { let program_values:Vec<i64>=line[9..].split(',')
                                                           .map(|value_str| value_str.parse::<i64>().unwrap())
                                                           .collect();machine_state.program.extend(program_values);};
    }
    let machine_state_pt2=machine_state.clone();
    println!("MachineState {:?}",machine_state);
    let mut halt=false;
    let mut output:Vec<i64>=Vec::new();
    while !halt {
        halt=process_step(&mut machine_state,&mut output);
    }
    let output_string=get_output_string(&output);
    println!("Pt1 Done. Output {}",output_string);


    let mut test_replacement_r_a=0; // Needed a hint for this bit! 😵‍💫
    // Cycle through and solve per individual output value
    let mut pos:i32=(machine_state_pt2.program.len()-1) as i32;
    while pos>=0 {
        test_replacement_r_a<<=3;
        let mut found_pattern=false;
        while !found_pattern {
            let mut output_pt2:Vec<i64>=Vec::new();
            let mut halt_pt2=false;
            let mut machine_state_clone=machine_state_pt2.clone();
            machine_state_clone.r_a=test_replacement_r_a;
            while !halt_pt2 {
                halt_pt2=process_step(&mut machine_state_clone,&mut output_pt2);
            }
            if output_pt2==machine_state_clone.program[(pos as usize)..] {
                found_pattern=true;
            } else {
                test_replacement_r_a+=1;
            }
        }
        pos-=1;
    }
    println!("Pt 2 Replacement Found: {}",test_replacement_r_a);
}
