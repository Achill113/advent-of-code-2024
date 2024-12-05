use tracing::instrument;

type Report = Vec<i32>;

#[instrument(skip(_input))]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    for report_str in _input.lines() {
        let report: Report = report_str
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        
        let mut safety_result = check_safety(&report);

        if safety_result.is_err() {
            for index in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(index);

                if check_safety(&new_report).is_ok() {
                    safety_result = Ok(());
                }
            }
        }

        if safety_result.is_ok() {
            result += 1;
        }
    }

    Ok(result.to_string())
}

enum Direction {
    Increasing,
    Decreasing,
}

#[instrument(ret)]
fn check_safety(report: &Report) -> Result<(), String> {
    let mut last_level = -1;
    let mut direction: Option<Direction> = None;

    for level in report.iter() {
        if last_level != -1 {
            let diff = last_level - level;

            match diff.signum() {
                1 => {
                    match direction {
                        Some(Direction::Increasing) => {
                            if !(1..=3).contains(&diff.abs()) {
                                return Err(format!("Diff {diff} out of range"));
                            }
                        }
                        Some(Direction::Decreasing) => {
                            return Err(format!("Direction changed"));
                        }
                        None => {
                            if !(1..=3).contains(&diff.abs()) {
                                return Err(format!("Diff {diff} out of range"));
                            } else {
                                direction = Some(Direction::Increasing);
                            }
                        },
                    };
                }
                -1 => {
                    match direction {
                        Some(Direction::Decreasing) => {
                            if !(1..=3).contains(&diff.abs()) {
                                return Err(format!("Diff {diff} out of range"));
                            }
                        }
                        Some(Direction::Increasing) => {
                            return Err(format!("Direction changed"));
                        }
                        None => {
                            if !(1..=3).contains(&diff.abs()) {
                                return Err(format!("Diff {diff} out of range"));
                            } else {
                                direction = Some(Direction::Decreasing);
                            }
                        },
                    };
                }
                0 => return Err(format!("Diff was zero")),
                _ => panic!("This should never be hit")
            };
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
