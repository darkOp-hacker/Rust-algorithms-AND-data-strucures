# Rust Algorithms & Data Structures 🦀📚

A collection of **algorithms and data structures implemented in Rust**, designed for learning, experimentation, and hands-on practice.

---

## 🚀 Features

- Classic algorithms: **Bubble Sort, Merge Sort, Quick Sort, etc.**
- Data structures: **Stacks, Queues, Linked Lists, Trees, Graphs**
- Fully implemented in **Rust**, emphasizing safety, performance, and idiomatic code.
- Includes **pseudocode** for easier understanding.
- Great for **students, developers, and Rust enthusiasts** wanting to strengthen problem-solving skills.

---

## 📂 Project Structure

src/
├── sorting/
│ ├── bubble-sort.rs
│ ├── bubble-sort-pseudocode.md
│ ├── merge_sort.rs
│ └── Merge-Sort-Pseudocode.md
├── main.rs
└── (other modules)

rust
Copy
Edit

- `sorting/` – contains sorting algorithms and their pseudocode.
- `main.rs` – entry point for testing algorithms.

---

## 📝 Example Usage

```rust
mod merge_sort;
mod bubble_sort;

fn main() {
    let mut numbers = vec![5, 3, 8, 1, 2];

    // Bubble Sort
    let sorted_bubble = bubble_sort::bubble_sort(&mut numbers.clone());
    println!("Bubble Sort: {:?}", sorted_bubble);

    // Merge Sort
    let sorted_merge = merge_sort::merge_sort(numbers.clone());
    println!("Merge Sort: {:?}", sorted_merge);
}
📖 Learning Goals
Understand core sorting and searching algorithms.

Learn algorithmic thinking and problem-solving in Rust.

Gain experience with Rust data structures and memory safety.

Connect pseudocode to actual Rust implementations.

⚡ Contributing
Contributions are welcome! Feel free to:

Add new algorithms or data structures

Optimize existing implementations

Improve pseudocode or documentation

Please fork the repo, make changes, and submit a pull request.

🛠 Tech Stack
Rust – primary language for all algorithms and data structures
