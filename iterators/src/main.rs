fn main() { // Iterator Testing
    let v1 = vec![1, 2, 3];

    let mut v1_iter= v1.iter().map(|x| x + 1);
    println!("Output: {:?}", v1_iter.next());
    println!("Output: {:?}", v1_iter.next());
    println!("Output: {:?}", v1_iter.next());

    assert_eq!(v1, vec![1, 2, 3]);
}
