fn main()
{
    let mut x: u8 = 8;
    x = x+1;
    let y: &mut u8 = &mut x;
    *y = *y+1; // Dereferencing
    println!("Now x becomes: {}", y);
}