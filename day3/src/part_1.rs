

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

pub fn execute() {
    let map = Map::build_map();
    println!("Result Part 1: {:?}", map.get_sum());
}


#[derive(Debug)]
struct  GetResult {
    number: usize,
    len: usize,
}

impl Map {
    fn build_map() -> Map {
        let input = include_str!("input.txt");
        let map = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let height = map.len();
        let width = map[0].len();
        Map { map, width, height }
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
                if !self.get(i, j).is_numeric() && self.get(i, j) != '.' {
                     return true
                }
            }
        }
        return false
    }
    fn get_number(&self, x: usize, y: usize) -> GetResult {
        let mut count = String::new();
        let mut isValid: bool = false;
        for i in y..self.width {
            if self.get(x, i).is_alphanumeric() {
                count.push(self.get(x, i));
                isValid = isValid || self.is_part(x, i);
            } else {
                break;
            }
        }
        GetResult { 
            number: if isValid {count.parse().unwrap()} else {0},
            len: count.len()
        }
    }

    fn get_sum(&self) -> usize {
        let mut sum = 0;
        let mut skip_y = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if skip_y > 0 {
                    skip_y -= 1;
                } else if self.get(i, j).is_numeric() {
                    let result = self.get_number(i, j);
                    println!("{:?}", result);
                    sum += result.number;
                    skip_y = result.len;
                }
            }
        }
        sum
    }
} 