use std::{fs, io::Lines};

fn main() {
    println!("AoC Day 1");

    println!("Reading in the file...");

    let str_contents = fs::read_to_string("dayone_actual.txt").expect("Something went wrong reading the file.");
  
    //depth_counter_one_step(str_contents);

    depth_counter_three_step(str_contents);
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

fn depth_counter_three_step(input : String)
{
    let lines = input.lines();

    let mut previous_value : i32 = 0;
    let mut current_value : i32 = 0;
    let mut depth_counter : i32 = 0;

    let mut remaining  = lines.count();
    let mut index : usize = 0;


    let numbers: Vec<i32> = input
    .split_whitespace()
    .map(|s| s.parse().expect("parse error"))
    .collect();

    while remaining > 3
    {
        let valueOne : i32 = numbers[index] + numbers[index+1] + numbers[index+2];
        let valueTwo: i32 =  numbers[index+1] + numbers[index+2] + numbers[index+3];

        if (valueTwo > valueOne)
        {
            depth_counter += 1;
        }
        index += 1;
        remaining -= 1;
    }
    

    println!("Day one Part Two answer : {}", depth_counter);
}


