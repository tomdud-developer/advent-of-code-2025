advent_of_code::solution!(2);


//
// I DONT KNOW WHAT IM DOING - RUST first time
//

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        for part in parts {
            let numbers: Vec<&str> = part.split('-').collect();
            if numbers.len() == 2 {
                let first: u64 = numbers[0].parse().ok()?;
                let second: u64 = numbers[1].parse().ok()?;
                for x in first..=second {
                    let x_str = x.to_string();
                    if ( x_str.len() % 2 == 0) {
                        let x_str_half_1 = &x_str[..x_str.len() / 2];
                        let x_str_half_2 = &x_str[x_str.len() / 2..];


                        println!("{}   {}", x_str_half_1, x_str_half_2);


                        if (x_str_half_1 == x_str_half_2) {
                            sum += x;
                        }
                    }
                }
            }
        }
    }
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        for part in parts {
            let numbers: Vec<&str> = part.split('-').collect();
            if numbers.len() == 2 {
                let first: u64 = numbers[0].parse().ok()?;
                let second: u64 = numbers[1].parse().ok()?;
                'outer: for x in first..=second {
                    let x_str = x.to_string();
                    let max_permute_len = x_str.len() / 2;
                    
                    for j in 1..=max_permute_len {
                        let substring = &x_str[0..j];
                        if x_str.len() % substring.len() == 0 && substring.repeat(x_str.len() / substring.len()) == x_str {
                            sum += x;
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
