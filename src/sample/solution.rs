use crate::data::data_provider;

static DAY_NUMBER: i32 = 1;

pub fn execute(level: i32) {
    let content = data_provider::load_data(DAY_NUMBER, level);
    data_provider::write_data(DAY_NUMBER, level, content);
}