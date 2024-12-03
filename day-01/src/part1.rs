#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut diff: i32 = 0;
    let mut left_location_ids: Vec<i32> = Vec::<i32>::new();
    let mut right_location_ids: Vec<i32> = Vec::<i32>::new();
    let rows: Vec<&str> = _input.lines().collect();

    for row in rows.into_iter() {
        let location_ids: Vec<&str> = row.split("   ").collect();

        let left_str = location_ids[0];
        let right_str = location_ids[1];

        match left_str.parse::<i32>() {
            Ok(left) => {
                match right_str.parse::<i32>() {
                    Ok(right) => {
                        left_location_ids.push(left);
                        right_location_ids.push(right);
                    },
                    Err(e) => panic!("Failed to parse int: {} ({right_str})", e)
                }
            },
            Err(e) => panic!("Failed to parse int: {} ({left_str})", e)
        }
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
