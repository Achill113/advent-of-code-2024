#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut diff: i32 = 0;
    let mut left_location_ids = vec![];
    let mut right_location_ids = vec![];

    for line in _input.lines() {
        let mut location_ids = line.split_whitespace();

        left_location_ids.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        right_location_ids.push(location_ids.next().unwrap().parse::<i32>().unwrap());
    }

    left_location_ids.sort();
    right_location_ids.sort();

    for i in 0..left_location_ids.len() {
        let left = left_location_ids[i];
        let right = right_location_ids[i];

        println!("{left} - {right}");

        diff += (left - right).abs();
    }

    println!("Result: {diff}");

    Ok(diff.to_string())
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
