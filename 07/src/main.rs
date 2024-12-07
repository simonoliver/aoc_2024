
fn concat (val0:i64,val1:i64) -> i64
{
    format!("{}{}",val0.to_string(),val1.to_string()).parse::<i64>().unwrap()
}


fn get_possible_output_values(current_total:i64, components: Vec<i64>) -> Vec<i64>
{

    let mut output_values:Vec<i64>=Vec::new();
    if components.len()==0
    {
        output_values.push(current_total);
    }
    else {
        // Calculate all possible values for remaining parts
        let next_component = components[0];
        let mut remaining_components = components.clone();
        remaining_components.remove(0);

        // Add
        let mut o1 = get_possible_output_values(current_total + next_component, remaining_components.clone());
        // Multiply
        let mut o2 = get_possible_output_values(current_total * next_component, remaining_components.clone());

        output_values.append(&mut o1);
        output_values.append(&mut o2);

        // Get list of all concat test values
        let mut concat_test_values = get_possible_output_values(next_component, remaining_components.clone());
        // Test concat with each of these as a potential value
        for concat_test in concat_test_values
        {
            output_values.push(concat(current_total,concat_test));
        }
    }
    output_values
}

fn test_valid_total(current_total:i64,target_total : i64, components: Vec<i64>, include_concat_test : bool) -> bool
{
    if components.len()==0 { return current_total==target_total;} // Reached end
    let next_component=components[0];
    let mut remaining_components=components.clone();
    remaining_components.remove(0);

    // Concat test
    if include_concat_test && remaining_components.len()>=1
    {
        // Test concatenation of next two component as well
        let successive_component=remaining_components[0];
        let mut remaining_components_one_removed=remaining_components.clone();
        remaining_components_one_removed.remove(0);
        // Test against concat component
        let concat_component=concat(next_component,successive_component);
        if test_valid_total(current_total+concat_component,target_total,remaining_components_one_removed.clone(),true)
            || test_valid_total(current_total*concat_component,target_total,remaining_components_one_removed.clone(),true)
        {
            return true;
        }
    }

    test_valid_total(current_total+next_component,target_total,remaining_components.clone(),include_concat_test)
     || test_valid_total(current_total*next_component,target_total,remaining_components.clone(),include_concat_test)

}

fn main() {
    println!("Hello, world!");
    let contents=std::fs::read_to_string("data/test_input").expect("Expected to read file");

    let input_data:Vec<(i64,Vec<i64>)>=contents.split("\n")
        .filter(|line|line.contains(":"))
        .map(|line_section|{
            let mut sections=line_section.split(":");
            let total=sections.next().unwrap().parse::<i64>().unwrap();
            let mut components=sections.next().unwrap().split(" ").filter(|component|component.len()>0);
            let parsed_components:Vec<i64>=components.map(|component|component.parse::<i64>().unwrap()).collect();
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
        if test_valid_total(0,entry.0,entry.1.clone(),false)
        {
            valid_solutions_test_sum+=entry.0;
        }
    }
    println!("Pt1 - Solution test value sum: {}",valid_solutions_test_sum);

    // pt2
    let mut valid_solutions_test_sum_concat=0;
    for (index,entry) in input_data.iter().enumerate()
    {
        let test_values=get_possible_output_values(0,entry.1.clone());
        if test_values.contains(&entry.0)
        {
            println!("Pass: {}",entry.0);
            valid_solutions_test_sum_concat+=entry.0;
        }
        else {
            println!("Fail: {}",entry.0);
        }
    }
    println!("Pt2 - Solution test concat included value sum: {}",valid_solutions_test_sum_concat);
}
