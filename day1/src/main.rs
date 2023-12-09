use std::{io::{BufReader, BufRead}, fs::File};

const FROM: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const TO: [&'static str; 9]  = ["o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e"];

fn preprocess(line:String) -> String {
   
   let mut retval = line.clone();

   let zipped: Vec<(_,_)> = FROM.iter().zip(TO.iter()).collect();
   for (f,t) in zipped {
        if retval.contains(f) {
            retval = retval.replace(f, t);
        }
   }
   return retval;
}

fn main() {
    const PATH: &str = "../day1Input.txt";

    // Open the file and create a BufReader to read the file in chunks
    let file = File::open(PATH).expect("Something went wrong opening the file");
    let reader = BufReader::new(file);

    let mut accu = 0;

    // Read the file in chunks and print each line to the console
    for line in reader.lines() {
        let l = preprocess(line.expect("Cannot get line"));

        let digits: Vec<_> = l.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).expect("Cannot parse number"))
        .collect();
        
        let num = digits.first().expect("No first number")*10 + digits.last().expect("No last number");

        println!("{}", num);

        accu += num;
    }

    println!("Result is {}.", accu);

}
