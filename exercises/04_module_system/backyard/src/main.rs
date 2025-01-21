// TO DO: Import Asparagus
// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs

pub mod garden;

// TO DO: Bring Asparagus struct into scope
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
