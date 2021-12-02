use std::{fs, io::Lines, error};

fn main() {
    println!("AoC Day 1");

    println!("Reading in the file...");

    let str_contents = fs::read_to_string("daytwo_actual.txt").expect("Something went wrong reading the file.");
  
    //depth_counter_one_step(str_contents);

    //depth_counter_three_step(str_contents);
    let use_aim : bool = true;
    submarine_course_calculator(str_contents, use_aim);
}

fn submarine_course_calculator(input : String, use_aim : bool)
{
    let split_string : Vec<&str> = input.split_whitespace().collect();

    let mut idx : i32 = 1;
    
    let mut horizontal : i32 = 0;
    let mut vertical : i32 = 0;
   
    let mut horizontalMuliplyer : i32 = 0;
    let mut verticalMultiplyer : i32 = 0;
    let mut aim = 0;

    for section in split_string
    {     
        // We've got a number to go by
        if idx % 2 == 0
        {
            let section_num : i32 = section.parse().expect("Unable to parse to i32");

            
            horizontal += section_num * horizontalMuliplyer;

             if (!use_aim)
             {
                vertical += section_num * verticalMultiplyer;
                
             }
             else 
             {
                aim += section_num * verticalMultiplyer;    
                
                // We need to move as well
                if (horizontalMuliplyer != 0)
                {
                    println!("aim is = {}, horizontal is {}", aim, horizontal);
                    vertical += section_num * aim;
                    println!("vertical {}", vertical);
                }

             }
            

            horizontalMuliplyer = 0;
            verticalMultiplyer = 0;
        }
        else
        {
            if section.eq("forward")
            {
                horizontalMuliplyer = 1;
            }
            // back isn't used but keeping here in case
            else if (section.eq("back"))
            {
                horizontalMuliplyer = -1;
            }
            else if (section.eq("up"))
            {
                verticalMultiplyer = -1;
            }
            else if (section.eq("down"))
            {
                verticalMultiplyer = 1;
            }
            else
            {
                println!("unable to interpret section {}",  section);
            }

        }
        
        idx+=1;
        
    }

    let answer = horizontal * vertical;

    println!("horizontal {}, vertical {} : answer {}", horizontal, vertical, answer);

    //assert!(answer == 1636725);
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


