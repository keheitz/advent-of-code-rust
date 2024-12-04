use regex::Regex;

fn extract_mul_expressions(input: String) -> Vec<String> {
    // Create a regex to match mul() expressions with 1-3 digit arguments
    let re: Regex = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    
    // Find all matches and collect them
    re.captures_iter(&input)
        .filter_map(|cap: regex::Captures<'_>| {
            cap.get(1).map(|m: regex::Match<'_>| m.as_str().to_string())
        })
        .collect()
}

fn remove_disabled_instructions(input: &str) -> String {
    let mut result = input.to_string();
    
    // Continue removing sections until no more sections between don't() and do() exist
    while let Some(start_index) = result.find("don't()") {
        // Find the next do() after don't()
        if let Some(end_index) = result[start_index..].find("do()") {
            // Remove the substring from don't() to the next do()
            result.replace_range(start_index..start_index + end_index + 4, "");
        } else {
            // If no do() is found, remove everything from don't() onwards
            result.truncate(start_index);
            break;
        }
    }
    
    result
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let multiply_instructions: Vec<String> = extract_mul_expressions(input.to_string());
    let mut total_product: i32 = 0;
    multiply_instructions.into_iter().for_each(|x: String| {
        let args: Vec<i32> = x.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let product: i32 = args.iter().product();
        total_product += product;
    });

    return total_product;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let stripped_instructions: String = remove_disabled_instructions(input);
    let multiply_instructions: Vec<String> = extract_mul_expressions(stripped_instructions);
    let mut total_product: i32 = 0;
    multiply_instructions.into_iter().for_each(|x: String| {
        let args: Vec<i32> = x.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let product: i32 = args.iter().product();
        total_product += product;
    });

    return total_product;
}
