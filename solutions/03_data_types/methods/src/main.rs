#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    struct_1();
    struct_2();
    struct_3();
    struct_4();
    struct_5();
    struct_6();
    struct_7();
    struct_8();
    struct_9();
    struct_10();
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn max(&self, other: &Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h,
        }
    }
    fn set_to_max(&mut self, other: &Rectangle) {
        *self = self.max(other);
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Update the following to use methods
fn struct_1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn struct_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if width(&rect1) {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// Implement the can_hold method
fn struct_3() {
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Implement square as an Associated Function (static method)
fn struct_4() {
    let sq = Rectangle::square(3);
}

// Replace method calls with function (syntatic sugar)
fn struct_5() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = Rectangle::area(&r);
    Rectangle::set_width(&mut r, 2);
}

//
fn struct_6() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect.max(other_rect);
}

// rect needs to be mutable
fn struct_7() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(0);
}

// Create a mutable reference to rect
fn struct_8() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(1);

    let rect_ref = &mut rect;
    rect_ref.set_width(2);
}

// Updated max to take an immutable reference
fn struct_9() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect.max(&other_rect); // Pass a reference to other_rect
    println!("{}", rect.area());
}

// The max method borrows self but takes ownership of other, which makes it incompatible with set_to_max, where self is mutably borrowed and attempts to pass other to max
fn struct_10() {
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 0,
    };
    rect.set_to_max(other_rect);
}

// mutable pointer is more idiomatic (doesn't take ownership)
struct Point(i32, i32);
impl Point {
    fn incr_v1(&mut self) {
        self.0 += 1;
    }
}

fn struct_11() {
    let mut p = Point(0, 0);
    p.incr_x();
    println!("{}", p.0);
}

// x held onto the mutable reference
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

fn struct_12() {
    let mut p = Point { x: 1, y: 2 };
    {
        let x = p.get_x(); // Mutable borrow of `p` for `x`
        *x += 1; // Update `x`
    } // Mutable borrow of `p` ends here
    println!("{} {}", p.x, p.y); // Now you can access `p.y` safely
}
