fn main()
{
    let x:u8 = 5;
    let y:&u8 = &x;  //Auto derefered here rather than *y:&u8 = &x;
    println!("Now x auto derefered : {}", y);
}