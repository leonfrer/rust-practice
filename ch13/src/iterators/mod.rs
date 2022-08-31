use std::vec;

#[test]
fn iterator_sum_test() {
    let list = vec![2, 6, 3, 7, 1];
    let sum: i32 = list.iter().sum();
    assert_eq!(sum, 19);
    println!("{:?}", list);
}

#[test]
fn iterator_map_test() {
    let v1 = vec![2, 6, 3, 7, 1];
    let v2: Vec<_> = v1.iter().map(|i| i + 1).collect();
    assert_eq!(v2, vec![3, 7, 4, 8, 2]);
}
