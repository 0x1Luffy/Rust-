fn main()
{
    let mut s1:String = String::from("Chetan");
    make_changes(&mut s1); // s1 gave permission to make changes 
    println!("after making changes: {}", s1);
}

fn make_changes(s3:&mut String) 
{
    s3.push_str("Kesare");
}