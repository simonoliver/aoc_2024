fn concat (val0:i64,val1:i64) -> i64 { format!("{}{}",val0.to_string(),val1.to_string()).parse::<i64>().unwrap() }

fn test_valid_total(current_total:i64,target_total : i64, components: Vec<i64>, include_concat_test : bool) -> bool
{
    if components.is_empty() { return current_total==target_total;} // Reached end
    let next_component=components[0];
    let remaining_components:Vec<i64>=components[1..].to_vec();

    test_valid_total(current_total+next_component,target_total,remaining_components.clone(),include_concat_test)
     || test_valid_total(current_total*next_component,target_total,remaining_components.clone(),include_concat_test)
     || include_concat_test && test_valid_total(concat(current_total,next_component),target_total,remaining_components.clone(),true)
}

fn main() {
    let contents=std::fs::read_to_string("data/input").expect("Expected to read file");

    let input_data:Vec<(i64,Vec<i64>)>=contents.split("\n")
        .filter(|line|line.contains(":"))
        .map(|line_section|{
            let mut sections=line_section.split(":");
            let total=sections.next().unwrap().parse::<i64>().unwrap();
            let components=sections.next().unwrap().split(" ").filter(|component|component.len()>0);
            let parsed_components:Vec<i64>=components.map(|component|component.parse::<i64>().unwrap()).collect();
            (total,parsed_components)
        }).collect();

    let valid_solutions_test_sum_concat:i64=input_data.iter()
        .map(|entry| if test_valid_total(0,entry.0,entry.1.clone(),false)==true {entry.0} else {0}).sum();
    println!("Pt1 - Solution test value sum: {}",valid_solutions_test_sum_concat);

    let valid_solutions_test_sum_concat:i64=input_data.iter()
        .map(|entry| if test_valid_total(0,entry.0,entry.1.clone(),true)==true {entry.0} else {0}).sum();
    println!("Pt2 - Solution test concat included value sum: {}",valid_solutions_test_sum_concat);
}