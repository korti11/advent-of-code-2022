use crate::data::data_provider;

static DAY_NUMBER: i32 = 4;

pub fn execute(level: i32) {
    let content = data_provider::load_data(DAY_NUMBER, level);

    let mut fully_ranged = 0;
    for pair in content.iter() {
        let mut pair_split = pair.split(',');
        let first_elf_range = to_range(pair_split.next().unwrap());
        let second_elf_range = to_range(pair_split.next().unwrap());

        if level == 1 {
            if (first_elf_range.0 <= second_elf_range.0 && first_elf_range.1 >= second_elf_range.1) || 
                (second_elf_range.0 <= first_elf_range.0 && second_elf_range.1 >= first_elf_range.1) {
                fully_ranged += 1;
            }
        } else if is_in_range(first_elf_range.0, first_elf_range.1, second_elf_range.0) || 
            is_in_range(first_elf_range.0, first_elf_range.1, second_elf_range.1) || 
            is_in_range(second_elf_range.0, second_elf_range.1, first_elf_range.0) || 
            is_in_range(second_elf_range.0, second_elf_range.1, first_elf_range.1) {
            fully_ranged += 1;
        }
    }

    data_provider::write_data(DAY_NUMBER, level, vec![fully_ranged.to_string()]);
}

fn to_range(string_range: &str) -> (u32, u32) {
    let mut range_split = string_range.split('-');
    let range_start = range_split.next().unwrap().parse().unwrap();
    let range_end = range_split.next().unwrap().parse().unwrap();

    (range_start, range_end)
}

fn is_in_range(range_start: u32, range_end: u32, value: u32) -> bool {
    value >= range_start && value <= range_end
}