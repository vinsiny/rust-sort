pub fn sum_numbers(numbers: &[u32]) -> Option<u32> {
  let mut sum: u32 = 0;
  for &num in numbers {
      match sum.checked_add(num) {
          Some(result) => sum = result,
          None => return None,
      }
  }
  Some(sum)
}
