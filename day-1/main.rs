use std::fs::File; 
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() 
{
    let filename = "./input";
    let file = File::open(filename).unwrap(); 
    let reader = BufReader::new(file);
    let mut vals = Vec::new(); 

    for(_index, line) in reader.lines().enumerate()
    {
        let line = line.unwrap(); 
        vals.push(line.parse::<i32>().unwrap());
    }

    let result = dual_sum(vals, 2020);

    match result
    {
        Some(x) => println!("Result: {}", x.0 * x.1), 
        None => println!("No pairs found"),
    }
    // let mult_result = result.0 * result.1;
    // println!("Result: {}", mult_result);
}

fn dual_sum(vals: Vec<i32>, target: i32) -> Option<(i32, i32)>
{
    let mut map = HashSet::<i32>::new(); 
    for val in vals.iter()
    {
        let ele = target - val; 

        if map.contains(&(ele))
        {
            return Some((*val, ele));
        }

        map.insert(*val);
    }

    None
}
