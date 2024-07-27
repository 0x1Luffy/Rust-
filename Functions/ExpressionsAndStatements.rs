fn main()
{
    let calculate_stock :i32 ={
        let price: i32 = 45;
        let quantity: i32 = 200;
        price * quantity 

// I have not used return to return the price * quantity 
// It is automatically handled by rust as its an expression 
    };
    println!("Result is: {}", calculate_stock);
}