use std::cmp::Reverse;

use crate::data::data_provider;

static DAY_NUMBER: i32 = 1;

pub fn execute(level: i32) {
    let content = data_provider::load_data(DAY_NUMBER, level);

    let mut calories: Vec<i32> = Vec::new();
    let mut elf_calories = 0;

    for calorie_s in content.iter() {
        match calorie_s.as_str() {
            "" => {
                calories.push(elf_calories);
                elf_calories = 0;
            },
            _ => {
                let parse_result: Result<i32, core::num::ParseIntError> = calorie_s.parse();
                match parse_result {
                    Ok(v) => elf_calories += v,
                    Err(err) => println!("{}", err)
                }
            }
        }
    }

    calories.sort_by_key(|v| Reverse(*v));
    let mut output: Vec<String> = Vec::new();

    if level == 1 {
        let highst_calories = calories.first();
        match highst_calories {
            None => {
                output.push(String::from(""));
            },
            Some(v) => {
                output.push(v.to_string());
            }
        }
    } else {
        let mut top_three_calories = 0;
        for i in 0..3 {
            let calorie = calories.get(i);
            match calorie {
                None => {
                    println!("Couldn't find calorie for index {}!", i);
                },
                Some(v) => {
                    top_three_calories += v;
                }
            }
        }
        output.push(top_three_calories.to_string());
    }

    data_provider::write_data(DAY_NUMBER, level, output);
}