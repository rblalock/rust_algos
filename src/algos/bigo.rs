// O(N) Problem
//
// As your input grows, so does your memory
// and computational costs
// GROWTH IS WITH RESPECT TO THE INPUT
//
// Simple trick: Look for loops!
pub fn sum_char_codes(input: &str) -> u32 {
	let mut sum: u32 = 0;

	for c in input.chars()  {
		// In this case - we still ignore this in O(N)
		// Because in O(N) we consider the worst case
		if c == 'E' {
			return sum;
		}

		sum += c as u32;
	}

	return sum;
}
