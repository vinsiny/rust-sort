pub fn bubble_sort<T: PartialOrd>(arrays: &mut[T]) {
  let len = arrays.len();
  for i in 0..len {
      for j in 0..(len - i - 1) {
          if arrays[j] > arrays[j + 1] {
              arrays.swap(j, j + 1);
          }
      }
  }
}

// fn main() {
//   let mut arr = [28.1, 47.1, 6.6, 75.7, 34.3, 6.3, -82.9, -11.1];
//   bubble_sort(&mut arr);
//   println!("{:?}", arr);  // 输出：[-82.9, -11.1, 6.3, 6.6, 28.1, 34.3, 47.1, 75.7]
//   let mut words = ["rust", "is", "the", "best", "language"];
//   bubble_sort(&mut words);
//   println!("{:?}", words); // 输出：["best", "is", "language", "rust", "the"]
// }

