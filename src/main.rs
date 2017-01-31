use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];

        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn main() {
    // No clone here because there is no mutation required.
    let s1 = [1,2,3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);

    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);


    // Here we clone because there is a mutation required.
    let s2 = [1,2,3, -45, 5];
    let mut i2 = Cow::from(&s2[..]);
    abs_all(&mut i2);

    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2);


    // And no clone here because the data is already owned.
    let mut v1 = Cow::from(vec![1,2,-3,4]);
    abs_all(&mut v1);

    println!("OUT: {:?}", v1);
}
