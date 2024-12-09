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

fn defrag_disk_data(vec_data : &mut Vec::<(i64,bool)>)
{
    let mut dst_test_index=0;
    let mut defrag_src_index=(vec_data.len() as i32)-1;

    while defrag_src_index>dst_test_index
    {
        if vec_data[defrag_src_index as usize].1 {
            let mut found_valid_dst=false;
            while !found_valid_dst && dst_test_index<defrag_src_index
            {
                if !vec_data[dst_test_index as usize].1 {
                    found_valid_dst=true;
                    vec_data.swap(defrag_src_index as usize,dst_test_index as usize); // Swap
                }
                dst_test_index+=1; // Increment dst pos
            }
        }
        defrag_src_index-=1; // Decrement src pos
    }
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
    defrag_disk_data(&mut disk_map);
    print_vec(&disk_map);
}
