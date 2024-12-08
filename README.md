# Rust-Lox Interpreter

A Lox interpreter implemented entirely in Rust, following the principles and design outlined in [Crafting Interpreters](http://www.craftinginterpreters.com/) by Robert Nystrom. This project isn’t just an exercise in language implementation—it’s also an opportunity to deeply understand interpreters, improve Rust skills, and gain practical insights into language design and runtime behavior.

---

## Why This Interpreter?

1. *Educational Value:*  
   Implementing a language from scratch is one of the best ways to understand how programming languages actually work. By building each component—lexer, parser, interpreter—you gain a solid grasp of language design principles.

2. *Practical Rust Experience:*  
   Rust’s strict safety and ownership rules encourage clean, efficient, and bug-resistant code. Crafting an interpreter in Rust provides hands-on learning with advanced features like lifetimes, pattern matching, and error handling.

3. *Insights Into Language Runtime Behavior:*  
   Working through Lox gives you a glimpse into how scope resolution, variable declaration, function calls, and object-oriented features (like classes and inheritance) are managed at runtime.

4. *Flexibility for Experimentation:*  
   Once the core interpreter is complete, you can extend Lox with custom features or optimizations. Experimentation is straightforward since you fully control the codebase.

---

## Features

- *Full Lox Language Support:*  
  - Variables, functions, classes, inheritance.
  - If statements, loops (while, for).
  - Built-in functions and error handling.

- *Interactive REPL:*  
  - Test code snippets on-the-fly.
  - Rapidly explore language features without writing a full script.

- *File Execution:*  
  - Run complete .lox files for more extensive testing and projects.

- *Robust Error Handling:*  
  - Clear messages for syntax and runtime errors.
  - Graceful handling of unexpected inputs.

---

## Getting Started

1. *Prerequisites:*  
   - [Rust](https://www.rust-lang.org/tools/install) installed.

2. *Installation:*  
   bash
   git clone https://github.com/yourusername/rust-lox.git
   cd rust-lox
   cargo build --release
   

3. *Run the REPL:*  
   bash
   cargo run
   
   Enter Lox code directly into the prompt.

4. *Run a Script:*  
   bash
   cargo run -- path/to/script.lox
   

---

## Examples

*Hello World:*
lox
print "Hello, world!";


*Fibonacci:*
lox
fun fib(n) {
  if (n <= 1) return n;
  return fib(n - 1) + fib(n - 2);
}

print fib(10); // Outputs: 55


*Classes:*
lox
class Animal {
  speak() { print "I am an animal."; }
}

class Dog < Animal {
  speak() { print "I am a dog."; }
}

var dog = Dog();
dog.speak(); // Outputs: "I am a dog."


---

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request if you have suggestions, bug fixes, or new features you’d like to add. This project is meant to be a learning tool, so all improvements that help others understand interpreters and language design are appreciated.

---

## Acknowledgments

- *Robert Nystrom* and [Crafting Interpreters](http://www.craftinginterpreters.com/) for providing the blueprint for this project.
- The Rust community for maintaining excellent documentation and fostering a helpful environment for learning systems programming.

---

## License

This project is released under the [MIT License](LICENSE).
