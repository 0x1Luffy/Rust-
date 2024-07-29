fn main()
{
    let s1:String = String::from("Hello");
    let len:usize = calculate_len(&s1); // s1 sent the borrowing 
    println!("The length of String s1 is: {}", len);

}

fn calculate_len(s2:&String)->usize // s2 borrowed s1 
{
    return s2.len();
}