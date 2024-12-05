use std::{fs, io};
use std::ops::Index;

fn get_order_rule(line:&str) -> Result<(i32, i32), std::num::ParseIntError>
{
    let mut splitter = line.splitn(2, "|");
    Ok(
        (splitter.next().unwrap().parse::<i32>()?,
        splitter.next().unwrap().parse::<i32>()?)
    )
}

fn get_int_vector(line:&str) -> Result<Vec<i32>, std::num::ParseIntError>
{
    let mut int_value_vector:Vec<i32>=Vec::new();
    let mut splitter = line.split(",");
    for value in splitter
    {
        let next_val=value.parse::<i32>()?;
        int_value_vector.push(next_val);
    }
    Ok(int_value_vector)
}

// Test a given update for its validity
fn test_update_validity(page_update: &Vec<i32>,rules:&Vec<(i32,i32)>) -> bool
{
    for (index,page_id) in page_update.iter().enumerate()
    {
        // Test against every rule
        for (rule_left,rule_right) in rules
        {
            // If the page matches the left side of the rule, then the second part must NOT come before it
            if page_id==rule_left
            {
                if page_update[0..index].contains(rule_right) {
                    return false;
                }
            }
            // If the page matches the right side of the rule, then the second part must NOT come before it
            else if page_id==rule_right
            {
                if page_update[index..].contains(rule_left) {
                    return false;
                }
            }
        }
    }
    // Passed (no rules failed)
    true
}



fn main()
{
    println!("Hello, world!");
    let contents=fs::read_to_string("data/test_input").expect("Should have been able to read file");

    let lines = contents.split("\n");

    let mut order_rules:Vec<(i32,i32)>=Vec::new();
    let mut page_updates:Vec<Vec<i32>>=Vec::new();

    for line in lines
    {
        if line.contains("|")
        {
            match get_order_rule(line)
            {
                Ok(order_rule) =>
                    {
                        order_rules.push(order_rule);
                    }
                Err(e) => println!("Error: {e}"),
            }
        }
        else if line.contains(",")
        {
            match get_int_vector(line)
            {
                Ok(page_update) =>
                    {
                        page_updates.push(page_update);
                    }
                Err(e) => println!("Error {e}"),
            }
        }
    }
    let rule_count=order_rules.len();
    let page_update_count=page_updates.len();
    println!("Rules Count {rule_count} page updates {page_update_count}");
    let mut middle_page_sum=0;
    for page_update in page_updates
    {
        if test_update_validity(&page_update,&order_rules)
        {
            // Append mid value
            middle_page_sum+=&page_update[(page_update.len()-1)/2];
        }
    }
    println!("Part 1 - Middle Page Sum {middle_page_sum}");
}
