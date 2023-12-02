fn main() {
    let (red, green, blue): (i32, i32, i32) = (12, 13, 14);
    part1(red, green, blue);
    part2();
}

pub fn part1(red:i32, green:i32, blue:i32) {
    let mut sum: i32 = 0;
    for line in std::fs::read_to_string("src/puzzleinput.txt").unwrap().lines() {
        let index = line.find(": ").unwrap();
        let game_info = &line[(index + 2)..];
        let game_pre = &line[..index];
        let space_index = game_pre.find(" ").unwrap();
        let game_num = &line[(space_index + 1)..index];
        if verify(red, green, blue, &game_info) {
            sum += game_num.parse::<i32>().unwrap();
        }
    }
    println!("{}", sum);
}


pub fn verify(red: i32, green: i32, blue: i32, string: &str) -> bool {
    let games = string.split("; ");
    for game in games {
        let piles = game.split(", ");
        for pile in piles {
            let vars: Vec<&str> = pile.split(" ").collect();
            let number = vars[0].parse::<i32>().unwrap();
            let color = vars[1];
            match color {
                "red" => {
                    if number > red {
                        return false;
                    }
                }
                "green" => {
                    if number > green {
                        return false;
                    }
                }
                "blue" => {
                    if number > blue {
                        return false;
                    }
                }
                _ => println!("{}", "Wtf"),
            }
        }
    }
    return true;
}

pub fn part2() {
    let mut sum: i32 = 0;
    for line in std::fs::read_to_string("src/puzzleinput.txt").unwrap().lines() {
        let index = line.find(": ").unwrap();
        let game_info = &line[(index + 2)..];
        sum += get_power(game_info);
    }
    println!("{}", sum);
}

pub fn get_power(game_info: &str) -> i32 {
    let (mut red_cap, mut green_cap, mut blue_cap): (i32, i32, i32) = (i32::MIN, i32::MIN, i32::MIN);
    let games = game_info.split("; ");
    for game in games {
        let piles = game.split(", ");
        for pile in piles {
            let vars: Vec<&str> = pile.split(" ").collect();
            let number = vars[0].parse::<i32>().unwrap();
            let color = vars[1];
            match color {
                "red" => {
                    if number > red_cap {
                        red_cap = number;
                    }
                }
                "green" => {
                    if number > green_cap {
                        green_cap = number;
                    }
                }
                "blue" => {
                    if number > blue_cap {
                        blue_cap = number;
                    }
                }
                _ => println!("{}", "Wtf"),
            }
        }
    }
    return red_cap * blue_cap * green_cap;
}