pub trait Polygon {
    fn area(&self) -> u32;
    fn corners(&self) -> i32;
    fn can_hold<T: Polygon>(&self, other: &T) -> bool;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Polygon for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn corners(&self) -> i32 {
        2
    }
    fn can_hold<T: Polygon>(&self, other: &T) -> bool {
        self.width > other.width() && self.height > other.height()
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}


#[cfg(test)]
mod rect_test
{
    use super::*;
    #[test]
    fn check_rectangle()
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        assert_eq!(rect1.area(), 1500);
        assert_eq!(rect1.can_hold(&rect2), true);
        assert_eq!(rect1.can_hold(&rect3), false);
    }
}

