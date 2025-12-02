advent_of_code::solution!(1);

//
// I DONT KNOW WHAT IM DOING - RUST first time
//

struct Clock {
    pub position: i64,
    pub equals_0_counter: i64,
}

struct Clock2 {
    pub position: i64,
    pub equals_0_counter: i64,
    pub previous_position: i64,
}

impl Clock {
    pub fn new(position: i64) -> Self {
        Clock { position, equals_0_counter: 0 }
    }
    
    pub fn rorate(&mut self, command : &str) {
        let direction = &command[0..1];
        let amount: i64 = command[1..].parse().unwrap();
        
        self.position = match direction {
            "L" => self.position - amount,
            "R" => self.position + amount,
            _ => self.position,
        };
        if self.position % 100 == 0 {
            self.equals_0_counter += 1;
        }

        println!("New position: {} after roration {}", self.position, command);
    }
    
    pub fn into_position(self) -> i64 {
        self.position
    }
}

impl Clock2 {
    pub fn new(position: i64) -> Self {
        Clock2 { position, equals_0_counter: 0, previous_position: position }
    }
    
    pub fn rorate(&mut self, command : &str) {
        let directionStr = &command[0..1];
        let amount: i64 = command[1..].parse().unwrap();
        
        let direction: i64 = match directionStr {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };

        // loop
        for _ in 1..=amount {
            self.position += direction;
            if (self.position == -1) {
                self.position = 99;
            }
            if self.position == 100 || self.position == 0 {
                self.position = 0;
                self.equals_0_counter += 1;
            }
        }

        println!("New position: {} after roration {}", self.position, command);
    }
    
    pub fn into_position(self) -> i64 {
        self.position
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut clock = Clock::new(50);

    for line in input.lines() {
        let command = line.trim();
        if !command.is_empty() {
            clock.rorate(command);
        }
    }

    Some(clock.equals_0_counter as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
        let mut clock = Clock2::new(50);

    for line in input.lines() {
        let command = line.trim();
        if !command.is_empty() {
            clock.rorate(command);
        }
    }

    Some(clock.equals_0_counter as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
