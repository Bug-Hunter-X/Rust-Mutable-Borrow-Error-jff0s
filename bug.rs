fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y += 1; // this is fine
    *z += 1; // ERROR: this will cause a compile-time error
}