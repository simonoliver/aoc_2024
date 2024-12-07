fn main() {
    println!("Hello, world!");
    let contents=std::fs::read_to_string("data/test_input").expect("Expected to read file");

    let input_data:Vec<(i32,Vec<i32>)>=contents.split("\n")
        .filter(|line|line.contains(":"))
        .map(|line_section|{
            let mut sections=line_section.split(":");
            let total=sections.next().unwrap().parse::<i32>().unwrap();
            let mut components=sections.next().unwrap().split(" ").filter(|component|component.len()>0);
            let parsed_components:Vec<i32>=components.map(|component|component.parse::<i32>().unwrap()).collect();
            (total,parsed_components)
        }).collect();

    println!("Entries found {}",input_data.len());
    for (index,entry) in input_data.iter().enumerate()
    {
        println!("Line {} total {} component count {}",index,entry.0,entry.1.len());
    }
}
