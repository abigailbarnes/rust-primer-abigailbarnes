use solution::{coin_value, plus_one, Coin, Rectangle};

fn main() {
    //testing plus_one
    println!("testing plus_one");
    let mut n: i32 = 2;
    plus_one(&mut n);
    println!("testing plus_one(2): {}", n);
    n = -1;
    plus_one(&mut n);
    println!("testing plus_one(-1): {}", n);
    plus_one(&mut n);
    println!("testing plus_one(0): {}", n);

    println!();

    //testing Rectangle struct and related functions
    println!("testing Rectangles");
    let rect = Rectangle {
        width: 8,
        height: 10,
    };
    let sqr = Rectangle {
        width: 4,
        height: 4,
    };
    println!("testing is_square of rect 8 x 10: {}", rect.is_square());
    println!("testing is_square of sqr 4 x 4: {}", sqr.is_square());

    println!();

    println!("testing calc_area of rect 8 x 10: {}", rect.calc_area());
    println!("testing calc_area of sqr 4 x 4: {}", sqr.calc_area());

    println!();

    //testing Coin enum and related functions
    println!("testing Coin");

    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter;

    println!("testing penny coin_value: {}", coin_value(p));
    println!("testing nickel coin_value: {}", coin_value(n));
    println!("testing dime coin_value: {}", coin_value(d));
    println!("testing quarter coin_value: {}", coin_value(q));
}
