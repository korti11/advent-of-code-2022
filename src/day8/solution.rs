use std::collections::HashMap;

use crate::data::data_provider;

static DAY_NUMBER: i32 = 8;

pub fn execute(level: i32) {
    let content = data_provider::load_data(DAY_NUMBER, level);

    let y_len = content.len() as u32;
    let x_len = content.first().unwrap().chars().count() as u32;

    let mut tree_map: HashMap<(u32, u32), u32> = HashMap::new();

    for (pos_y, line) in content.iter().enumerate() {
        for (pos_x, tree) in line.chars().enumerate() {
            let height = tree.to_digit(10);
            match height {
                Some(v) => {
                    tree_map.insert((pos_x as u32, pos_y as u32), v);
                },
                None => {
                    tree_map.insert((pos_x as u32, pos_y as u32), 0);
                }
            }
        }
    }

    if level == 1 {
        let mut visible_trees = 0;

        for pos_y in 0..y_len {
            for pos_x in 0..x_len {
                // If we are on the edge of the grid every tree is visible
                if pos_y == 0 || pos_y == (y_len - 1) || pos_x == 0 || pos_x == (x_len - 1) {
                    visible_trees += 1;
                    continue;
                }

                let own_height = tree_map.get(&(pos_x, pos_y)).unwrap();

                let mut up_hidden = false;
                for up_pos_y in (0..pos_y).rev() {
                    let up_tree_height = tree_map.get(&(pos_x, up_pos_y)).unwrap();
                    if own_height <= up_tree_height {
                        up_hidden = true;
                        break;
                    }
                }

                let mut down_hidden = false;
                for down_pos_y in (pos_y + 1)..y_len {
                    let down_tree_height = tree_map.get(&(pos_x, down_pos_y)).unwrap();
                    if own_height <= down_tree_height {
                        down_hidden = true;
                        break;
                    }
                }

                let mut left_hidden = false;
                for left_pos_x in (0..pos_x).rev() {
                    let left_tree_height = tree_map.get(&(left_pos_x, pos_y)).unwrap();
                    if own_height <= left_tree_height {
                        left_hidden = true;
                        break;
                    }
                }

                let mut right_hidden = false;
                for right_pos_x in (pos_x + 1)..y_len {
                    let down_tree_height = tree_map.get(&(right_pos_x, pos_y)).unwrap();
                    if own_height <= down_tree_height {
                        right_hidden = true;
                        break;
                    }
                }

                if !up_hidden || !down_hidden || !left_hidden || !right_hidden {
                    visible_trees += 1;
                }
            }
        }

        data_provider::write_data(DAY_NUMBER, level, vec![visible_trees.to_string()]);
    } else {
        let mut hightest_scenic_score = 0;

        for pos_y in 0..y_len {
            for pos_x in 0..x_len {
                let own_height = tree_map.get(&(pos_x, pos_y)).unwrap();

                let mut up_score = 0;
                for up_pos_y in (0..pos_y).rev() {
                    let up_tree_height = tree_map.get(&(pos_x, up_pos_y)).unwrap();
                    up_score += 1;
                    if own_height <= up_tree_height {
                        break;
                    }
                }

                let mut down_score = 0;
                for down_pos_y in (pos_y + 1)..y_len {
                    let down_tree_height = tree_map.get(&(pos_x, down_pos_y)).unwrap();
                    down_score += 1;
                    if own_height <= down_tree_height {
                        break;
                    }
                }

                let mut left_score = 0;
                for left_pos_x in (0..pos_x).rev() {
                    let left_tree_height = tree_map.get(&(left_pos_x, pos_y)).unwrap();
                    left_score += 1;
                    if own_height <= left_tree_height {
                        break;
                    }
                }

                let mut right_score = 0;
                for right_pos_x in (pos_x + 1)..y_len {
                    let down_tree_height = tree_map.get(&(right_pos_x, pos_y)).unwrap();
                    right_score += 1;
                    if own_height <= down_tree_height {
                        break;
                    }
                }

                let scenic_score = up_score * down_score * left_score * right_score;
                if scenic_score > hightest_scenic_score {
                    hightest_scenic_score = scenic_score;
                }
            }
        }

        data_provider::write_data(DAY_NUMBER, level, vec![hightest_scenic_score.to_string()]);
    }
}