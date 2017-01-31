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
    let s1 = [1,2,3];
    let mut i1 = Cow::from(&s1[..]);

    let s2 = [1,2,3, -45, 5];
    let mut i2 = Cow::from(&s2[..]);

    // No clone here because there is no mutation required.
    abs_all(&mut i1);

    // Here we clone because there is a mutation required.
    abs_all(&mut i2);

    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);

    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2);
}
