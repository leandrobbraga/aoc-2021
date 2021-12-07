use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let buffer = read_file("./examples/input/day-02.txt");
    let commands: Vec<Command> = buffer
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| line.parse().unwrap())
        .collect();

    part1(&commands);
    part2(&commands);
}

fn part1(commands: &[Command]) {
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;

    for command in commands {
        match command {
            Command::Forward(value) => distance += value,
            Command::Down(value) => depth += value,
            Command::Up(value) => depth -= value,
        }
    }

    println!(
        "depth={}, distance={}, mul={}",
        depth,
        distance,
        depth * distance
    )
}

fn part2(commands: &[Command]) {
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;
    let mut aim: u32 = 0;

    for command in commands {
        match command {
            Command::Forward(value) => {
                distance += value;
                depth += aim * value;
            }
            Command::Down(value) => aim += value,
            Command::Up(value) => aim -= value,
        }
    }

    println!(
        "depth={}, distance={}, mul={}",
        depth,
        distance,
        depth * distance
    )
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ser_command: Vec<&str> = s.split_ascii_whitespace().collect();

        if ser_command.len() != 2 {
            return Err(ParseError);
        }

        let command = ser_command[0];
        let value: u32 = ser_command[1].parse().unwrap();

        match command {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug)]
struct ParseError;

fn read_file(filepath: &str) -> io::BufReader<File> {
    let file = File::open(filepath).unwrap();
    io::BufReader::new(file)
}
