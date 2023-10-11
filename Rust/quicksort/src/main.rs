fn quicksort(list: Vec<i32>) -> Vec<i32> {
    if list.len() < 2{
        return list;
    } else {
        let pivot = list[0];
        let (smalls, bigs) = list[1..].iter().partition(|x| *x < &pivot);
        return [quicksort(smalls), vec![pivot], quicksort(bigs)].concat();
    }
}

fn main() {
    println!("{:?}",quicksort(vec![12,3,45,634,54,23,432,123,54,3,53,1,52,23,12,3542,43]));
}
