mod Rectangle;


use crate::Rectangle::Rectangle as Rect;
//#[derive(Debug)]
fn main() {
    let rect1 = Rect { width: 30, height: 50 };
    let rect2 = Rect { width: 10, height: 40 };
    let rect3 = Rect { width: 60, height: 45 };

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Rect1 Size{}",rect1.area());
    
}
