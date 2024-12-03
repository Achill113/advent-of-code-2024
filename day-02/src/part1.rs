use tracing::instrument;

type Report = Vec<i32>;

#[instrument(skip(_input))]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    for (index, report_str) in _input.lines().enumerate() {
        println!("Report: {index}");

        let report: Report = report_str
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        
        let safety_result = check_safety(&report);

        match safety_result {
            Ok(_) => result += 1,
            Err(e) => println!("{:?}", e)
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
    println!("{report:?}");
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
