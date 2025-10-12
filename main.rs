//! hello_world crate is just for lean out
/// # reatangle struct
/// ```rust
/// // create rectangle
/// let r = Rectangle::square(32)
/// // or
/// let r2 = Rectangle {
///     width:3,
///     height:4,
/// }
/// ```
///
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r = Rectangle {
        width: 3,
        height: 4,
    };
    let r2 = Rectangle {
        width: 2,
        height: 3,
    };
    let r3 = Rectangle::square(5);
    let a = r.area();
    println!(
        "Hello,world {:#?} \n {} \n r can hold r2?:{} \n r3 can hold r?:{}",
        r,
        a,
        r.can_hold(&r2),
        r3.can_hold(&r)
    );
}

#[test]
fn test() {
    println!("test")
}

#[test]
fn another_test() {
    panic!("just panic for test")
}
