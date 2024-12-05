use std::fs;

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
    let splitter = line.split(",");
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


fn repair_first_rule_page_updates(page_update: &mut Vec<i32>,rules:&Vec<(i32,i32)>) -> bool
{
    // We need to use iter_mut as will be modifying these values
    //for (index,page_id) in page_update.iter_mut().enumerate()

    for index in 1..page_update.len()
    {
        let page_id=page_update[index];
        // Test against every rule
        for (rule_left,rule_right) in rules
        {
            // Currently the indices are wrong on replace_index

            // If the page matches the left side of the rule, then the second part must NOT come before it
            if page_id==*rule_left
            {
                if page_update[0..index].contains(rule_right) {
                    // Swap
                    let replace_index= page_update.iter().position(|&r| r==*rule_right).unwrap();
                    page_update[replace_index]=*rule_left;
                    page_update[index]=*rule_right;
                    //let csv_string=page_update.iter().map(|x| x.to_string()+",").collect::<String>();
                    //println!("L: Swapped index {replace_index}({rule_right}), with {index}({rule_left}) now {csv_string}");
                    return true;
                }
            }
            // If the page matches the right side of the rule, then the second part must NOT come before it
            else if page_id==*rule_right
            {
                if page_update[index..].contains(rule_left) {
                    let replace_index= page_update.iter().position(|&r| r==*rule_left).unwrap();
                    page_update[replace_index]=*rule_right;
                    page_update[index]=*rule_left;
                    //let csv_string=page_update.iter().map(|x| x.to_string()+",").collect::<String>();
                    //println!("R: Swapped index {replace_index}({rule_left}), with {index}({rule_right}) - Now {csv_string}");
                    return true;
                }
            }
        }
    }

    false

}


fn main()
{
    let contents=fs::read_to_string("data/input").expect("Should have been able to read file");

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
    for page_update in &page_updates
    {
        if test_update_validity(&page_update,&order_rules)
        {
            // Append mid value
            middle_page_sum+=&page_update[(page_update.len()-1)/2];
        }
    }
    println!("Part 1 - Middle Page Sum {middle_page_sum}");

    // Part 2 - Fix up the data
    middle_page_sum=0; // Reset sum
    for mut page_update in page_updates
    {
        let mut was_repaired=false;
        // one at a time
        while repair_first_rule_page_updates(&mut page_update,&order_rules)
        {
            was_repaired=true;
        }
        if was_repaired
        {
            // Add the middle page number after this update
            middle_page_sum += &page_update[(page_update.len() - 1) / 2];
            //let csv_string=page_update.iter().map(|x| x.to_string()+",").collect::<String>();
            //println!("Fixed line {csv_string}");
        }
    }
    println!("Part 2 - Middle Page Sum {middle_page_sum}");

}

