use std::fs;
use std::num::ParseIntError;

fn parse_int_line(in_string: &str) -> Result<Vec<i32>, ParseIntError> {
    let splitter = in_string.split(" ");
    let mut parsed_values=Vec::new();

    for split_string in splitter.into_iter()
    {
        parsed_values.push(split_string.parse::<i32>()?);
    }

    Ok(parsed_values)
}

fn test_report_valid(value_list:&Vec<i32>) -> bool
{
    let mut positive_increments=0;
    let mut negative_increments=0;

    let mut index=0;
    let mut last_value=0;
    for report_value in value_list.iter()
    {
        if index>0
        {
            let increment=report_value-last_value;
            if increment>0 {positive_increments+=1}
            else if increment<0 {negative_increments+=1}
            else {return false};// Need at least 1 increment
            if increment.abs()>3 {return false}; // Max 3 increment
        }
        last_value=*report_value;
        index+=1;
    }
    // Either positive or negative increments must be zero
    return positive_increments==0 || negative_increments==0;
}


fn main() {

    let contents = fs::read_to_string("data/input")
        .expect("Should have been able to read the file");
    let lines = contents.split("\n");

    // First part - safe report count
    let mut safe_reports_count=0;
    let mut test_count=0;
    for line in lines.clone()
    {
        let result_values=parse_int_line(line);
        match result_values
        {
            Ok(values) =>
                {
                    if test_report_valid(&values)
                    {
                        safe_reports_count+=1;
                    }
                    test_count+=1;
                },
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("Safe reports (unmodified) {safe_reports_count}/{test_count}");

    // Second level - allow one removal
    safe_reports_count=0;
    for line in lines.clone()
    {
        let result_values = parse_int_line(line);

        match result_values
        {
            Ok(values) =>
                {
                    let mut test_passed=false;
                    let value_count=values.iter().count();
                    if test_report_valid(&values)
                    {
                       // Valid without changes
                        test_passed=true;
                    } else {
                       let mut n=0;
                        // Remove a single value at each point and test
                        while n<value_count && !test_passed
                        {
                            // Clone and Remove a value at index x
                            let mut cloned_values=values.clone();
                            cloned_values.remove(n);
                            if test_report_valid(&cloned_values)
                            {
                                test_passed=true;
                            }
                            n+=1;
                        }
                    }
                    // If one of these passed
                    if test_passed {safe_reports_count+=1};
                },
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Valid reports (one value removal) {safe_reports_count}/{test_count}");

}
