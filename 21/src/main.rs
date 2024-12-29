use std::collections::HashMap;
use std::{cmp, fs};
use grid::Grid;

const DIRECTION_CHARS:[char;4]=['<','>','^','v'];
const DIRECTION_CHARS_VERT_FIRST:[char;4]=['^','v','<','>'];

struct Keypad {
    grid: Grid<char>,
    entry_position_table: HashMap<char,(usize,usize)>, // Row/column
}

impl Keypad {

    fn get_move_sequence_for_output(self:&Keypad,required_output:&Vec<char>,position:(usize,usize)) -> (Vec<char>,(usize,usize)) {
        let mut sequence:Vec<char>=Vec::new();
        let mut current_position=position;
        for char in required_output {
            let target_pos=self.entry_position_table.get(&char).unwrap();
            let diff=(target_pos.0 as i32 - current_position.0 as i32,target_pos.1 as i32 - current_position.1 as i32);

            // In order to address issues with spaces, we'll check if dest row or target row have a space in
            let space_pos=self.entry_position_table.get(&' ').unwrap();
            let mut up_down:Vec<char>=vec!{if (diff.0 > 0) {'v'} else {'^'};diff.0.abs() as usize};
            let mut left_right:Vec<char>=vec!{if (diff.1 > 0) {'>'} else {'<'};diff.1.abs() as usize};

            if diff.1 > 0 && (target_pos.0,current_position.1)!=*space_pos {
                sequence.append(&mut up_down);
                sequence.append(&mut left_right);
            } else if (current_position.0,target_pos.1)!=*space_pos {
                sequence.append(&mut left_right);
                sequence.append(&mut up_down);
            } else {
                sequence.append(&mut up_down);
                sequence.append(&mut left_right);
            }
            
            sequence.push('A'); // Add "Press" button
            current_position=*target_pos; // Now at target pos
        }
        (sequence,current_position)
    }

    fn new_from_vec(input:Vec<char>,width:usize) -> Self {
        let grid=Grid::from_vec(input,width);
        let mut entry_position_table:HashMap<char,(usize,usize)>=HashMap::new();
        for grid_entry in grid.indexed_iter() { // Record each
            //println!("Registering {} at {:?}",grid_entry.1,(grid_entry.0.0,grid_entry.0.1));
            entry_position_table.entry(*grid_entry.1).or_insert((grid_entry.0.0,grid_entry.0.1)); // Row, column
        }
        Self {
            grid:grid,
            entry_position_table:entry_position_table
        }
    }
}

fn main() {
    solve("data/input");
    //solve("data/test_input");
}

fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let lines=content.split("\n").filter(|line|!line.is_empty());
    let num_keypad=Keypad::new_from_vec(vec!{'7','8','9','4','5','6','1','2','3',' ','0','A'},3);
    let dir_keypad=Keypad::new_from_vec(vec!{' ','^','A','<','v','>'},3);
    let mut complexity_sum=0;
    let mut keypad_position:(usize,usize)=(3,2);
    let mut dirpad_position_0:(usize,usize)=(0,2);
    let mut dirpad_position_1:(usize,usize)=(0,2);
    for line in lines {
        let (sequence_keypad,keypad_position_new)=num_keypad.get_move_sequence_for_output(&line.chars().collect(),keypad_position); // Row, column
        let (sequence_dir_pad_0,dirpad_position_0_new)=dir_keypad.get_move_sequence_for_output(&sequence_keypad,dirpad_position_0);
        let (sequence_dir_pad_1,dirpad_position_1_new)=dir_keypad.get_move_sequence_for_output(&sequence_dir_pad_0,dirpad_position_1);
        (keypad_position,dirpad_position_0,dirpad_position_1)=(keypad_position_new,dirpad_position_0_new,dirpad_position_1_new);

        //println!("keypad_position {:?} from {:?}",keypad_position,keypad_position_new);
        let output_str_keypad=sequence_keypad.iter().fold(String::new(),|mut acc,entry|{acc.push(*entry);acc});
        let output_str_dir_pad_0=sequence_dir_pad_0.iter().fold(String::new(),|mut acc,entry|{acc.push(*entry);acc});
        let output_str_dir_pad_1 =sequence_dir_pad_1.iter().fold(String::new(), |mut acc, entry|{acc.push(*entry);acc});
        let input_numeric_value=line.chars().filter(|entry|entry.is_numeric()).fold(String::new(),|mut acc,entry|{acc.push(entry);acc}).parse::<i32>().unwrap();
        let complexity=sequence_dir_pad_1.len() as i32 * input_numeric_value;
        println!("{line}");
        println!("{output_str_keypad}");
        println!("{output_str_dir_pad_0}");
        println!("{output_str_dir_pad_1}");
        complexity_sum+=complexity;
        println!("Output for {} is {:?} length {} Numeric value {input_numeric_value}", line, &output_str_dir_pad_1, output_str_dir_pad_1.len());
    }
    println!("Pt1 - Complexity sum {complexity_sum}");
}
