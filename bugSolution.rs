fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1;
    }
    {
        let z = &mut x; // z is a mutable reference to x, in a different scope
        *z += 1;
    }
    println!("x: {}", x);
}