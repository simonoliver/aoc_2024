use std::collections::HashMap;
use std::fs;

fn main_old() {
    solve("data/input");
    //solve("data/test_input");
}

fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let (register_line,gate_lines) = content.split_once("\n\n").unwrap();
    let mut register_values:HashMap<&str,bool> = HashMap::new();
    for register_line in register_line.split("\n").filter(|line|!line.is_empty()) {
        let (register,value_string)=register_line.split_once(": ").unwrap();
        register_values.entry(register).or_insert(value_string.parse::<i32>().unwrap()>0);
    }
    let mut gate_logic_vec:Vec<(&str,&str,&str,&str)>=Vec::new();
    for gate_line in gate_lines.split("\n").filter(|line|!line.is_empty()) {
        let (gate_logic_string,out_register) = gate_line.split_once(" -> ").unwrap();
        let mut gate_logic_split=gate_logic_string.split(" ");
        gate_logic_vec.push((gate_logic_split.next().unwrap(),gate_logic_split.next().unwrap(),gate_logic_split.next().unwrap(),out_register));
    }

    let mut outstanding_gates=gate_logic_vec.clone();
    let mut check_gate_logic_index=0;
    while outstanding_gates.len()>0 {
        let test_gate=outstanding_gates[check_gate_logic_index];
        if register_values.contains_key(test_gate.0) && register_values.contains_key(test_gate.2) { // Are registers ready
            // Process
            let (register_0,register_1) = (*register_values.get(test_gate.0).unwrap(),*register_values.get(test_gate.2).unwrap());
            register_values.entry(test_gate.3).or_insert(
                match test_gate.1 {
                    "AND" => {register_0 && register_1},
                    "OR" => {register_0 || register_1},
                    "XOR" => {register_0 ^ register_1},
                    _ => {false}
                });
            // Remove
            outstanding_gates.remove(check_gate_logic_index);
        } else {
            check_gate_logic_index += 1;
        }
        if check_gate_logic_index>=outstanding_gates.len() {check_gate_logic_index=0;} // Loop
    }
    let mut all_register_ids:Vec<&str>=register_values.iter().map(|key_pair|*key_pair.0).collect();
    all_register_ids.sort();
    /*
    for register_id in &all_register_ids {
        println!("register {} -> {}",register_id,register_values.get(register_id).unwrap());
    }
    */
    let mut output_number:i64=0;
    for (index,register_id) in all_register_ids.iter().filter(|register|register.contains('z')).enumerate() {
        if *register_values.get(register_id).unwrap() {output_number |= 1<<index;}
        println!("Check register index {index}");
    }
    println!("Pt1 - Output number {}",output_number);

}
