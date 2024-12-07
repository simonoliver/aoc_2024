
fn test_valid_total(current_total:i32,target_total : i32, components: Vec<i32>) -> bool
{
    if components.len()==0 { return current_total==target_total;} // Reached end
    let next_component=components[0];
    let mut remaining_components=components.clone();
    remaining_components.remove(0);
    test_valid_total(current_total+next_component,target_total,remaining_components.clone())
     || test_valid_total(current_total*next_component,target_total,remaining_components.clone())
}

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
    // Iterate
    let mut valid_solutions_test_sum=0;
    for (index,entry) in input_data.iter().enumerate()
    {
        if test_valid_total(0,entry.0,entry.1.clone())
        {
            valid_solutions_test_sum+=entry.0;
        }
    }
    println!("Solution test value sum: {}",valid_solutions_test_sum);

}
