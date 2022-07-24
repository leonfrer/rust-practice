fn main() {
    let r1 = Rectangle {
        width: 40,
        height: 70,
    };
    println!("rectangle r1: {:?}, area: {}", r1, r1.area());

    let r2 = Rectangle {
        width: 20,
        height: 30,
    };
    let r3 = Rectangle {
        width: 30,
        height: 10,
    };
    println!("r1 can hold r2: {}", r1.can_hold(&r2));
    println!("r2 can hold r3: {}", r2.can_hold(&r3));

    // associated function
    let sq = Rectangle::square(10);
    println!("sq: {:?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        self.width > other_rec.width && self.height > other_rec.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
