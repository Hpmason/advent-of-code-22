use std::collections::BinaryHeap;


pub fn solution<S: AsRef<str>>(input: S) -> (u32, u32, u32) {
	let mut heap = BinaryHeap::new();
	let mut sum = 0u32;
	let mut lines = input.as_ref().lines();
	while let Some(line) = lines.next() {
		// println!("{line}");
		let Ok(val): Result<u32, std::num::ParseIntError> = line.parse() else {
			heap.push(sum);
			sum = 0;
			continue;
		};
		sum += val;
	}
	if sum != 0 {
		heap.push(sum);
	}
	(heap.pop().expect("input has >= 3 elves"), heap.pop().expect("input has >= 3 elves"), heap.pop().expect("input has >= 3 elves"))
}

#[cfg(test)]
mod tests {
    use crate::day1::solution;

	#[test]
	fn example() {
		let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
		assert_eq!(solution(input), 24000);
	}
}