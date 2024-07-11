# NabeelScript Interpreter

NabeelScript is a simple, interpreted programming language designed for educational purposes. This project implements a basic interpreter for NabeelScript using Rust.

## Features

- Basic arithmetic operations (+, -, *, /)
- Variable assignments
- Printing numbers, strings, booleans, and arrays
- Support for comments
- Boolean operations (==, !=, <, >, <=, >=, &&, ||, !)
- Array operations (index access, split, join, count)

## Getting Started

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/nabeelscript.git
   cd nabeelscript
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Usage

1. Create a `.nabeel` file with your NabeelScript code. For example, `example.nabeel`:

   ```
   // This is a comment
   print 1 + 2 * 3; // This will print 7

   // Printing a string
   print "Hello, World!";

   // Variable assignment and printing
   x = 10;
   y = 20;
   print x + y; // This will print 30

   // Combined operations
   a = 5;
   b = 10;
   c = a * b + (a - b) / 2;
   print c; // This will print 45

   // Another string example
   print "NabeelScript is fun!";

   // Boolean operations
   print true;
   print false;
   print 5 > 3;
   print 10 == 10;
   print true && false;
   print true || false;
   print !true;
   x = 15;
   y = 20;
   print x < y && y > 10;

   // Arrays
   arr = [1, 2, 3, 4, 5];
   print arr[2]; // Outputs: 3

   sentence = "Hello world";
   words = split(sentence, " ");
   print words[1]; // Outputs: world
   ```

2. Run the interpreter:
   ```
   cargo run -- example.nabeel
   ```

## Documentation

For detailed documentation of NabeelScript, please visit our [online documentation](https://nabeelgit.github.io/NabeelScript/).

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.