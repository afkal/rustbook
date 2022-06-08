#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//struct method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // bigger can hold smaller -> note no if sentence needed!!!
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    // associative function - not a method since do not consume &self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

}

fn main() {
    // initial implementation with ints
    /*
    let width1 = 30;
    let height1 = 50;
    */
    // improved implementation with tuples
    // let rect1 = (30, 50);
    // even better implementation with struct

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

    let _sq = Rectangle::square(3);

    println!("rect2 is {:#?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Was before...
// tuple
/*
fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
*/

//struct function
/*
fn area(rect: &Rectangle) -> u32 {
    dbg!(rect.height * rect.width)
}
*/


