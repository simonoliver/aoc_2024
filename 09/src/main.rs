use std::fs;

fn create_disk_map(input:&str) -> Vec::<(i64,bool)>
{
    let compressed_representation:Vec<u32>=input.chars()
        .filter(|char| {char.is_numeric()}).map(|input_char|input_char.to_digit(10).unwrap()).collect();
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

fn defrag_disk_data_pt1(vec_data : &mut Vec::<(i64,bool)>) {
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

fn defrag_disk_data_pt2(vec_data : &mut Vec::<(i64,bool)>) {

    let mut defrag_src_index=(vec_data.len() as i32)-1;
    while defrag_src_index>0 // Loop back to start
    {
        if vec_data[defrag_src_index as usize].1 {
            let file_index=vec_data[defrag_src_index as usize].0;
            let file_end_index=defrag_src_index;
            let mut file_start_index=defrag_src_index;
            let mut found_file_start=false;
            while !found_file_start
            {
                if defrag_src_index<0 {
                    file_start_index = defrag_src_index + 1; // Previous location is start pos
                    found_file_start = true;
                }
                else {
                    let block_data = vec_data[defrag_src_index as usize];
                    if !block_data.1 || block_data.0 != file_index { // Either empty or diff index
                        file_start_index = defrag_src_index + 1; // Previous location is start pos
                        found_file_start = true;
                        defrag_src_index+=1; // Increment back one to support next file start
                    } else {
                        defrag_src_index -= 1; // Decrement
                    }
                }
            }
            let file_length=(file_end_index+1)-file_start_index;
            //println!("Moving file index {file_index} length {file_length} - start index {file_start_index}->{file_end_index}");
            let mut dst_test_index=0; // We'll restart at 0 each time
            let mut found_valid_dst=false;
            while !found_valid_dst && dst_test_index<file_start_index
            {
                if !vec_data[dst_test_index as usize].1 { // Empty
                    // Calc space
                    let mut sufficient_space=true;
                    // Check each location ahead also clear
                    for size_test in 1..file_length {
                        if vec_data[(dst_test_index+size_test) as usize].1 {
                            sufficient_space=false;
                        }
                    }
                    if sufficient_space {
                        found_valid_dst = true; // End loop
                        // Swap each location
                        for size_index in 0..file_length {
                            vec_data.swap((file_start_index + size_index) as usize, (dst_test_index + size_index) as usize); // Swap
                        }
                    }
                    // TODO - Could also skip index forward a little as don't need to recheck
                }
                dst_test_index+=1; // Increment dst pos if not found
            }
        }
        defrag_src_index-=1; // Decrement src pos
    }
}

fn calc_checksum (vec_data : &Vec::<(i64,bool)>) -> i64{
    let mut checksum=0;
    for (index,data) in vec_data.iter().enumerate() {
        if data.1 {checksum+=data.0*(index as i64)};
    }
    checksum
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
    let mut disk_map_pt1=create_disk_map(&content);
    let mut disk_map_pt2=disk_map_pt1.clone();
    print_vec(&disk_map_pt1);
    defrag_disk_data_pt1(&mut disk_map_pt1);
    print_vec(&disk_map_pt1);
    println!("Pt1 - checksum {}",calc_checksum(&disk_map_pt1));

    defrag_disk_data_pt2(&mut disk_map_pt2);
    print_vec(&disk_map_pt2);
    println!("Pt2 - checksum {}",calc_checksum(&disk_map_pt2));
}
