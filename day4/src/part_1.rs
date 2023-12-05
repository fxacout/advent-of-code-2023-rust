pub fn execute() {
    let lines = include_str!("input.txt").lines();
    let result: i32 = lines
    .map(|line| line.split(": ").collect::<Vec<&str>>().get(1).unwrap().to_owned().to_owned()
    )
    .map(|line| {
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
        return match hits {
            0 => 0,
            n => i32::pow(2, n-1),
            _ => panic!("Should not happen")
        }
    }
    ).sum();
    println!("Result: {}", result);
}
