// Will the following compile? Why or why not?
fn main() {
    let mut h = HashMap::new();
    h.insert("k1", 0);

    let v1 = &h["k1"]; // Immutable reference to h["k1"]

    // Hint: This causes an issue because h is being mutated while v1 is still in use.
    h.insert("k2", 1); // Mutable borrow of h occurs here.

    let v2 = &h["k2"]; // Immutable reference to h["k2"]
    println!("{} {}", v1, v2);
}
