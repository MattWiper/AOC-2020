use std::fs::File; 
use std::io::{BufRead, BufReader};
use std::convert::TryInto; 

fn main()
{
    let filename = "./input";
    let file = File::open(filename).unwrap(); 
    let reader = BufReader::new(file);

    let mut count = 0; 
    let mut val_pwd_1 = 0; 
    let mut val_pwd_2 = 0; 

    for(_index, line) in reader.lines().enumerate()
    {
        let line = line.unwrap(); 

        if validate_pwd_1(&line.clone())
        {
            val_pwd_1 += 1; 
        }

        if validate_pwd_2(&line.clone())
        {
            val_pwd_2 += 1; 
        }

        count = count + 1; 
    }

    println!("\nPART 1\n");
    println!("Valid passwords: {}\nInvalid passwords: {}", val_pwd_1, count - val_pwd_1); 
    println!("\nPART 2\n");
    println!("Valid passwords: {}\nInvalid passwords: {}", val_pwd_2, count - val_pwd_2); 
}

fn validate_pwd_1(pwd: &String) -> bool
{
    let a: Vec<&str> = pwd.split_whitespace().collect();

    if a.len() == 3
    {
        let count: Vec<&str> = a[0].split("-").collect();
        let mut min = -1; 
        let mut max = -1; 

        if count.len() == 2
        {
            min = count[0].parse::<i32>().unwrap(); 
            max = count[1].parse::<i32>().unwrap(); 
        }

        let character = a[1].split(':').take(1).next().unwrap_or("");

        let char_count = a[2].matches(character).count(); 

        if char_count >= min.try_into().unwrap() && char_count <= max.try_into().unwrap()
        {
            return true;
        }
    }

    false
}

fn validate_pwd_2(pwd: &String) -> bool
{
    let a: Vec<&str> = pwd.split_whitespace().collect();

    if a.len() == 3
    {
        let count: Vec<&str> = a[0].split("-").collect();
        let mut pos1 = -1; 
        let mut pos2 = -1; 

        if count.len() == 2
        {
            pos1 = count[0].parse::<i32>().unwrap(); 
            pos2 = count[1].parse::<i32>().unwrap(); 
        }

        let character = a[1].split(':').take(1).next().unwrap_or("");

        pos1 -= 1; 
        pos2 -= 1; 

        if a[2].len() > pos1 as usize && a[2].len() > pos2 as usize
        {
            let v = a[2].chars().nth(pos1 as usize).unwrap();
            let x = a[2].chars().nth(pos2 as usize).unwrap();

            if (v.to_string() == character && x.to_string() != character) 
                || (v.to_string() != character && x.to_string() == character)
            {
                return true; 
            }
        }
    }

    false
}