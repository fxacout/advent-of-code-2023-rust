use std::{collections::HashMap};

pub fn execute() {
    let lines = include_str!("input.txt").lines();
    let mut card_map: HashMap<usize, usize> = HashMap::new();
    (1..lines.clone().collect::<Vec<&str>>().len() + 1).for_each(|line| {
        card_map.insert(line, 1);
    });

    let result: usize = lines
    .map(|line| line.split(": ").collect::<Vec<&str>>().get(1).unwrap().to_owned().to_owned()
    )
    .enumerate()
    .map(|(index, line)| {
        let splitted: Vec<&str> = line.split("|").collect::<Vec<&str>>();
        let winning_numbers = splitted.get(0).unwrap().split(" "). map(
            |number| number.parse::<i32>().unwrap_or(-1)
        ).collect::<Vec<i32>>();
        let hits = splitted.get(1).unwrap().split(" ").fold(0, |acc, number| {
            if winning_numbers.contains(&number.parse::<i32>().unwrap_or(-2)) {
                acc + 1
            } else {
                acc
            }
        });
        if hits > 0 {
            (index+2..(index + hits+2)).for_each(|x| {
                card_map.insert(x, card_map.get(&(x)).unwrap() + card_map.get(&(index + 1)).unwrap());
            });
        }
        card_map.get(&(index + 1)).unwrap().clone()
    }
    ).sum();
    println!("Card Map: {:?}", card_map);
    println!("Result Part 2: {}", result);
}
