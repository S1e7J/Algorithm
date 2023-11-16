fn factorial(obj: i32) -> i32 {
    return match obj {
        0..=1 => 1,
        _ => factorial(obj - 1) * obj,
    }
}

fn main() {
    println!("{:}",factorial(5));
}
