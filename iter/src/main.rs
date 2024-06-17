fn main() {
   let v1 = vec![1,2,3,4];
    let mut owl = v1.iter();
    // for x in owl {
    //     println!("{}", x);
    // }
    assert_eq!(owl.next(), Some(&1));
    assert_eq!(owl.next(), Some(&2));
    assert_eq!(owl.next(), Some(&3));
    assert_eq!(owl.next(),  Some(&4));
    assert_eq!(owl.next(),  None);

    let v2 = vec![2,3,4];
    let dog = v2.iter();
    let k:i32 = dog.sum();
    println!("{:?}", k);

    let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();
}
