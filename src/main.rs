pub fn bubble_sort<T: Ord>(arrays: &mut[T]) {
    let len = arrays.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arrays[j] > arrays[j + 1] {
                arrays.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut arr = [28, 47, 6, 75, 34, 63, 82, 11];
    bubble_sort(&mut arr);
    println!("{:?}", arr);  // 输出：[1, 2, 3, 4, 5, 6, 7, 8]
}

