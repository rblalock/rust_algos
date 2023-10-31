
pub fn linear_search(haystack: &[u32], needle: u32) -> bool {
	for n in haystack {
		if *n == needle {
			return true;
		}
	}
	return false;
}

// Binary search: log(n) type of searching
// Halving the haystack over and over until it's found
// Assumes this array is sorted of course
pub fn binary_search(haystack: &[u32], needle: u32) -> Option<u32> {
	let mut lo = 0;
	let mut hi = haystack.len() as u32;
	let mut value: Option<u32> = None;

	while lo < hi {
		let midpoint = lo + (hi - lo) / 2;
		let v = haystack[midpoint as usize] as u32;

		if v == needle {
			value = Some(v);
			break;
		} else if v > needle {
			hi = midpoint;
		} else {
			lo = midpoint + 1;
		}
	}

	return value;
}


#[cfg(test)]
mod search_tests {
	use super::*;

	#[test]
	fn test_linear_search() {
		assert_eq!(
			linear_search(&[1,2,3,4], 1), 
			true
		);
	}

	#[test]
	fn test_binary_search() {
		assert_eq!(
			binary_search(&[1,2,3,4], 2), 
			Some(2)
		);
	}
}
