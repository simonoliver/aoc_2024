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

    // Part 2 - Also check for do/don't

    let re2 = Regex::new(r"(mul)\(([0-9]+),([0-9]+)\)|(do)\(\)|(don't)\(\)").unwrap();

    let mut add_enabled=true;
    total_mul=0;
    for capture in re2.captures_iter(&contents)
    {
        // Order will be (match,mul,arg0,arg1,do,dont)
        let mul=capture.get(1).map_or("", |m| m.as_str());
        let arg0=capture.get(2).map_or("", |m| m.as_str());
        let arg1=capture.get(3).map_or("", |m| m.as_str());
        let inst_do=capture.get(4).map_or("", |m| m.as_str());
        let inst_dont=capture.get(5).map_or("", |m| m.as_str());

        if mul.len()>0 && add_enabled
        {
            let arg0int=arg0.parse::<i32>().unwrap();
            let arg1int=arg1.parse::<i32>().unwrap();
            total_mul+=arg0int*arg1int;
        }
        else if inst_do.len()>0 {add_enabled=true;}
        else if inst_dont.len()>0 {add_enabled=false;}
    }

    println!("Pt2 total {total_mul}");

    Ok(())
}
