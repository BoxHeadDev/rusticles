// 🦀 Rustlings Challenge: Closure Capture
//
// Your task is to finish implementing the `giveaway` method.
// If the user has a color preference, return it.
// If not, use the `.unwrap_or_else()` method with a closure that calls `self.most_stocked()`.
//
// HINTS:
// - The closure will need to capture `self`.
// - You don’t need to add parameters to the closure itself.
// - Use match or `unwrap_or_else`.

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // TODO: Return the preferred color or call `self.most_stocked()` using a closure
        todo!() // 🔴 This needs to be replaced
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    assert_eq!(giveaway1, ShirtColor::Red);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    assert_eq!(giveaway2, ShirtColor::Blue);
}
