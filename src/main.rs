use std::vec;

fn main() {
    let x = String::from("Hello, ");
    let y = String::from("World!");
    println!("x: {}, y: {}", x, y);
    let z = x.clone() + y.as_str();

    for c in 1..50 {
        println!("c: {}", c);
    }

    let a = vec![1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("i: {}", i);
    }
    println!("sum: {}", sum(a));

    println!("z: {}", z);
    println!("x: {}, y: {}", x, y);

}




fn sum(a:Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in a.iter() {
        sum += i;
    }
    sum
}