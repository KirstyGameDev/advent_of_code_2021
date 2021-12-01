use std::{fs, io::Lines};

fn main() {
    println!("AoC Day 1");

    println!("Reading in the file...");

    let str_contents = fs::read_to_string("dayone_actual.txt").expect("Something went wrong reading the file.");
  
    depth_counter_one_step(str_contents)
}

fn depth_counter_one_step(input : String)
{
    let lines = input.lines();
    let mut previous_value : i32 = 0;
    let mut depth_counter : i32 = 0;

    for line in lines
    {
        let temp = line.trim().parse().expect("Unable to parse to i32");

        if previous_value == 0
        {
            previous_value = temp;
        }

        if previous_value < temp
        {
            depth_counter+= 1;            
        }

        previous_value = temp;
    }

    println!("Day one Part One answer : {}", depth_counter);
}


