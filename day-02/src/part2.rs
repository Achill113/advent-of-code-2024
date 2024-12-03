#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    for (index, report) in _input.lines().enumerate() {
        println!("Report: {index}");

        let levels: Vec<i32> = report.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut last_level = -1;
        let mut direction = 0;
        let mut safe = true;
        let mut skipped = false;

        for (index, level) in levels.iter().enumerate() {
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
                    let next_level: i32 = levels[index + 1];

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
