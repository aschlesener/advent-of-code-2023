enum SwitchLight {
    On,
    Off,
    Toggle
}

type Grid = Vec<[i32; 1000]>;
struct Point {
    x: i32,
    y: i32,
}
struct Rectangle {
    start: Point,
    end: Point,
}

pub fn part1(contents: &str, part2: bool) -> i32 {
    // Create 1000x1000 grid of lights, defaulted to Off
    let mut grid = vec![[0; 1000]; 1000];
    
    for line in contents.lines() {
        let (instruction, coord_start) = parse_instructions(line);

        // Parse coordinates, e.g. "499,499 through 500,500"
        let (_, coordinates) = line.split_at(coord_start);
        let rect = parse_coords(coordinates);

        // Apply instruction to grid
        if part2 {
            // part2 is the same, except the instruction rules are slightly different
            apply_instructions(&mut grid, &instruction, rect, true);
        } else {
            apply_instructions(&mut grid, &instruction, rect, false);
        }
    }

    // Check how many lights are lit
    return count_lights(&grid);
}

fn parse_instructions(line: &str) -> (SwitchLight, usize) {
    let instruction: SwitchLight;
    let coord_start;

    // Parse instruction, e.g. "turn off 499,499 through 500,500"
    if line.contains("turn on") {
        instruction = SwitchLight::On;
        coord_start = 8;
    } else if line.contains("turn of") {
        instruction = SwitchLight::Off;
        coord_start = 9;
    } else if line.contains("toggle") {
        instruction = SwitchLight::Toggle;
        coord_start = 7;
    } else {
        panic!("Instructions must include turn off, turn on, or toggle");
    }

    return (instruction, coord_start)
}

fn parse_coords(coordinates: &str) -> Rectangle {
    let coords: Vec<&str> = coordinates.split(" through ").collect();
    let start = *coords.get(0).unwrap(); // e.g. 499,499
    let end = *coords.get(1).unwrap(); // e.g. 500,500

    let start_points: Vec<&str> = start.split(",").collect();
    let start_x: i32 = start_points.get(0).unwrap().parse::<i32>().unwrap();
    let start_y: i32 = start_points.get(1).unwrap().parse::<i32>().unwrap();

    let end_points: Vec<&str> = end.split(",").collect();
    let end_x: i32 = end_points.get(0).unwrap().parse::<i32>().unwrap();
    let end_y: i32 = end_points.get(1).unwrap().parse::<i32>().unwrap();

    let start_point: Point = Point { x: start_x, y: start_y };
    let end_point: Point = Point { x: end_x, y: end_y };

    let rect: Rectangle = Rectangle{start: start_point, end: end_point};

    rect
}

fn apply_instructions(grid: &mut Grid, instruction: &SwitchLight, rect: Rectangle, part2: bool) {
    for x in rect.start.x..(rect.end.x + 1) {
        for y in rect.start.y..(rect.end.y + 1) {
            let value = grid[x as usize][y as usize];

            match instruction {
                SwitchLight::On => { 
                    if part2 {
                        grid[x as usize][y as usize] += 1;
                    } else {
                        grid[x as usize][y as usize] = 1;
                    }
                }
                SwitchLight::Off => {
                    if part2 {
                        if value > 0 {
                            grid[x as usize][y as usize] -= 1;
                        }                        
                    } else {
                        grid[x as usize][y as usize] = 0;
                    }
                }
                SwitchLight::Toggle=> {
                    if part2 {
                        grid[x as usize][y as usize] += 2;
                    } else {
                        if value == 1 {
                            grid[x as usize][y as usize] = 0;
                        } else if value == 0 {
                            grid[x as usize][y as usize] = 1;
                        }
                    }
                }
            } 
        }
    }
}

fn count_lights(grid: &Grid) -> i32 {
    let mut lights_on = 0;

    for row in grid.iter() {
        for value in row.iter() {
                lights_on += value;
        }
    }

    lights_on
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let example: String = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500".to_string();
        assert_eq!(998996, part1(&example, false));
    }

    #[test]
    fn part2_test() {
        let example: String = "turn on 0,0 through 0,0
toggle 0,0 through 999,999".to_string();
        assert_eq!(2000001, part1(&example, true));
    }
}
