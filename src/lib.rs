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

pub fn day1() {

}
