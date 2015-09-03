/// Rust code to perform quick sort on Vec<i32>

fn get_pivot_element(beg: i32, end: i32) -> i32 {
	(beg + end)/2
}

fn partition(arr: &mut Vec<i32>, beg: i32, end: i32, pivot_pos: i32) -> i32 {
	arr.swap(beg as usize, pivot_pos as usize);
	let mut i = beg + 1;
	let mut j = i;

	// get pivot element to the first location
	let pivot = arr[0];

	while j <= end {
		if arr[j as usize] < pivot {
			arr.swap(i as usize, j as usize);
			i += 1;
		}
		j += 1;
	}
	arr.swap(beg as usize , (i - 1) as usize);
	//i = i - 1;
	println!("array: {:?}", arr);
	println!("{:?}  {:?}  {:?}", beg, end, i);
	// panic!();
	if i == beg {
		return beg + 1;
	}
	if i == end {
		return end - 1;
	}
	return i;
}

fn sort(arr: &mut Vec<i32>, beg: i32, end: i32) {
	if end - beg <= 1 {
		return;
	}

	let pivot_pos = get_pivot_element(beg, end);
	let part = partition(arr, beg, end, pivot_pos);

	sort(arr, beg, part - 1);
	sort(arr, part, end);
}

fn main() {
	let mut v = vec![10, 20, 23423, 83, 5, 4, 3, 2, 1];
	sort(&mut v, 0, 8);


}