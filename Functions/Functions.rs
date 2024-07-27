fn main()
{
    simple_function();
    arg_function(190);
    multi_arg_functions("Chetan", 23, false);
}

fn simple_function()
{
    println!("Hello world from my created function !");
}


fn arg_function(height:u16)
{
    println!("Your height is : {}", height);
}

fn multi_arg_functions(name: &str, age:u8 , disability: bool)
{
     println!("My name is : {}, and i am {}, years old and my disability is {}",
                name, age, disability
    );
}