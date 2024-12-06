use std::{cmp::Ordering, collections::HashMap};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    let sections: Vec<&str> = _input.split("\n\n").collect();

    let rules: Vec<&str> = sections[0].lines().collect();
    let pages: Vec<Vec<i32>> = sections[1].lines().map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect()).collect();

    let rules_map: HashMap<i32, Vec<i32>> = process_rules(&rules);

    for row in pages {
        let mut correct = true;

        for (index, page_num_a) in row.clone().iter().rev().enumerate() {
            if let Some(page_rules) = rules_map.get(page_num_a) {
                for i in 0..(row.len() - index - 1) {
                    let page_num_b = row[i];

                    if page_rules.contains(&page_num_b) {
                        correct = false;
                        break;
                    }
                }
            }

            if !correct {
                break;
            }
        }

        if !correct {
            let mut sorted_row = row.clone();
            sorted_row.sort_by(|a, b| if let Some(page_rules) = rules_map.get(a) {
                if page_rules.contains(&b) {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            } else {
                return Ordering::Greater;
            });
            // add middle page number to results
            result += sorted_row[(row.len() - 1) / 2];
        }
    }

    Ok(result.to_string())
}

fn process_rules(rules: &Vec<&str>) -> HashMap<i32, Vec<i32>> {
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        let parsed: Vec<i32> = rule.split("|").map(|x| x.parse::<i32>().unwrap()).collect();

        if let Some(orders) = result.get_mut(&parsed[0]) {
            orders.push(parsed[1]);
        } else {
            result.insert(parsed[0], vec![parsed[1]]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
