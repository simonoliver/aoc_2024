use std::fs;

fn main() {
    //solve("data/input");
    solve("data/test_input");
}

fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
}
