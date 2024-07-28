fn main ()
{
    let x:String = String::from("I transferred Ownership");
    transfer_ownership(x);
    // println!("Accessing the old variable: {}", x);
}

fn transfer_ownership(new_owner:String)
{
    println!("I am the new owner now: {}", new_owner);
}


// The above code throwed an error as i was accessing the 
// old owners value for the x which he trasnfered to the new_owner
// So to make the code according to the Ownership rules
 // I'll comment out line 5 