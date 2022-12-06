use crate::data::data_provider;

static DAY_NUMBER: i32 = 6;
static MARKER_LENGHT: [usize; 2] = [4, 14];

pub fn execute(level: i32) {
    let content = data_provider::load_data(DAY_NUMBER, level);

    let marker_length = MARKER_LENGHT[(level as usize) - 1];

    let signal = content.first().unwrap().chars();
    let signal_length = signal.clone().count();

    let mut marker_position: i32 = -1;
    for i in 0..(signal_length - marker_length + 1) {
        let mut marker = signal.clone().skip(i).take(marker_length);
        let mut marker_found = true;

        'check_loop: for _ in 0..(marker_length - 1) {
            let check_marker = marker.next().unwrap();

            for check in marker.clone() {
                if check_marker == check {
                    marker_found = false;
                    break 'check_loop;
                }
            }
        }

        if marker_found {
            marker_position = (i + marker_length) as i32;
            break;
        }
    }

    data_provider::write_data(DAY_NUMBER, level, vec![marker_position.to_string()]);
}