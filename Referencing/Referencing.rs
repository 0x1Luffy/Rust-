fn main ()
{
    let mut s1:String = String::from("Chetan");
    let mut s2:&mut String = &mut s1;
    println!("Created and used right away: {}", s2);
    s2.push_str("kesare");
    
    let s4:String = String::from("Sangli");
    println!("After pushing string: {}", s1);

    println!("Using again second time: {}", s2);
}