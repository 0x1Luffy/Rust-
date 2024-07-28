fn main ()
{
    let s1:String = String::from("hello");
    let s2:String = s1.clone();

    println!("s1 and s2 are same: {}, {}.", s1, s2);
}