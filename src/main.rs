mod bubble_sort;
mod traffic_light;
mod sum_nums;
mod calculate_area;

use bubble_sort::bubble_sort;
use traffic_light::light_duration;
use sum_nums::sum_numbers;
use calculate_area::*;

// lesson 3
fn sort_test() {
    let mut arr = [28.1, 47.1, 6.6, 75.7, 34.3, 6.3, -82.9, -11.1];
    bubble_sort(&mut arr);
    println!("{:?}", arr);  // 输出：[-82.9, -11.1, 6.3, 6.6, 28.1, 34.3, 47.1, 75.7]
    let mut words = ["rust", "is", "the", "best", "language"];
    bubble_sort(&mut words);
    println!("{:?}", words); // 输出：["best", "is", "language", "rust", "the"]
}

// lesson 4-1 
fn traffic_light_test() {
    light_duration()
}
// lesson 4-2
fn sum_numbers_test() {
    let numbers = &[7, 5, 89, 33, 266];
    let result = sum_numbers(numbers);
    match result {
        Some(sum) => println!("Sum result: {}", sum),
        None => println!("Sum overflowed!"),
    }
}
// lesson 4-3
fn calculate_area_test() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 6.0, height: 8.0 };
    let square = Square { side: 6.5 };
  
    print_area(circle);
    print_area(triangle);
    print_area(square);
  }
fn main() {
    // sort_test()
    println!("👇1. 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同：");
    traffic_light_test();
    println!("👇2. 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None：");
    sum_numbers_test();
    println!("👇3. 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束：");
    
    calculate_area_test();
}

