// utils

pub fn dedup_near<T: Clone + PartialEq>(items: &mut Vec<(T, u32)>) {
	let mut write = 0;
	for read in 1..items.len() {
		if items[read].0 == items[write].0 {
			items[write].1 += items[read].1;
		} else {
			write += 1;
			items[write] = items[read].clone();
		}
	}
	items.truncate(write + 1);
}