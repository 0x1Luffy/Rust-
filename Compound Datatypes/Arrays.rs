fn main ()
{
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Array is : {:?}", numbers ); // Debuggable format

    // Array of String

    let fruits : [&str; 3] = ["Apple","Banana", "Orange"];
    println!("Fruits are : {:?}", fruits);
}