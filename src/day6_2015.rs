pub fn part1(contents: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let example: String = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500".to_string();
        assert_eq!(998996, part1(example));
    }
}
