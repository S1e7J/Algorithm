fn selection_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 1..list.len() {
        let mut min_id = i;
        for j in i+1..list.len() {
            if list[min_id] > list[j] {
                min_id = j;
            }
        }
        (list[i], list[min_id]) = (list[min_id], list[i]);
    }
    return list;
}

fn main() {
    println!("{:?}",selection_sort(vec![1,231,541,51,2,3,12,342]));
}
