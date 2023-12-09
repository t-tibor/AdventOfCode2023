use lazy_static::lazy_static;

use std::{io::{BufReader, BufRead}, fs::File};
use regex::Regex;


struct Sample {
    red: u32,
    green: u32,
    blue: u32
}

impl Sample {

    fn parse(s: &str) -> Sample {

        lazy_static! {
            static ref RED_RE: Regex = Regex::new(r"(\d+) red").unwrap();
            static ref GREEN_RE: Regex = Regex::new(r"(\d+) green").unwrap();
            static ref BLUE_RE: Regex = Regex::new(r"(\d+) blue").unwrap();
        }

        let red_cnt = RED_RE.captures(s).map_or(0, |c| c[1].parse().unwrap() );
        let green_cnt = GREEN_RE.captures(s).map_or(0, |c| c[1].parse().unwrap() );
        let blue_cnt = BLUE_RE.captures(s).map_or(0, |c| c[1].parse().unwrap() );

        Sample { red: red_cnt, green: green_cnt, blue: blue_cnt }
    }

    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }

    fn combine(&self, other: &Sample) -> Sample {
        Sample {
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
            red: self.red.max(other.red),

        }
    }

    fn empty() -> Sample {
        Sample{blue:0,green:0,red:0}
    }

}

fn is_game_ok(sample: &Sample) -> bool {
    (sample.red <= 12) && (sample.green <= 13) && (sample.blue <= 14)
}

fn main() {
    const PATH: &str = r"C:\500_Repos\AdventOfCode2023\day2\day2Input.txt";

    // Open the file and create a BufReader to read the file in chunks
    let file = File::open(PATH).expect("Something went wrong opening the file");
    let reader = BufReader::new(file);

    let mut accu = 0;
    let mut power_accu = 0;

    let payload_re = Regex::new(r"Game \d+: (.+)").unwrap();


    for (game_idx, line_option) in reader.lines().enumerate() {

        if let Ok(line) = line_option {

            println!("{}", line);

            if let Some(payload_capture) = payload_re.captures(&line) {

                let samples: Vec<_> = payload_capture[1]
                .split(";")
                .map(|s| Sample::parse(s))
                .collect();

                if samples.iter().all(|s| is_game_ok(&s)) {
                    accu += game_idx+1;
                    println!("OK - accu={}", accu);
                }

                let min_required_sample = samples.iter().fold(Sample::empty(), |acc,s| acc.combine(s));
                power_accu += min_required_sample.power();
            }
        }
        else {
            println!("Cannot get line!");
        }
    }

    println!("{}", accu);
    println!("Sum of powers = {}", power_accu);

}
