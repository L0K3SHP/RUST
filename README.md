# RUST
- Rust is a systems programming language focused on performance, reliability and safety.
- Desgined to provied low-level control over system resources while also prevenring comman programming error such as null or dangling pointers, buffer overflows and data races.
- Memory safety.
- Performance.
- Concurrency.
- Community.
- Expressiveness.

## First project 

### create project
```
 D:\RUST> cargo new my_rust_proj
     Created binary (application) `my_rust_proj` package
```

### run project
```
D:\RUST\my_rust_proj\src> cargo run 
```

### comments
- **// single line comment**
- **/\* Multi line comment \*/**

## Primitives (Scalar Types)

### Integers
- In Rust, integers are a primitive data type that represent whole numbers. They are used to represent numerical values in programs, and can be used in a variety of ways, such as for counting, indexing, and arithmetic operations.
- There are several integer types available in Rust, each with a different range of values they can represent. The most common integer types are:
  - i8, i16, i32, and i64: signed integers that can represent negative and positive values. The number in their names represents the number of bits they use to store their values.
  - u8, u16, u32, and u64: unsigned integers that can represent only positive values. The number in their names represents the number of bits they use to store their values.
- In addition to these types, Rust also has two more integer types:
  - isize and usize: signed and unsigned integers that can represent the size of a pointer on the current platform. On 32-bit platforms, they are 32 bits long, and on 64-bit platforms, they are 64 bits long.
- Integers in Rust can be created using literals, such as 42 or 0xff, or by using expressions and variables. Arithmetic operations such as addition, subtraction, multiplication, and division can be performed on integers, and there are also bitwise operations such as bitwise and (&), bitwise or (|), and bitwise not (!).
- It's important to note that when performing arithmetic operations on integers, Rust will check for overflow and panic at runtime if an overflow occurs. This is to prevent unexpected behavior and security vulnerabilities in programs. However, there are also ways to handle overflow situations explicitly, such as using the Wrapping struct to perform arithmetic operations with wrapping behavior instead of panicking.
```
fn main() {
   // Scalar types: int, float, loolean, char
   //Unsigned -  never negative - u8, u16, u32, u64, u128, usize
   //Signed -  can we negative and positive - i8, i16, i32, i64, i128, isize
 println!("Max size of u32: {}", u32::MAX);
 println!("Max size of u64: {}", u64::MAX);
 println!("Max size of i32: {}", i32::MAX);
 println!("Max size of i64: {}", i64::MAX);
 println!("Max size of usize: {}", usize::MAX);
 println!("Max size of isize: {}", isize::MAX);
}
```

### Floats
- In Rust, floats are a primitive data type that represent floating-point numbers. They are used to represent numerical values that have a fractional component or a very large or very small value.
- There are two main types of floating-point numbers available in Rust:
  - f32: a 32-bit floating-point number, which can represent values with a precision of roughly 7 decimal digits.
  - f64: a 64-bit floating-point number, which can represent values with a precision of roughly 16 decimal digits.
- Floats in Rust are stored using the IEEE 754 standard, which is also used by most other programming languages and hardware architectures.
- Floats in Rust can be created using literals, such as 3.14 or 2.0e-5, or by using expressions and variables. Arithmetic operations such as addition, subtraction, multiplication, and division can be performed on floats, and there are also special functions available for more complex operations, such as sqrt, sin, cos, and exp.
- It's important to note that floats in Rust, and in most other programming languages, have limited precision and can introduce rounding errors when performing calculations. This is due to the fact that floats are represented using a finite number of bits, and not all decimal values can be accurately represented in binary form. As a result, it's important to be aware of these limitations when working with floats and to use appropriate techniques to minimize rounding errors, such as rounding or using integer arithmetic when possible.
```
fn main() {
 //floats - f32, f64 
 println!("Max size of f32: {}", f32::MAX);
 println!("Max size of f64: {}", f64::MAX);
}
```

### Boolean
- In Rust, a boolean is a primitive data type that represents a logical value of either true or false. Booleans are used to represent conditions or to control the flow of a program based on a certain condition.
- Booleans in Rust are represented using the bool type. This type can only have two possible values: true or false. Booleans can be created using literals, such as true or false, or by using expressions and variables.
- Boolean expressions are used to create logical conditions in Rust. These expressions can be created using comparison operators such as == (equal to), != (not equal to), < (less than), > (greater than), <= (less than or equal to), and >= (greater than or equal to). Logical operators such as && (and), || (or), and ! (not) can also be used to create more complex boolean expressions.
- Boolean values are often used in Rust to control the flow of a program using conditional statements such as if and else.
- In addition to conditional statements, boolean values can also be used in loops, function returns, and other control structures to determine program behavior based on a logical condition.

### Characters
- In Rust, a char is a primitive data type that represents a single Unicode character. Unicode is a character encoding standard that allows characters from different writing systems, languages, and cultures to be represented using a common set of codes.
- In Rust, char values are represented using a 32-bit Unicode scalar value. This means that they can represent characters from the Basic Multilingual Plane (BMP) as well as supplementary characters that require more than one code point to represent.
- Chars in Rust can be created using literals, such as 'a', '👍', or '\u{1F44D}'. Chars can also be created from integer values using the std::char::from_u32 function, which returns an Option<char> value that represents the Unicode character corresponding to the given integer code point.
- Chars in Rust can be compared using operators such as == and !=, and can also be used in string operations and indexing operations. It's important to note that chars in Rust are not the same as strings, which are represented using the String or &str types.
- When working with char values in Rust, it's important to be aware of Unicode normalization issues, which can arise when comparing or manipulating Unicode characters that have equivalent but different code point sequences. Rust provides several functions in the std::char module to handle normalization and other Unicode-related operations on char values.
