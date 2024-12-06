use std::fs;

fn is_safe(report: &Vec<i32>) -> bool {
    let direction: bool = report[0] - report[1] < 0;
    let mut curr = report[0];
    for sample in &report[1..] {
        let diff = curr - *sample;
        curr = *sample;
        if diff < 0 && !direction {
            return false
        }
        if diff > 0 && direction {
            return false
        }
        if (3 < diff.abs()) || (diff.abs() < 1) {
            return false
        }
    }
    return true
}

fn part1() {

    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut safe_count = 0;
    for line in data.lines() {
        let report: Vec<i32> = line.split_whitespace()
                                   .collect::<Vec<&str>>()
                                   .iter()
                                   .map(
                                        |x| x.parse::<i32>().unwrap()
                                    ).collect();
        
        if is_safe(&report) {
            safe_count += 1;
        }
    }
    println!("{}", safe_count);
}

fn part2() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut safe_count = 0;
    for line in data.lines() {
        let report: Vec<i32> = line.split_whitespace()
                                   .collect::<Vec<&str>>()
                                   .iter()
                                   .map(
                                        |x| x.parse::<i32>().unwrap()
                                    ).collect();
        
        if is_safe(&report) {
            safe_count += 1;
        } else {
            for index in 0..report.len() {
                let mut temp_report = report.clone();
                temp_report.remove(index);
                if is_safe(&temp_report) {
                    safe_count += 1;
                    break
                }
            }
        }
    }
    println!("{}", safe_count);    
}

fn main() {
    part1();
    part2();
}