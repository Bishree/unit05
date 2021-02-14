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
}

fn main() {
    let rec1 =Rectangle {
        width: 30,
        height: 50,
    };
    println!(" the area of the rectangle is: {} \n \
                the lines lenth of regtnagle is: {}",
                    rec1.area(), rec1.circul())

}