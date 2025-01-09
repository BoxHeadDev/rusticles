#[derive(Debug)]
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

// Add the following functions as a methods on Rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn width(rectangle: &Rectangle) -> bool {
    rectangle.width > 0
}

fn can_hold(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    rect1.width > rect2.width && rect1.height > rect2.height
}

fn set_width(rectangle: &Rectangle, width: u32) {
    rectangle.width = width;
}

fn max(rectangle: Rectangle, other: Rectangle) -> Rectangle {
    Rectangle {
        width: rectangle.width.max(other.width),
        height: rectangle.height.max(other.height),
    }
}

fn set_to_max(&mut rectangle: Rectangle, other: Rectangle) {
    *rectangle = rectangle.max(other);
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
        area(&rect1)
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
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Replace method calls with function (syntatic sugar)
fn struct_5() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    r.set_width(2);
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

// Fix error
fn struct_7() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(0);
}

// Fix error
fn struct_8() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(1);

    let rect_ref = &rect;
    rect_ref.set_width(2);
}

// Fix error
fn struct_9() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect.max(other_rect);
    println!("{}", rect.area());
}

// Fix error
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

// Fix error
struct Point(i32, i32);
impl Point {
    fn incr_v1(mut self) {
        self.0 += 1;
    }
}

fn struct_11() {
    let mut p = Point(0, 0);
    p.incr_x();
    println!("{}", p.0);
}

// Fix error
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
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
}
