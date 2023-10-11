use std::cmp::Ordering;

fn binary_search(list: Vec<i32>, obj: i32) -> usize {
    let mut low : usize = 0;
    let mut high : usize = list.len();
    let mut ret : usize = 0;
    while low <= high {
        let guess : usize = (low + high)/2;
        match list[guess].cmp(&obj) {
            Ordering::Less => {low = guess + 1},
            Ordering::Equal => {ret = guess; break},
            Ordering::Greater => {high = guess - 1},
        }
    }
    return ret;
}

fn main() {
    println!("{:}", binary_search(vec![1,2,3,4,5,6],5));
}
