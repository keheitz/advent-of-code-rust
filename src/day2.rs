#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut safe_report_count: usize = 0;
    input.lines().for_each(|x: &str| {
        let report: Vec<i32> = x.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        if is_monotonic(report.clone()) && no_differences_greater_than_three(report.clone()) {
            safe_report_count += 1;
        }
    });

    return safe_report_count;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut safe_report_count: usize = 0;
    input.lines().for_each(|x: &str| {
        let report: Vec<i32> = x.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        // println!("{:?}", report);
        if is_monotonic(report.clone()) && no_differences_greater_than_three(report.clone()) {
            safe_report_count += 1;
            println!("ALREADY SAFE");
        } else {
            for i in 0..report.len() {
                let mut modified_report = report.clone();
                modified_report.remove(i);
                // println!("{:?}", modified_report);
                if is_monotonic(modified_report.clone()) && no_differences_greater_than_three(modified_report.clone()) {
                    safe_report_count += 1;
                    println!("SAFE MODIFIED");
                    break;
                }
            }

        }
    });

    return safe_report_count;
}

pub fn is_monotonic(report: Vec<i32>) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let is_increasing = report.windows(2).all(|w: &[i32]| w[0] <= w[1]);
    let is_decreasing = report.windows(2).all(|w: &[i32]| w[0] >= w[1]);

    is_increasing || is_decreasing
}

pub fn no_differences_greater_than_three(report: Vec<i32>) -> bool {
    let differences: Vec<i32> = report.windows(2).map(|w: &[i32]| (w[1] - w[0]).abs()).collect();

    return differences.iter().filter(|&x| *x > 3 || *x < 1).count() == 0;
}
