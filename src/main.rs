#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area (&self) -> u32{
        self.width * self.height
    }
    fn circul (&self) -> u32{
        self.width*2 + self.height*2
    }
    fn can_hold (&self, other:&Rectangle) -> bool {
        self.width>other.width && self.height>other.height
    }
}

fn main() {
    let rec1 =Rectangle {
        width: 30,
        height: 50,
    };
    let rec2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rec3 = Rectangle{
        width: 60,
        height: 45,
    };
    println!(" the area of the rectangle is: {} \n \
                the lines lenth of regtnagle is: {}",
             rec1.area(), rec1.circul());
    println!("can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("can rec1 hold rec3? {}", rec1.can_hold(&rec3));
}