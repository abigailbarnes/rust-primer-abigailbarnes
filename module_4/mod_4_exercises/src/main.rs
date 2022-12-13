use solution::Circle;
use solution::Rectangle;
use solution::Geometry;
fn main() 
{
    //Testing Geometry: Rectangle
    println!("Testing Geometry: Rectangle");
    println!();

    //instantiating rectangle from struct to use
    let rect = Rectangle{
        length: 8.0,
        width: 10.0
    };

    println!("Expected Area: 80, Actual Area: {}", rect.get_area());
    println!();

    //instantiating circle from struct to use
    let circ = Circle{
        radius: 4.0
    };

    println!("Expected Area: 50.27, Actual Area: {}", circ.get_area());
    println!();

}
