#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Refactor the following standalone functions into methods on the Rectangle struct.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn has_width(rectangle: &Rectangle) -> bool {
    rectangle.width > 0
}

fn set_width(rectangle: &mut Rectangle, width: u32) {
    rectangle.width = width;
}

fn max(rectangle: Rectangle, other: Rectangle) -> Rectangle {
    Rectangle {
        width: rectangle.width.max(other.width),
        height: rectangle.height.max(other.height),
    }
}

fn set_to_max(rectangle: &mut Rectangle, other: Rectangle) {
    *rectangle = max(*rectangle, other);
}

// Refactor to use the `area` method instead of the standalone function.
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

// Refactor this to use a method on the `Rectangle` struct.
fn struct_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if has_width(&rect1) {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// Implement the `can_hold` method for the Rectangle struct.
fn can_hold(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    rect1.width > rect2.width && rect1.height > rect2.height
}

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

    // Use the `can_hold` method to determine if rect1 can hold rect2 and rect3.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Refactor the following function into an associated function within the `Rectangle` struct.
fn struct_4() {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    let sq = square(3); // Use the refactored associated function here.
    println!("{:?}", sq); // Should print: Rectangle { width: 3, height: 3 }
}

// Rewrite the method calls in `struct_5` using their equivalent function syntax.
fn struct_5() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    let area1 = r.area(); // Rewrite this using the function syntax.
    r.set_width(2); // Rewrite this using the function syntax.
}

// Will the following compile? Why or why not?
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
    // Will `rect` still be usable here? Why or why not?
}

// Fix the following code so it compiles.
fn struct_7() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(0); // This method requires mutable access to `rect`.
}

// Fix the following code so it compiles.
fn struct_8() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(1);

    let rect_ref = &rect; // Hint: This should be a mutable reference to `rect`.
    rect_ref.set_width(2); // This method requires a mutable reference.
}

// Fix: Update the method call to work with the modified method.
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
    println!("{}", rect.area()); // Ensure rect can still be used here.
}

// Fix the following code so it compiles.
fn struct_10() {
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 0,
    };
    rect.set_to_max(other_rect); // Fix the method conflict.
}

// Refactor th `incr_v1` method to take a mutable reference instead of ownership.
struct Point(i32, i32);

impl Point {
    fn incr_v1(mut self) {
        self.0 += 1;
    }
}

fn struct_11() {
    let mut p = Point(0, 0);

    // Fix this method call to work with the refactored method.
    p.incr_x();
    println!("{}", p.0);
}

// Fix the mutable borrow error so the code compiles.
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // This method returns a mutable reference to `x`.
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

fn struct_12() {
    let mut p = Point { x: 1, y: 2 };

    let x = p.get_x(); // Mutable borrow of `p` for `x`
    *x += 1; // Update `x`
    println!("{} {}", *x, p.y); // Error: Cannot access `p.y` while `x` holds a mutable borrow.
}

fn main() {
    // Execute the function to see if your changes worked!
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
    struct_11();
    struct_12();
}
