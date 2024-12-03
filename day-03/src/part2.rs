use regex::Regex;
use tracing::instrument;

#[instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result: i32 = 0;
    let mut act = true;
    let pattern = r"(mul\(\d{1,3},\d{1,3}\))|(do(n't)?\(\))";
    let regexer = Regex::new(pattern).unwrap();

    for command_match in regexer.find_iter(_input) {
        let command = command_match.as_str();

        if command == "don't()" {
            act = false;
            continue;
        } else if command == "do()" {
            act = true;
            continue;
        }

        if act {
            let first_index = command.find('(').unwrap(); // we know this should always return a value
            let last_index = command.find(')').unwrap(); // we know this should always return a vlue
            let num_substr = &command[first_index + 1..last_index];
            let nums: Vec<i32> = num_substr.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

            result += nums[0] * nums[1];
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
