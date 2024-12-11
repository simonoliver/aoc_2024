use std::fs;

fn process_stones(stone_data : &mut Vec<i64>) {
    let mut process_index=0;
    let mut reached_end=false;
    while !reached_end {
        let stone_value=stone_data[process_index];
        if stone_value==0 {stone_data[process_index]=1;}
        else if stone_value.to_string().len()%2==0 {
            let stone_value_string=stone_value.to_string();
            let (stone_string_left,stone_string_right)=stone_value_string.split_at(stone_value_string.len()/2);
            stone_data[process_index]=stone_string_left.parse::<i64>().unwrap();
            process_index+=1;
            stone_data.insert(process_index,stone_string_right.parse::<i64>().unwrap());
        }
        else {stone_data[process_index]*=2024};
        process_index+=1;
        if process_index>=stone_data.len() {reached_end=true;}
    }
}


fn main() {
    let content=fs::read_to_string("data/input").expect("Expected to read the file");
    let mut stone_data:Vec<i64>=content.split("\n").next().unwrap().split(" ")
        .filter(|value_string| match value_string.parse::<i64>() {
            Ok(_) => {true}
            Err(_) => {false}
        })
        .map(|value_string|
            value_string.parse::<i64>().unwrap()
        ).collect();

    for _ in 0..25 {
        process_stones(&mut stone_data)
    }
    println!("Pt1 - Stone count {}",stone_data.len());
    for loop_index in 0..50 {
        println!("Loop index {loop_index}");
        process_stones(&mut stone_data)
    }
    println!("Pt2 - Stone count {}",stone_data.len());

}
