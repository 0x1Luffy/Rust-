fn main ()
{
    let name: String = String::from("Chetan");
    let name_transfer: String = name;

    // println!("New name before transfer : {}", name);
    println!("My name after transfer: {}", name_transfer);

}

// When i run this program it throws error as Ownership comes into picture
// Since i transfered name to name_transfer 
   // As rule 1 says there can be only one owner of one variable
   // Hence the code voileted the rule it throwed an error
   // To fix that i'll comment out line 6 from the above code