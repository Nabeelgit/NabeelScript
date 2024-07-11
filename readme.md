# NabeelScript Interpreter

NabeelScript is a simple, interpreted programming language designed for educational purposes. This project implements a basic interpreter for NabeelScript using Rust.

## Features

- Basic arithmetic operations (+, -, *, /)
- Variable assignments
- Printing numbers and strings
- Support for comments

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
   ```

2. Run the interpreter:
   ```
   cargo run -- example.nabeel
   ```

## Syntax

### Comments
Comments start with `//` and continue to the end of the line.

### Print Statement
Use the `print` keyword followed by an expression to output values:
```
print 1 + 2 * 3;
print "Hello, World!";
```

### Variables
Assign values to variables using the `=` operator:
```
x = 10;
y = 20;
print x + y;
```

### Arithmetic Operations
NabeelScript supports basic arithmetic operations:
```
print 10 + 5;  // Addition
print 10 - 5;  // Subtraction
print 10 * 5;  // Multiplication
print 10 / 5;  // Division
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
