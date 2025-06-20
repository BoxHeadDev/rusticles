// 🦀 Rustlings Challenge: Iterator Filter with Closure
//
// This challenge is about using closures that capture variables from their environment.
// Your goal is to complete the function `shirts_in_color` so it returns only
// shirts matching the specified color.
//
// HINTS:
// - Use `.into_iter()` to consume the input vector
// - Use `.filter()` with a closure that captures `color`
// - Use `.collect()` to turn the filtered results into a Vec

#[derive(PartialEq, Debug)]
struct Shirt {
    color: String,
    size: u32,
}

fn shirts_in_color(shirts: Vec<Shirt>, color: String) -> Vec<Shirt> {
    shirts
        .into_iter()
        .filter(|s| s.color == color) // ✅ closure captures `color`
        .collect() // ✅ collect filtered items into a Vec<Shirt>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_color() {
        let shirts = vec![
            Shirt {
                color: String::from("blue"),
                size: 38,
            },
            Shirt {
                color: String::from("red"),
                size: 40,
            },
            Shirt {
                color: String::from("blue"),
                size: 42,
            },
        ];

        let blue_shirts = shirts_in_color(shirts, String::from("blue"));

        assert_eq!(
            blue_shirts,
            vec![
                Shirt {
                    color: String::from("blue"),
                    size: 38
                },
                Shirt {
                    color: String::from("blue"),
                    size: 42
                },
            ]
        );
    }
}
