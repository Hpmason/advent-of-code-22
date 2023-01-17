use advent_of_code_22::day2::*;

fn main() {
    let input = include_str!("../data/d2.txt");
    let score = solution_p1(input);
    println!("Part 1 answer: {}", score);
    let score = solution_p2(input);
    println!("Part 2 answer: {}", score);
}
