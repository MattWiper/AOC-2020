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
        vals.push(process_pass(line));
    }

    let max = vals.iter().max(); 

    match max
    {
        Some(x) => println!("Highest Seat ID: {}", x), 
        None => println!("Error, no Seat ID's found")
    }

    let mut positions = Vec::new(); 
    for val in vals
    {
        positions.push(seat_pos(pos));
    }

    positions.sort_by_key(|&(k, y)| (k, y));
    let first = positions.first().unwrap(); 
    let last = positions.last().unwrap(); 

    let i = first.0 + 1; 
    let y = last.0 - 1; 

    for x in i..y
    {
        let temp: Vec<_> = positions.iter().filter(|&&z| z.0 == x).collect();

        if temp.len() != 8
        {
            let indexes = [0, 1, 2, 3, 4, 5, 6, 7];
            let difference: Vec<_> = indexes.iter().filter(|item| !temp.iter().any(|q| q.1 == **item)).collect();

            if difference.len() == 1
            {
                println!("Seat ID: {}", (x * 8) + difference[0]); 
            }
            else 
            {
                println!("ERROR");
            }
        }
    }
}

fn process_pass(test: String) -> i32
{
    let mut coord = (0, 127);
    let mut coord_2 = (0, 7);
    let mut row = 0; 
    let mut col = 0; 

    for i in 0..6
    {
        coord = part(coord.0, coord.1, test.chars().nth(i).unwrap());
    }

    match test.chars().nth(6)
    {
        Some(x) => if x == 'F' { col = coord.0 } else { col = coord.1 }, 
        None => (), 
    }

    for i in 7..test.len() - 1
    {
        coord_2 = part(coord_2.0, coord_2.1, test.chars().nth(i).unwrap());
    }

    match test.chars().nth(6)
    {
        Some(x) => if x == 'F' { row = coord.0 } else { row = coord.1 }, 
        None => (), 
    }

    match test.chars().nth(9)
    {
        Some(x) => if x == 'L' { col = coord_2.0 } else { col = coord_2.1 }, 
        None => (), 
    }

    (row * 8) + col
}

fn part(lo: i32, hi: i32, ch: char) -> (i32, i32)
{
    let mid: i32 = ((hi - lo) / 2) + lo;

    if ch == 'F' || ch == 'L'
    {
        if lo % 2 == 0
        {
            return (lo, mid);
        }

        return (lo + 1, mid)
    }
    else if ch == 'B' || ch == 'R'
    {
        if mid % 2 != 0
        {
            return (mid + 1, hi)
        }

        return (mid, hi)
    }

    (0, 0)
}

fn seat_pos(id: i32) -> (i32, i32)
{
    let col = id % 8; 
    let row = (id - col) / 8;

    (row, col)
} 