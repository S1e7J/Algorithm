fn recursive_sum(list: Vec<i32>) -> i32 {
    if list.len() == 1 {
        return list[0];
    } else {
        return list[0] + recursive_sum(list[1..].to_vec());
    }
}

fn main() {
    println!("{:}",recursive_sum(vec![1,2,3,4,5]));
}
