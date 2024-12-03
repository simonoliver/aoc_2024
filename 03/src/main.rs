use std::fs;
use std::num::ParseIntError;
use regex::Regex;


fn main() -> Result<(),ParseIntError>{

    let contents = fs::read_to_string("data/input")
        .expect("Should have been able to read the file");

    // Basic Regex
    // mul\(([0-9]+),([0-9]+)\)

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results = vec![];
    for (_, [mul0, mul1]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push((mul0.parse::<i32>()?, mul1.parse::<i32>()?));
    }

    let mut total_mul:i32=0;
    for result in results.iter()
    {
        let (r0,r1)=result;
        total_mul+=r0*r1;
        println!("Multiplying found {r0}x{r1}");
    }

    let matches_found=results.iter().count();
    println!("Matches found {matches_found} total {total_mul}");

    Ok(())
}
