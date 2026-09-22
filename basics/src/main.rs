
const TAX_RATE: f64 = 0.08;
type Meters = i32;
const SEASON: &str = "autumn";
//compiler directive
#[allow(unused_variables)]
fn main() {
    //scope
    let coffee_price: f64 = 5.99;
    let points_scored: i32 = 28;
    //constant
    
    //alias
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!("The tax rate is {TAX_RATE}");

    {
       println!("The price is {coffee_price}");
       println!("A one mile race is {mile_race_length} meters") ;
    }
}

