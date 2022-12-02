use crate::data::data_provider;

static DAY_NUMBER: i32 = 2;

pub fn execute(level: i32) {
    let content = data_provider::load_data(DAY_NUMBER, level);

    let mut score = 0;
    for round in content {
        let mut split = round.split(' ');
        let round = (split.next().unwrap_or(""), split.next().unwrap_or(""));

        if level == 1 {
            match round {
                ("A", "X") => {     // Rock vs Rock - Draw
                    score += 1 + 3;
                },
                ("A", "Y") => {     // Rock vs Paper - Win
                    score += 2 + 6;
                },
                ("A", "Z") => {     // Rock vs Scissors - Lose
                    score += 3;
                },
                ("B", "X") => {     // Paper vs Rock - Lose
                    score += 1;
                },
                ("B", "Y") => {     // Paper vs Paper - Draw
                    score += 2 + 3;
                },
                ("B", "Z") => {     // Paper vs Scissor - Win
                    score += 3 + 6;
                },
                ("C", "X") => {     // Scissor vs Rock - Win
                    score += 1 + 6;
                },
                ("C", "Y") => {     // Scissor vs Paper - Lose
                    score += 2;
                },
                ("C", "Z") => {     // Scissor vs Scissor - Draw
                    score += 3 + 3;
                },
                _ => {}
            }
        } else {
            match round {
                ("A", "X") => {     // Rock vs Scissor - Lose
                    score += 3;
                },
                ("A", "Y") => {     // Rock vs Rock - Draw
                    score += 1 + 3;
                },
                ("A", "Z") => {     // Rock vs Paper - Win
                    score += 2 + 6;
                },
                ("B", "X") => {     // Paper vs Rock - Lose
                    score += 1;
                },
                ("B", "Y") => {     // Paper vs Paper - Draw
                    score += 2 + 3;
                },
                ("B", "Z") => {     // Paper vs Scissor - Win
                    score += 3 + 6;
                },
                ("C", "X") => {     // Scissor vs Paper - Lose
                    score += 2;
                },
                ("C", "Y") => {     // Scissor vs Scissor - Draw
                    score += 3 + 3;
                },
                ("C", "Z") => {     // Scissor vs Rock - Win
                    score += 1 + 6;
                },
                _ => {}
            }
        }
    }

    data_provider::write_data(DAY_NUMBER, level, vec![score.to_string()]);
}
