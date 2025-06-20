// 🦀 Rustlings Challenge: The Tests Module and #[cfg(test)]
//
// 💡 This test module should only compile when running `cargo test`
// 🛠️ Run `cargo build` — the test code should not be compiled or included
// 🧪 Then run `cargo test` to ensure your test runs and passes

pub fn double(x: u32) -> u32 {
    x * 2
}

// TODO: Add #[cfg(test)] and write the test module and test function
mod tests {
    use super::*;

    #[test]
    fn doubles_properly() {
        assert_eq!(double(4), 8);
    }
}
