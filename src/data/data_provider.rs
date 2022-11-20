use std::io::Write;
use std::fs;
use std::fs::OpenOptions;

pub fn load_data(day: i32, level: i32) -> Vec<String> {
    let full_path = format!("./data/day{day}-{level}.txt", day=day, level=level);
    let contents = fs::read_to_string(full_path).expect("Couldn't read file.");

    contents.split('\n').map(String::from).collect()
}

pub fn write_data(day: i32, level: i32, data: Vec<String>) {
    let full_path = format!("./data/day{day}-{level}.solution.txt", day=day, level=level);
    let file = OpenOptions::new().write(true).create(true).open(full_path);

    let result = write!(file.expect("Couldn't write solution to file."), "{}", data.join("\n"));

    match result {
        Ok(()) => println!("Solution written successfully."),
        Err(e) => println!("{e:?}"),
    }
}