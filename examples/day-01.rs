use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let buffer = read_file();
    let measurements: Vec<u32> = buffer
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    part_1(&measurements);
    part_2(&measurements);
}

fn read_file() -> io::BufReader<File> {
    let file = File::open("./examples/input/day-01.txt").unwrap();
    io::BufReader::new(file)
}

fn part_1(measurements: &[u32]) {
    let mut prev_measurement = measurements.get(0).unwrap();
    let mut counter = 0;

    for cur_measurement in measurements.iter().skip(1) {
        if cur_measurement > prev_measurement {
            counter += 1;
        }

        prev_measurement = cur_measurement;
    }
    println!("part_1={}", counter)
}

fn part_2(measurements: &[u32]) {
    let mut windows = measurements.windows(3);

    let mut prev_measurement: u32 = windows.next().unwrap().iter().sum();
    let mut counter = 0;

    for cur_measurement in windows {
        let cur_measurement = cur_measurement.iter().sum();

        if cur_measurement > prev_measurement {
            counter += 1;
        }

        prev_measurement = cur_measurement;
    }
    println!("part_2={}", counter)
}
