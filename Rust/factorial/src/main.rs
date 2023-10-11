fn factorial(obj: i32) -> i32 {
    if obj == 0 {
        return 1;
    } else {
        return obj * factorial(obj - 1)
    }
}

fn main() {
    println!("{:}",factorial(5));
}
