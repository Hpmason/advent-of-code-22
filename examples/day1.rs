use advent_of_code_22::day1;

fn main() {
    let input = include_str!("../data/d1-p1.txt");
    let top3 = day1::solution(input);
    println!("Part 1 answer: {}", top3.0);
    
    let total = top3.0 + top3.1 + top3.2;
    println!("Part 2 answer: {total}");
}
