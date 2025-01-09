struct YoRectangle {
    width: u32,
    height: u32,
}

impl YoRectangle {
    fn area(&self) -> u32 {
        self.width * self.height // calculate the area
    }
}

pub fn main() {
    let rect = YoRectangle {
        width: 10,
        height: 5,
    };
    println!("Rectangle area: {}", rect.area()) // expected: 50
}
