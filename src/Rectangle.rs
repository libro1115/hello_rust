pub struct Rectangle
{
   pub  width:u32,
   pub height:u32,
}
impl Rectangle
{
    pub fn area(&self)->u32
    {
        self.width * self.height
    }
    pub fn can_hold(&self, other:&Rectangle)->bool
    {
        self.width > other.width && self.height > other.height
    }
    pub fn square(size:u32)->Rectangle
    {
        Rectangle { width: size, height: size }
    }
}