#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if the width is greater than 0.
    fn has_width(&self) -> bool {
        self.width > 0
    }

    // Method to update the width of the rectangle.
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Method to return the rectangle with the maximum dimensions.
    fn max(&self, other: &Self) -> Self {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // Method to update the rectangle to have the maximum dimensions.
    fn set_to_max(&mut self, other: &Rectangle) {
        *self = self.max(other);
    }
}
// Context: Structs in Rust can have associated methods implemented via the impl block. These methods allow you to encapsulate functionality related to the struct, making your code more readable and idiomatic. Methods can take &self, &mut self, or even consume self, depending on the desired behaviour.

// Use the `area` method on the `Rectangle` struct.
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
// Context: When working with structs, it's more idiomatic in Rust to define methods within an impl block rather than relying on standalone functions. Methods allow you to directly operate on an instance of the struct, making your code cleaner and more intuitive.

// Use the `width` method instead of a standalone function.
fn struct_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.has_width() {
        println!("The rectangle has a width; it is {}", rect1.width);
    }
}
// Context: Structs can have methods that encapsulate functionality related to their fields. In Rust, using methods instead of accessing fields directly or relying on standalone functions makes the code more concise, modular, and intuitive.

// Solution:
impl Rectangle {
    // Method to determine if this rectangle can hold another rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
// Context: Struct methods in Rust allow you to add behaviour directly to your types. In this challenge, you’ll implement a can_hold method for the Rectangle struct. This method will determine if one rectangle can entirely fit inside another.

// Solution: Associated function to create a square rectangle.
impl Rectangle {
    // Associated function to create a square.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn struct_4() {
    // Use the associated function to create a square.
    let sq = Rectangle::square(3);
    println!("{:?}", sq); // Should print: Rectangle { width: 3, height: 3 }
}
// Context: Associated functions, often called "static methods," are functions defined within an impl block but do not operate on an instance of the struct. Instead, they are called directly on the struct itself.

// Solution:
fn struct_5() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    // Desugared method call for `area`.
    let area1 = Rectangle::area(&r);

    // Desugared method call for `set_width`.
    Rectangle::set_width(&mut r, 2);
}
// Context: In Rust, method calls like r.area() are syntactic sugar for Rectangle::area(&r). Similarly, for methods that require mutable access, r.set_width(2) is shorthand for Rectangle::set_width(&mut r, 2). This desugared form highlights how methods are resolved internally by the compiler.

// Solution:
fn struct_6() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", rect.area()); // This works because `area` borrows `rect`.

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(&other_rect); // `rect` and `other_rect` are consumed here.

    // `rect` is no longer usable here because it was moved into the `max` method.
    // Uncommenting the line below will cause a compile error.
    // println!("{}", rect.area());
}
// Context: In Rust, methods can borrow or take ownership of a struct instance. When a method takes ownership (e.g., self instead of &self), the instance is consumed and cannot be used afterward. If a method borrows the instance (e.g., &self or &mut self), it allows the instance to remain accessible after the method call.

// Solution: Mark `rect` as mutable to allow calling the `set_width` method.
fn struct_7() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(0); // Now this works because `rect` is mutable.
}
// Context: In Rust, methods that modify a struct instance require mutable access. If the struct instance is not declared as mut, you’ll encounter a compile-time error. This ensures that modifications to a struct instance are explicit and safe.

// Solution: Use a mutable reference for calling `set_width` through a reference.
fn struct_8() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.set_width(1);

    let rect_ref = &mut rect; // Create a mutable reference to `rect`.
    rect_ref.set_width(2); // Now this works because `rect_ref` is mutable.
}
// Context: In Rust, you can create references to variables for borrowing. If a method requires a mutable reference (e.g., &mut self), you must create a mutable reference to the variable. Furthermore, you cannot have both mutable and immutable references to the same variable at the same time.

// Solution:
// Updated max method to take an immutable reference to `other`.
fn struct_9() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    // Pass an immutable reference to `other_rect` to the `max` method.
    let max_rect = rect.max(&other_rect);
    println!("{}", rect.area()); // Works because `rect` is borrowed immutably.
}
// Context: In Rust, methods can take ownership of their parameters, borrow them immutably (&), or borrow them mutably (&mut). By using references, you can avoid consuming variables, allowing them to remain usable after the method call.

// Solution:
// Update `max` and `set_to_max` to borrow `other` instead of taking ownership.
fn struct_10() {
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 0,
    };
    rect.set_to_max(&other_rect); // Pass a reference to `other_rect` instead of moving it.
    println!("{:?}", rect); // Outputs: Rectangle { width: 1, height: 1 }
}
// Context: In Rust, methods that borrow (&self) or mutably borrow (&mut self) can conflict with methods that take ownership (self) of a parameter. When chaining or combining methods, these conflicts can lead to compile-time errors. This is because Rust enforces strict borrowing rules to ensure memory safety.

// Solution: mutable pointer is more idiomatic (doesn't take ownership)
struct Point(i32, i32);

impl Point {
    // Refactor to use a mutable reference.
    fn incr_v1(&mut self) {
        self.0 += 1;
    }
}

fn struct_11() {
    let mut p = Point(0, 0);

    // Call the refactored method, which now uses a mutable reference.
    p.incr_v1();
    println!("{}", p.0); // Outputs: 1
}
// Context: In Rust, methods that modify the state of a struct should use mutable references (&mut self) instead of taking ownership (self). This allows the caller to retain ownership of the struct while enabling in-place modifications.

// Solution:
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    // This method returns a mutable reference to `x`.
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

fn struct_12() {
    let mut p = Point2 { x: 1, y: 2 };

    {
        let x = p.get_x(); // Mutable borrow of `p` for `x`
        *x += 1; // Update `x`
    } // Mutable borrow of `p` ends here

    println!("{} {}", p.x, p.y); // Now you can safely access `p.y`
}
// Context: In Rust, you cannot borrow a struct mutably and then access it while the mutable borrow is still active. This is to prevent undefined behaviour caused by simultaneous access and mutation. To fix such errors, you need to ensure that the mutable borrow’s lifetime ends before accessing other parts of the struct.

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
