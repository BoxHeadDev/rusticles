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
        // TODO: Part A – spawn a new task to run concurrently
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        // TODO: Run a second loop in the main task
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // TODO: Part B – wait for the first task to finish
        handle.await.unwrap();

        // ✅ Expected output:
        // Lines from both loops interleaved
        // First task continues running after second task finishes
    });
}
