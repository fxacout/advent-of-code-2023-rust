use std::{vec, collections::HashMap};



#[derive(Debug)]
struct EngineMap {
    map: Vec<Vec<char>>,
    number_indexes: HashMap<(usize, usize), usize>,
    width: usize,
    height: usize,
}

pub fn execute() {
    let mut map = EngineMap::build_map();
    println!("Result Part 2:{:?}", map.get_sum());
}


#[derive(Debug)]
struct  GetResult {
    number: usize,
    indexes: Vec<usize>,
    height: usize,
}

impl EngineMap {
    fn build_map() -> EngineMap {
        let input = include_str!("input.txt");
        let map = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let height = map.len();
        let width = map[0].len();
        let number_indexes = HashMap::new();
        EngineMap { map, width, height, number_indexes }
    }



    fn get(&self, x: usize, y: usize) -> char {
        if x >= self.height {
            return '.'
        } else if y >= self.width {
            return '.'
        }
        self.map[x][y]
    }

    fn is_part(&self, x: usize, y: usize) -> bool {
        let x_range = if x == 0 {0..x+2} else {x-1..x+2};
        let y_range = if y == 0 {0..y+2} else {y-1..y+2};
        for i in x_range {
            for j in y_range.clone() {
               // adjacent fields are not alphanumeric nor .
                if self.get(i, j) == '*' {
                     return true
                }
            }
        }
        return false
    }
    fn get_number(&self, x: usize, y: usize) -> GetResult {
        let mut count = String::new();
        let mut is_valid: bool = false;
        for i in y..self.width {
            if self.get(x, i).is_alphanumeric() {
                count.push(self.get(x, i));
                is_valid = is_valid || self.is_part(x, i);
            } else {
                break;
            }
        }
        GetResult { 
            number: if is_valid {count.parse().unwrap()} else {0},
            indexes: (y..(y + count.len())).collect(),
            height: x,
        }
    }
    fn get_asterisc(&self, x: usize, y: usize) -> usize {
        let mut number_count: u8 = 0;
        let mut mult = 1;
        let x_range = if x == 0 {0..x+2} else {x-1..x+2};
        let y_range = if y == 0 {0..y+2} else {y-1..y+2};
        let mut used_numbers = vec![];
        println!("{:?}", self.number_indexes);
        for i in y_range {
            for j in x_range.clone() {
                if self.number_indexes.contains_key(&(j, i)) && !used_numbers.contains(self.number_indexes.get(&(j, i)).unwrap()) {
                    println!("Got number {:?} for index {:?} with number count {:?}",
                    self.number_indexes.get(&(j, i)).unwrap(),
                    (j, i),
                    number_count
                    );
                    number_count += 1;
                    used_numbers.push(self.number_indexes.get(&(j, i)).unwrap().clone());
                    mult *= self.number_indexes.get(&(j, i)).unwrap();
                }
            }
        }
        return if number_count == 2 {mult} else {0}
    }

    fn get_sum(&mut self) -> usize {
        let mut sum = 0;
        let mut skip_y = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if skip_y > 0 {
                    skip_y -= 1;
                } else if self.get(i, j).is_numeric() {
                    let result = self.get_number(i, j);
                    println!("{:?}", result);
                    skip_y = result.indexes.len() - 1;
                    result.indexes.iter().for_each(|index| {
                        if result.number > 0{ self.number_indexes.insert((result.height, index.clone()), result.number);}
                    });
                }
            }
        }
        for i in 0..self.height {
            for j in 0..self.width {
                if skip_y > 0 {
                    skip_y -= 1;
                } else if self.get(i, j) == '*' {
                    let result = self.get_asterisc(i, j);
                    println!("{:?}", result);
                    sum += result;
                }
            }
        }
        sum
    }
}     