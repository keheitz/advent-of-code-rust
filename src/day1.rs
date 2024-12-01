pub fn setup_vectors(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    let mut count: i32 = 0;

    input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).for_each(|x| {
        if count == 0 || count % 2 == 0 {
            v1.push(x);
        } else {
            v2.push(x);
        }
        count+= 1;
    });
    v1.sort();
    v2.sort();

    return (v1, v2);
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let setup_vectors = setup_vectors(input);

    let differences: Vec<i32> = setup_vectors.0.iter()
        .zip(setup_vectors.1.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();

    return differences.iter().sum();
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let setup_vectors: (Vec<i32>, Vec<i32>) = setup_vectors(input);
    let mut similarity_scores: Vec<i32> = Vec::new();

    setup_vectors.0.iter().for_each(|x: &i32| {
        let similarity: i32;
        similarity = setup_vectors.1.iter().filter(|&n| n == x).count().try_into().unwrap();
        if similarity > 0 {
            similarity_scores.push(similarity * x);
        }
    });

    return similarity_scores.iter().sum();
}
