pub trait  Polygon{
    fn area(&self)->u32;
    fn corners(&self)->i32;
    fn can_hold<T:Polygon>(&self, other:&T)->bool;
    fn width(&self)->u32;
    fn height(&self)->u32;
}

pub struct Rectangle
{
   pub width:u32,
   pub height:u32,
}
impl Polygon for Rectangle
{
    fn area(&self)->u32
    {
        self.width * self.height
    }
    fn corners(&self)->i32 
    {
        2
    }
    fn can_hold<T :Polygon>(&self, other:&T)->bool
    {
        self.width > other.width() && self.height > other.height()
    }
    fn width(&self)->u32 {
        self.width
    }
    fn height(&self)->u32 {
        self.height
    }
}