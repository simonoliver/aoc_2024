use std::fs;

fn create_disk_map(input:&str) -> Vec::<(i64,bool)>
{
    let compressed_representation:Vec<u32>=input.chars()
        .filter(|char| {char.is_numeric()})
        .into_iter().map(|input_char|input_char.to_digit(10).unwrap()).collect();
    let mut disk_map:Vec<(i64,bool)>=Vec::new();
    let mut file_index=0;
    let mut is_file_data=true;
    for compressed_value in compressed_representation {
        let write_value= if is_file_data {(file_index,true)} else {(0,false)}; // Either push index or empty
        for _ in 0..compressed_value {
            disk_map.push(write_value);
        }
        if is_file_data {file_index+=1;}
        is_file_data=!is_file_data; // Flip
    }
    disk_map
}

fn print_vec(vec_data : &Vec::<(i64,bool)>)
{
    let mut output_string=String::new();
    for data in vec_data
    {
        let value_string= if data.1 {data.0.to_string()} else {".".to_string()};
        output_string+=&value_string;
    }
    println!("Uncompressed output {}",output_string);
}

fn main() {
    let content=fs::read_to_string("data/test_input").expect("Expected to read file");
    let mut disk_map=create_disk_map(&content);
    print_vec(&disk_map);
}
