use std::fs::File; 
use std::io::{BufRead, BufReader};

fn main()
{
    let filename = "./input";
    let file = File::open(filename).unwrap(); 
    let reader = BufReader::new(file);
    let mut vals = Vec::new(); 

    for(_index, line) in reader.lines().enumerate()
    {
        let line = line.unwrap(); 
        vals.push(line);
    }

    slopes_pt_1(&vals);
    slopes_pt_2(&vals);
}

fn slopes_pt_1(dat: &Vec<String>)
{
    println!("PART 1\n");

    let slope_1 = process_slope(3, 1, dat.clone());

    println!("Tree count: {}", slope_1)
}

fn slopes_pt_2(dat: &Vec<String>)
{
    println!("\nPART 2\n");
    
    let slope_1 = process_slope(1, 1, dat.clone());
    println!("Slope 1: {}", slope_1);

    let slope_2 = process_slope(3, 1, dat.clone()); 
    println!("Slope 2: {}", slope_2);
   
    let slope_3 = process_slope(5, 1, dat.clone());
    println!("Slope 3: {}", slope_3);

    let slope_4 = process_slope(7, 1, dat.clone());
    println!("Slope 4: {}", slope_4);

    let slope_5 = process_slope(1, 2, dat.clone());
    println!("Slope 5: {}", slope_5);

    let result: u128 = slope_1 * slope_2 * slope_3 * slope_4 * slope_5;

    println!("Result: {}", result);
}

fn process_slope(dx: u32, dy: u32, mut dat: Vec<String>) -> u128
{
    let mut count = 0; 
    let mut coords = (0, 0, false);
    let mut tree_count = 0; 

    while count < dat.len()
    {
        if coords.1 >= dat.len() as u32
        {
            break; 
        }

        coords = step(coords.0, coords.1 as u32, dx, dy, &dat);

        if coords.2
        {
            tree_count += 1; 
        }

        if coords.0 >= dat[0].len() as u32
        {
            // need to add another pattern
            for x in 0..dat.len()
            {
                let mut owned: String = dat[x].to_owned(); 
                owned += &owned.clone(); 
                dat[x] = owned;
            }
        }

        count += 1; 
    }

    tree_count
}

fn step(x: u32, y: u32, dx: u32, dy: u32, dat: &Vec<String>) -> (u32, u32, bool)
{
    if dat[y as usize].chars().nth(x as usize).unwrap() == '#'
    {
        return (x + dx, y + dy, true);
    }

    (x + dx, y + dy, false)
}