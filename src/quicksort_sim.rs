// extern crate rand;

// use rand::random;
/// Rust code to perform quick sort on Vec<i32>

// Return middle element of the array under consideration to be the pivot
fn get_pivot_element(len: usize) -> usize {
	(len)/2
}

fn partition(arr: &mut [i32], len: usize, pivot_pos: usize) -> usize {
	arr.swap(0, pivot_pos);
	let mut i: usize = 1;
	let mut j: usize = i;

	// get pivot element to the first location
	let pivot = arr[0];

	while j < len {
		if arr[j] < pivot {
			arr.swap(i, j);
			i += 1;
		}
		j += 1;
	}
	arr.swap(0 , i - 1);

	return i;
}

// Actual quick sort function, recursively calls itself after partitioning the array around the first middle element of the array
// TODO: change the middle element to the element from a randomly generated index
fn sort(arr: &mut [i32], len: usize) {
	if len < 2 {
		return;
	}

	let pivot_pos = get_pivot_element(len);
	let part = partition(arr, len, pivot_pos);

	if part == len {
		sort(&mut arr[0..(part - 1)], part - 1);
		return;
	}
	else if part == 1 {
		sort(&mut arr[1..len], len - 1);
		return;
	}
	sort(&mut arr[0..(part - 1)], part - 1);
	sort(&mut arr[part..len], len - part);
}

fn main() {
	// let mut v = [0..100];
	// for index in 0..100 {
	// 	v[index] = random::<i32>() % 1000;
	// }

	let mut v = [3, 4, 2, 1, 6, 13, 16, 12, 86, 72, 62, 14, 62];
	let len = v.len();
	println!("Initial array     : {:?}", v);
	sort(&mut v, len);

	println!("Final sorted array: {:?}", v);
}
