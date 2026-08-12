
const TAX_RATE: f64 = 0.08;
fn main() {
    //scope
    let coffee_price: f64 = 5.99;
    //constant
    println!("The tax rate is {TAX_RATE}");

    {
       println!("The price is {coffee_price}"); 
    }
}

