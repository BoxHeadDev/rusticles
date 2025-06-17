# Rusticles 🦀 🧠

Rusticles is a personal learning project inspired by [Rustlings](https://rustlings.rust-lang.org/), designed to help reinforce your Rust knowledge through custom exercises and spaced repetition.

While Rustlings provides a solid introduction to Rust concepts, Rusticles extends this by adding new challenges and integrating a system that helps you retain what you've learned using spaced repetition techniques. Think of it as a code kata meets flashcards – tailored for your Rust journey.

--


🚀 Features

    🧩 Custom exercises not found in Rustlings

    📚 Exercises sourced and inspired by various Rust resources

    ⏳ Integrated spaced repetition scheduling system

    📅 Track which challenges need reviewing each day

    ✅ CLI workflow similar to Rustlings, so it feels familiar


--

🔧 Installation

Clone the repo and install the necessary components:
```bash
git clone https://github.com/your-username/rusticles.git
cd rusticles
cargo install --path .
```

--

📘 Getting Started

To begin practicing, run:
```bash
rusticles start
```
You’ll be shown the next exercises scheduled for today. If you’re new, you’ll start with a fresh set of initial challenges.

To manually browse and run specific exercises:
```bash
rusticles list     # Lists all exercises and their status
rusticles run N    # Run exercise N
```
To reset your progress:
```bash
rusticles reset 
```

--

📅 Spaced Repetition

Rusticles uses a simple spaced repetition algorithm (like SuperMemo or Anki) to schedule reviews:

- When you complete an exercise, you rate your recall.

- Based on your score, Rusticles reschedules that challenge.

- Each day, run rusticles start to see your review queue.

This method ensures that you're consistently reinforcing your knowledge without burning out.

--

📂 Project Structure

```bash
/exercises
    01_variables/
    02_functions/
    ...
/src
    main.rs       # CLI entry point
    scheduler.rs  # Spaced repetition logic
```
--

🌱 Contributing

This is primarily a personal learning project, but feel free to explore, fork, and build on top of it. If you have an interesting idea or challenge, contributions are welcome.

--

🙏 Acknowledgements

- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Rust Book](https://doc.rust-lang.org/stable/book/)

--

📜 License

MIT License. See LICENSE for details.
