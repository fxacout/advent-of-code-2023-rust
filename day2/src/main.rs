use std::os::fd;

use regex::Regex;

fn part_1() {
    let red: usize = 12;
    let green =  13;
    let blue = 14;
    // Load txt an iterate over lines
    let input = include_str!("input.txt");
    let result: usize = input.lines().enumerate().map(|(index,  line)| {
        for subset in line.split(";") {
            // Regex match following: 1 blue
            for item in subset.split(",") {
                let re = Regex::new(r"(\d+) (\w+)").unwrap();
                let caps = re.captures(item).unwrap();
                let count: usize = caps[1].parse().unwrap();
                let color = &caps[2];
                match color {
                    "red" => {if count > red {return 0}},
                    "green" => {if count > green {return 0}},
                    "blue" => {if count > blue {return 0}},
                    _ => panic!("Unknown color: {}", color)
                    
                }
            }
        }
        return index + 1;
    }).sum();
    println!("Part 1: {}", result);
}

fn part_2() {
    let red: usize = 12;
    let green =  13;
    let blue = 14;
    // Load txt an iterate over lines
    let input = include_str!("input.txt");
    let result: usize = input.lines().enumerate().map(|(index,  line)| {
        let mut  max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
        for subset in line.split(";") {
            for item in subset.split(",") {
                let re = Regex::new(r"(\d+) (\w+)").unwrap();
                let caps = re.captures(item).unwrap();
                let count: usize = caps[1].parse().unwrap();
                let color = &caps[2];
                match color {
                    "red" => {max_red = std::cmp::max(max_red, count)},
                    "green" => {max_green = std::cmp::max(max_green, count)},
                    "blue" => {max_blue = std::cmp::max(max_blue, count)},
                    _ => panic!("Unknown color: {}", color)
                    
                }
            }
            
        }
        return max_red * max_green * max_blue;
    }).sum();
    println!("Part 2: {}", result);
}


fn main() {
    part_1();
    part_2();
    
}
