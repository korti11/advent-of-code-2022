use crate::data::data_provider;

static DAY_NUMBER: i32 = 3;

static LOWERCASE_OFFSET: u32 = 96;
static UPPERCASE_OFFSET: u32 = 38;

pub fn execute(level: i32) {
    let content = data_provider::load_data(DAY_NUMBER, level);

    let mut items_sum = 0;

    if level == 1 {
        for rucksack_content in content.iter() {
            let items = rucksack_content.chars();
            
            let compartment_size = items.clone().count() / 2;
            let first_compartment = items.clone().take(compartment_size);
            let second_compartment = items.clone().skip(compartment_size);

            for item in first_compartment {
                if second_compartment.clone().any(|i| i == item) {
                    items_sum += item_to_priority(item);
                    break;
                }
            }
        }
    } else {
        for i in (0..content.len()).step_by(3) {
            let first_rucksack = content.get(i).unwrap();
            let second_rucksack = content.get(i + 1).unwrap();
            let third_rucksack = content.get(i + 2).unwrap();

            for item in first_rucksack.chars() {
                if second_rucksack.contains(item) && third_rucksack.contains(item) {
                    items_sum += item_to_priority(item);
                    break;
                }
            }
        }
    }

    data_provider::write_data(DAY_NUMBER, level, vec![items_sum.to_string()]);
}

fn item_to_priority(item: char) -> u32 {
    if item.is_lowercase() {
        (item as u32) - LOWERCASE_OFFSET
    } else {
        (item as u32) - UPPERCASE_OFFSET
    }
}