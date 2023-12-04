use std::fs;
use std::error::Error;

pub struct AdventDay {
    pub number: i32,
}

impl AdventDay {
    pub fn build(args: &[String]) -> Result<AdventDay, &'static str> {
        if args.len() < 2 {
            return Err("not enough args");
        }

        let day = args[1].clone().parse::<i32>().unwrap();

        if day < 0 || day > 1 {
            return Err("day not implemented");
        }

        Ok(AdventDay { number: day, })
    }
}

pub fn parse_file_to_string(file_name: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_name)?;

    Ok(contents)
}

pub fn day1(contents: String) -> i32 {
    let mut sum: i32 = 0;
    
    for line in contents.lines() {
        let mut first_char: Option<char> = None;
        let mut second_char: Option<char> = None;

        for c in line.chars() {
            if c.is_digit(10) {
                // handle first digit
                if first_char.is_none() {
                    first_char = Some(c);
                } else {
                    // handle last digit - continuously update
                    second_char = Some(c);
                }  
            }
        }

        // handle case where no second digit is present
        if second_char.is_none() {
            second_char = Some(first_char.unwrap());
        }

        // create 2-digit number and add to running sum
        let mut line_num: i32 = 0;
        if first_char.is_some() && second_char.is_some() {
            let v: Vec<Option<char>> = vec![first_char, second_char];
            let filtered = v.iter().flatten().collect::<Vec<_>>();
            let str = String::from_iter(filtered);
            line_num = str.parse().unwrap();
        }

        sum += line_num;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_test() {
        let example: String = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".to_string();
        assert_eq!(142, day1(example));
    }
}
