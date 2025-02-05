fn main() {
    let mut x = 5;
    {  // This block will ensure that y is dropped before z is used. 
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x;
    *z = 15;
    println!("x = {}", x);
}