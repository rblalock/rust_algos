pub fn bubble_sort(mut nums: Vec<u32>) -> Vec<u32> {
	let len = nums.len();

	for i in 0..len {
		for j in 0..(len - i - 1) {
			if nums[j] > nums[j + 1] {
				nums.swap(j, j + 1)
			}
		}
	}

	return nums;
}

pub fn merge_sort(nums: Vec<u32>) -> Vec<u32> {
	if nums.len() <= 1 {
		return nums;
	}

	let mid = nums.len() / 2;
	let left_half = merge_sort(nums[..mid].to_vec());
	let right_half = merge_sort(nums[mid..].to_vec());

	return merge(left_half, right_half);
}

fn merge(left: Vec<u32>, right: Vec<u32>) -> Vec<u32> {
	let mut result = Vec::new();
	let mut left_index = 0;
	let mut right_index = 0;

	while left_index < left.len() && right_index < right.len() {
		if left[left_index] <= right[right_index] {
			result.push(left[left_index]);
			left_index += 1;
		} else {
			result.push(right[right_index]);
			right_index += 1;
		}
	}

	if left_index < left.len() {
		result.extend_from_slice(&left[left_index..]);
	}

	if right_index < right.len() {
		result.extend_from_slice(&right[right_index..]);
	}

	return result;
}


#[cfg(test)]
mod sort_tests {
	use super::*;

	#[test]
	fn test_bubble_sort() {
		assert_eq!(
			bubble_sort(vec![4,2,3,1]), 
			vec![1,2,3,4]
		);
	}

	#[test]
	fn test_merge_sort() {
		assert_eq!(
			merge_sort(vec![9,7,5,3,1,2,4,6,8]), 
			vec![1,2,3,4,5,6,7,8,9]
		);
	}
}
