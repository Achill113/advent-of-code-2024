use regex::Regex;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result: i32 = 0;
    let pattern = r"mul\(\d{1,3},\d{1,3}\)";
    let regexer = Regex::new(pattern).unwrap();

    for command_match in regexer.find_iter(_input) {
        let command = command_match.as_str();

        let first_index = command.find('(').unwrap(); // we know this should always return a value
        let last_index = command.find(')').unwrap(); // we know this should always return a vlue
        let num_substr = &command[first_index + 1..last_index];
        let nums: Vec<i32> = num_substr.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

        result += nums[0] * nums[1];
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
