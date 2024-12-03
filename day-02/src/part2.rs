type Report = Vec<i32>;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    for (index, report_str) in _input.lines().enumerate() {
        println!("Report: {index}");

        let report: Report = report_str
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut last_level = -1;
        let mut direction = 0;
        let mut safe = true;
        let mut skipped = false;

        for (index, level) in report.iter().enumerate() {
            if last_level != -1 {
                if direction == 0 {
                    if *level > last_level {
                        direction = 1;
                    } else {
                        direction = -1;
                    }
                }

                let diff = level - last_level;

                println!("Direction: {direction}, Last Level: {last_level}, Level: {level}, Diff: {diff}");

                if direction > 0 && (diff < 1 || diff > 3) {
                    safe = false;
                } else if direction < 0 && (diff < -3 || diff > -1) {
                    safe = false;
                }

                if !safe && !skipped {
                    safe = true;
                    skipped = true;
                    if index == 1 {
                        direction = 0;
                    }
                    continue;
                }
            } else {
                if !skipped {
                    let next_level: i32 = report[index + 1];

                    if (level - next_level).abs() > 3 || (level - next_level).abs() < 1 {
                        skipped = true;
                        continue;
                    }
                }
            }

            last_level = *level;
        }

        println!("Safe: {safe}");
        if safe {
            result += 1;
        }
    }

    Ok(result.to_string())
}

enum Direction {
    increasing,
    decreasing,
}

fn check_safety(report: &Report) -> Result<(), String> {
    let mut last_level = -1;
    let mut direction: Option<Direction> = None;
    let mut safe = true;
    let mut skipped = false;

    for (index, level) in report.iter().enumerate() {
        if last_level != -1 {
            if direction.is_none() {
                if *level > last_level {
                    direction = Some(Direction::increasing);
                } else {
                    direction = Some(Direction::decreasing);
                }
            }

            let diff = level - last_level;

            match diff.signum() {
                1 => {
                    match direction {
                        Some(Direction::increasing) => {
                            if !(1..=3).contains(&diff.abs()) {
                                Err(format!("Diff out of range"));
                            }
                        }
                        Some(Direction::decreasing) => {
                            Err(format!("Direction changed"));
                        }
                        None => panic!(),
                    };
                }
                -1 => {
                    todo!();
                }
                0 => Err(format!("Diff was zero")),
            };

            if !safe && !skipped {
                safe = true;
                skipped = true;
                if index == 1 {
                    direction = None;
                }
                continue;
            }
        } else {
            if !skipped {
                let next_level: i32 = report[index + 1];

                if (level - next_level).abs() > 3 || (level - next_level).abs() < 1 {
                    skipped = true;
                    continue;
                }
            }
        }

        last_level = *level;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
