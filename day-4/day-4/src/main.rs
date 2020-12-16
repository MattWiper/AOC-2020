use std::fs;
use std::collections::HashMap;

fn main()
{    
    let data = fs::read_to_string("../../day-4/src/input").expect("failed to read file"); 
    let mut v = Vec::<String>::new(); 
    let mut temp = String::new();
    let mut active = false; 
    let mut objs = Vec::new(); 

    data.lines().for_each(|x| 
        if !x.trim().is_empty() 
        {

            active = true;  
            temp.push_str(&x.clone());
            temp.push_str(" ");
        }
        else 
        {
            active = false;
            if  temp.len() > 0
            {
                v.push(temp.clone());
                temp.clear();
            }
        });

    if active && temp.len() > 0
    {
        v.push(temp.clone());
    }

    for dat in v
    {

        let mut map = HashMap::new(); 

        let k = dat.trim().split(" "); 
        for ele in k
        {
            let field = ele.split(":"); 
            let mapped: Vec<&str> = field.collect(); 

            map.insert(mapped[0].to_string(), mapped[1].to_string());            
        }

        objs.push(map);
    }

    let mut valid_passports = 0; 
    let mut valid_passports_2 =  0;

    for passport in &objs
    {
        if validate_passport(passport.clone())
        {
            valid_passports += 1; 
        }

        if validate_passport_2(passport.clone()) 
        {
            valid_passports_2 += 1
        }
    }

    println!("\n\nRESULTS\n\nPart 1");
    println!("Valid Passports: {}. Invalid: {}\n\nPart 2", valid_passports, objs.len() - valid_passports);
    println!("Valid Passports 2: {}. Invalid: {}", valid_passports_2, objs.len() - valid_passports_2);
}

fn validate_passport_2(passport: HashMap<String, String>) -> bool
{
    let dat_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let eye_colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let clr = ['a', 'b', 'c', 'd', 'e', 'f'];

    let mut errors = HashMap::new(); 

    for field in dat_fields.iter()
    {
        let mut res = true;

        if !passport.contains_key(&field.to_string())
        {
            errors.insert(field.clone(), "Missing");
            break; 
        }

        match *field
        {
            "byr" => res =  passport["byr"].parse::<i32>().unwrap() >= 1920 && passport["byr"].parse::<i32>().unwrap() <= 2002,
            "iyr" => res =  passport["iyr"].parse::<i32>().unwrap() >= 2010 && passport["iyr"].parse::<i32>().unwrap() <= 2020,
            "eyr" => res =  passport["eyr"].parse::<i32>().unwrap() >= 2020 && passport["eyr"].parse::<i32>().unwrap() <= 2030,
            "hgt" => 
            {
  
                if passport["hgt"].contains("in")
                {
                    let sub: i32 = passport["hgt"].to_string()[..passport["hgt"].len() - 2].parse::<i32>().unwrap();
                    res = sub >= 59 && sub <= 76
                }
                else if passport["hgt"].contains("cm")
                {
                    let sub: i32 = passport["hgt"].to_string()[..passport["hgt"].len() - 2].parse::<i32>().unwrap();
                    res = sub >= 150 && sub <= 193
                }
                else
                {
                    res = false; 
                }
            }, 
            "hcl" => res = passport["hcl"].starts_with("#") && passport["hcl"].len() == 7 && passport["hcl"].to_string()[ 1..].chars().all(|o| (o.is_alphabetic() && clr.contains(&o)) || o.is_numeric()), 
            "ecl" => res = eye_colours.contains(&&passport["ecl"].to_string()[..]), 
            "pid" => res = passport["pid"].len() == 9 && passport["pid"].chars().all(char::is_numeric), 
            _ => res = false
        };

        if !res
        {
            errors.insert(field.clone(), passport.get::<str>(field).unwrap());
        }
    }

    if errors.len() > 0
    {
        println!("VALIDATION ERROR: {:?}", errors);
    }

    return errors.len() == 0;
}

fn validate_passport(passport: HashMap<String, String>) -> bool
{
    let dat_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut res = true;
    for field in dat_fields.iter()
    {
        if !passport.contains_key(&field.to_string())
        {
            res = false; 
            break; 
        }
    }

    return res
}