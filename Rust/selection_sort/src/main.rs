fn selection_sort(list: &mut Vec<i32>) {
    for i in 1..list.len() {
        let mut min_id = i;
        for j in i+1..list.len() {
            if list[min_id] > list[j] {
                min_id = j;
            }
        }
        (list[i], list[min_id]) = (list[min_id], list[i]);
    }
}

fn main() {
    let mut list = vec![1,231,541,51,2,3,12,342];
    selection_sort(&mut list);
    println!("{:?}",list);
}
