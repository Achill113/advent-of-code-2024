#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut left_location_ids = vec![];
    let mut right_location_ids = vec![];

    for line in _input.lines() {
        let mut location_ids = line.split_whitespace();
        left_location_ids.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        right_location_ids.push(location_ids.next().unwrap().parse::<i32>().unwrap());
    }

    let mut result: i32 = 0;

    for left_id in left_location_ids {
        let mut occurrences: i32 = 0;

        for right_id in right_location_ids.clone() {
            if left_id == right_id {
                occurrences += 1;
            }
        }

        if occurrences > 0 {
            result += left_id * occurrences;
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
