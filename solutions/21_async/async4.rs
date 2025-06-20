// 🦀 Rustlings Challenge: Async Spawn and Join
//
// In this challenge, you'll explore concurrent execution using `spawn_task` and `join`.
//
// Part A:
// 1. Spawn a new task using `trpl::spawn_task`.
// 2. Inside it, print numbers from 1 to 10, sleeping 500ms between each.
//
// 3. In the outer task, print numbers from 1 to 5, also sleeping 500ms between.
//
// Part B:
// 4. Use the `JoinHandle` from `spawn_task` to `.await` the inner task.
//    This ensures the first task completes before the program ends.
//
// BONUS:
// 5. Rewrite both loops as `async` blocks and run them with `trpl::join` instead.
//
// HINT: Use `std::time::Duration`.

use std::time::Duration;

fn main() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}
