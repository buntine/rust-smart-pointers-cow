use std::borrow::Cow;

// Using a clone-on-write (Cow) smart pointer allows us to write abstractions that can operate on
// borrowed data (eg: &str) that is cloned into an owned object (eg: String) in the case that mutation
// is required.

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];

        if v < 0 {
            // The first time we to_mut, input is cloned and the pointer is updated.
            // Successive calls return the existing cloned data.
            input.to_mut()[i] = -v;
        }
    }
}

fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);

    abs_all(&mut lst);

    lst.iter()
       .fold(0, |acc, &n| acc + n)
}

fn main() {
    // No clone here because there is no mutation required.
    let s1 = [1,2,3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);

    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);


    // Here we clone because there is a mutation required.
    // NOTE: The borrowed data is cloned into a new object:
    //       s2 != i2. Infact, s2 is not mutable and is not changed.
    let s2 = [1,2,3, -45, 5];
    let mut i2 = Cow::from(&s2[..]);
    abs_all(&mut i2);

    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2);


    // And no clone here because the data is already owned.
    let mut v1 = Cow::from(vec![1,2,-3,4]);
    abs_all(&mut v1);

    println!("OUT: {:?}", v1);


    // No clone here because no mutation is required.
    let s3 = [1,3,5,6];
    let sum1 = abs_sum(&s3[..]);
    println!("{:?}", s3);
    println!("{}", sum1);


    // We clone here because mutation is required.
    let s4 = [1,-3,5,-6];
    let sum2 = abs_sum(&s4[..]);
    println!("{:?}", s4);
    println!("{}", sum2);

    // Allocating memory on the heap via Box allows us to define recursive data structures.
    // Because a Box is a pointer into the heap it has a known size and so appropriate
    // memory can be allocated on the heap (to store the pointer).
    // If we defined List as "Cons(T, List<T>)" then the size of a list is dependent on the number
    // of items in it and this we would not know how much memory to allocate.
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
