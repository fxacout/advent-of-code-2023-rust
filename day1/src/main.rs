use regex::Regex;


fn main() {
    // Load txt an iterate over lines
    let input = include_str!("input.txt");
    let result = input
        .lines()
        .map(|x| { // Part 2
            let numbers_strings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            let mut new_string = x.to_string();
            let mut chars_consumed = 0;
            let mut built_string = String::new();
            new_string.clone().char_indices().for_each(|(charIndex, c)| {
                for (i, number) in numbers_strings.iter().enumerate() {
                    if chars_consumed > 0 {
                        chars_consumed -= 1;
                        continue;
                    }
                    // replace at start of string if number
                    let regex_pattern = format!("^{}", regex::escape(number));
                    let regex = Regex::new(&regex_pattern).unwrap();
                    let sliced_string = x.split_at(charIndex).1.to_owned();
                    if regex.is_match(sliced_string.as_str()) {
                        built_string = format!("{}{}", built_string, regex.replace(sliced_string.as_str(), &format!("{}", i + 1)).to_string());
                        chars_consumed += number.len() - 1;
                    } else {
                        built_string = format!("{}{}", built_string, c);
                    }
                }
            });
            built_string
        })
        .map(|x| { // Part 1
            let numbers = x.chars().fold(String::new(), |mut acc, c| {
                if c.is_numeric() {
                    acc.push(c);
                }
                acc
            });
            String::from(format!(
                "{}{}",
                numbers.chars().nth(0).unwrap(),
                numbers.chars().last().unwrap()
            ))
            .parse::<i32>()
            .unwrap()
        })
        .sum::<i32>();
    println!("Result: {}", result);
}
