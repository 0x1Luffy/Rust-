fn main()
{
    let y:u8 = add_numbers(2,2);
    println!("Result is : {}", y);
}

fn add_numbers(a: u8 , b:u8) -> u8 
{
    a * b
}