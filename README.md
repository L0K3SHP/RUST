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
   //Signed -  can we negative and positive - i8, i16, i32 (default), i64, i128, isize
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
 //floats - f32, f64 (default)
 println!("Max size of f32: {}", f32::MAX);
 println!("Max size of f64: {}", f64::MAX);
}
```

### Boolean
- In Rust, a boolean is a primitive data type that represents a logical value of either true or false. Booleans are used to represent conditions or to control the flow of a program based on a certain condition.
- Booleans in Rust are represented using the **bool** type. This type can only have two possible values: true or false. Booleans can be created using literals, such as true or false, or by using expressions and variables.
- Boolean expressions are used to create logical conditions in Rust. These expressions can be created using comparison operators such as == (equal to), != (not equal to), < (less than), > (greater than), <= (less than or equal to), and >= (greater than or equal to). Logical operators such as && (and), || (or), and ! (not) can also be used to create more complex boolean expressions.
- Boolean values are often used in Rust to control the flow of a program using conditional statements such as if and else.
- In addition to conditional statements, boolean values can also be used in loops, function returns, and other control structures to determine program behavior based on a logical condition.

### Characters
- In Rust, a char is a primitive data type that represents a single Unicode character. Unicode is a character encoding standard that allows characters from different writing systems, languages, and cultures to be represented using a common set of codes.
- In Rust, char values are represented using a 32-bit / 4-bytes Unicode scalar value. This means that they can represent characters from the Basic Multilingual Plane (BMP) as well as supplementary characters that require more than one code point to represent.
- Chars in Rust can be created using literals, such as 'a', '👍', or '\u{1F44D}'. Chars can also be created from integer values using the std::char::from_u32 function, which returns an Option<char> value that represents the Unicode character corresponding to the given integer code point.
- Chars in Rust can be compared using operators such as == and !=, and can also be used in string operations and indexing operations. It's important to note that chars in Rust are not the same as strings, which are represented using the String or &str types.
- When working with char values in Rust, it's important to be aware of Unicode normalization issues, which can arise when comparing or manipulating Unicode characters that have equivalent but different code point sequences. Rust provides several functions in the std::char module to handle normalization and other Unicode-related operations on char values.

## Variables
### Types of variable
- In Rust, there are three keywords used to declare variables: let, let mut, and const. These keywords have different meanings and are used to declare variables with different properties.
- **let** is used to declare an immutable variable, which means that its value cannot be changed after it has been assigned. For example:```let x = 42;```In this code, x is an immutable variable that has been assigned the value 42. If we try to reassign x to a different value, Rust will give us an error.  
- **let mut** is used to declare a mutable variable, which means that its value can be changed after it has been assigned. For example:```let mut x = 42; x = 43;```In this code, x is a mutable variable that has been assigned the value 42. We can then reassign x to a different value without any errors.
- **const** is used to declare a constant variable, which means that its value cannot be changed after. it has been assigned. Constants are always immutable, and their values must be known at compile time. For example:```const PI: f32 = 3.14159;```In this code, PI is a constant variable that has been assigned the value 3.14159. We cannot change the value of PI at runtime.
- It's important to choose the appropriate variable type for a given use case in Rust. If a variable does not need to be modified after it has been assigned, it should be declared as an immutable variable using let. If a variable needs to be modified after it has been assigned, it should be declared as a mutable variable using let mut. If a value is known at compile time and will not change during program execution, it should be declared as a constant using const.
```
// Constants
/*  
It work as global variable and faster and immutable
synatx : const SCREENCASE: variable_type = value
 */
const ABC: i32 = 17; 

fn main() {
    // variable are immutable
   let hello = "Hello Word";
   println!("{}",hello);

   // muatble eg 
   let mut hello1 = "I am hello";
   println!("hello1 before mutable: {}",hello1);
   hello1 = "I am mutable hello";
   println!("hello1 after mutable:{}", hello1);

    // eg 3
    let x = 5;
    let y = 6;
    println!("Math in rust {} + {}  = {}",x,y,x+y);

    // Constants
    println!("{}",ABC);
}
```

### Scope and Shadowing
- In Rust, scope refers to the region of a program where a variable is valid and accessible. Variables declared inside a block or a function have a limited scope and are only accessible within that block or function, while variables declared outside of any block or function have a global scope and are accessible throughout the program.
- For example, consider the following code:
```
let x = 42;
{
  let y = 43;
  println!("x: {}, y: {}", x, y); 
}
println!("x: {}", x);
```
- In this code, x is declared outside of any block and has a global scope, while y is declared inside a block and has a limited scope. The first println! statement inside the block can access both x and y, while the second println! statement outside the block can only access x.
- Shadowing is a feature in Rust that allows you to declare a new variable with the same name as an existing variable in the same scope. This can be useful for changing the type or value of a variable without having to use a different name.
- For example, consider the following code:
```let x = 42;
let x = "hello";
println!("x: {}", x);
 ```
- In this code, x is first declared as an integer, and then shadowed by a new declaration that assigns a string value to it. The final println! statement will print the string value of x.
- Shadowing can also be used to convert a mutable variable into an immutable variable. For example:
```
let mut x = 42;
x = 43;
let x = x;
println!("x: {}", x);
```
- In this code, x is first declared as a mutable variable and assigned the value 42. It is then reassigned the value 43. Finally, x is shadowed by a new declaration that makes it immutable and assigns it the current value of x. The final println! statement will print the value 43.
- Shadowing is a powerful feature in Rust, but it can also be confusing if used excessively or without a clear purpose. It's important to use shadowing judiciously and to choose descriptive variable names to avoid name collisions and improve code readability.

## Primitive (Compund Types)

### Tuples
- In Rust, a tuple is a fixed-size collection of elements of different types. Tuples are used to group related data together and provide a way to define custom types with specific behavior and properties.
- Tuples are declared using parentheses ( and ), and the elements are separated by commas ,. For example:```let person = ("John", 42, 1.72);```. In this code, person is a tuple that contains three elements: a string, an integer, and a float.
- Tuples in Rust can be accessed using pattern matching. For example:
```
let person = ("John", 42, 1.72);
let (name, age, height) = person;
println!("Name: {}, Age: {}, Height: {}", name, age, height);
```
- In this code, the let statement uses pattern matching to extract the three elements of the person tuple into separate variables: name, age, and height. These variables are then used to print out the values of the tuple elements.
- Tuples in Rust can also be indexed using dot notation, starting from zero. For example:
```
let person = ("John", 42, 1.72);
println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);
```
- In this code, the dot notation is used to access the first, second, and third elements of the person tuple, respectively.
- Tuples in Rust can contain any combination of variable types, as long as their number is fixed at compile time. Tuples are often used to return multiple values from a function, or to group related data together in a convenient and expressive way.
```
fn main() {
    // COmpound types -Tuples % Arrays
   // Tuples idicated - ()
    // Max value of tuple - 12

    let stu_a = ("Lokesh",'A',7.48);
    
    // Type 1 by indexing
    let name_stu_a = stu_a.0;
    let grade_stu_a = stu_a.1;
    let gpa_stu_a = stu_a.2;
    println!("My name is {}, my class garde is {} and overall gpa is {}", name_stu_a, grade_stu_a,gpa_stu_a);

    // Type 2 
    let (name_stu_a1, grade_stu_a1, gpa_stu_a1) = stu_a;
    println!("My name is {}, my class garde is {} and overall gpa is {}", name_stu_a1, grade_stu_a1,gpa_stu_a1);
  }
```

### Arrays
- In Rust, an array is a fixed-size collection of elements of the same type. Arrays are used to store multiple values of the same type in a contiguous memory block and provide a way to access these values using a single variable name.
- Arrays are declared using square brackets [ and ], and the elements are separated by commas ,. The length of an array is part of its type, and must be known at compile time. For example:```let numbers = [1, 2, 3, 4, 5]; ```In this code, numbers is an array that contains five integer elements.
- Arrays in Rust can be accessed using indexing notation, starting from zero. For example:
```
let numbers = [1, 2, 3, 4, 5];
println!("The first number is {}", numbers[0]);
```
- In this code, the indexing notation is used to access the first element of the numbers array.
- Arrays in Rust can also be iterated using a for loop. For example:
```
let numbers = [1, 2, 3, 4, 5];
for number in numbers.iter() { println!("{}", number); }
```
- In this code, the for loop is used to iterate over the elements of the numbers array using the iter() method.
- It's important to note that arrays in Rust have a fixed size that cannot be changed at runtime. If you need a collection that can grow or shrink in size, you should use a Vec (vector) instead of an array. Rust arrays are often used to represent fixed-size data structures, such as matrices or game boards, or to pass a fixed number of values between functions or threads.
```
fn main() {
    // COmpound types -Tuples % Arrays
    // Array idicated - []
    // Max value of Array  - 32 - similar data type

    let stu_a = ["A", "B", "C"];

    println!("First student is {}", stu_a[0]);
    println!("2nd student is {}", stu_a[1]);
    println!("3rd student is {}", stu_a[2]);
  }
```

### Slices 
- In Rust, slices are a lightweight view into a contiguous sequence of elements in a collection, such as an array or a vector. Slices provide a way to access a subset of the elements in a collection without creating a new collection.
- Slices are denoted using a range of indices enclosed in square brackets, such as [1..3] or [..5]. A slice can be created from an existing collection by specifying a range of indices:
```
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3];
println!("{:?}", slice); // prints "[2, 3]"
```
- In this code, a slice is created from the arr array using the range of indices 1..3, which selects the second and third elements of the array. The resulting slice is stored in the slice variable, which is then printed to the console.
- Slices can be used to pass a subset of elements in a collection to a function:
```
fn sum(slice: &[i32]) -> i32 {
  let mut total = 0;
  for &x in slice {
    total += x;
  }
  total
}

let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3];
let total = sum(slice);
println!("The sum is {}", total); // prints "The sum is 5"
```
- In this code, a function named sum takes a slice of i32 elements as a parameter and returns the sum of its elements. A slice is created from the arr array using the range of indices 1..3, and the sum function is called with the slice as an argument. The resulting sum is then printed to the console.
- Slices can also be used to modify a subset of elements in a collection:
```
let mut arr = [1, 2, 3, 4, 5];
let slice = &mut arr[1..3];
slice[0] = 6;
slice[1] = 7;
println!("{:?}", arr); // prints "[1, 6, 7, 4, 5]"
```
- In this code, a mutable slice is created from the arr array using the range of indices 1..3, which selects the second and third elements of the array. The values of these elements are then changed to 6 and 7, respectively. The resulting array is then printed to the console.
- Overall, slices in Rust are a lightweight and powerful tool for accessing and manipulating a subset of elements in a collection without creating a new collection. Slices can be created from arrays, vectors, and other collections, and can be used to pass subsets of elements to functions or modify a subset of elements in a collection.
```
fn main() {
   // ':?' - Debug Formate

   let mut abc = [1,2,3,4,5,6];
   let slice = & mut abc[0..3];
   println!("{:?}",slice);

   // replace data

   slice[0] = 10;
   slice[1] = 20;

   println!("{:?}",slice);
  }
```

## Strings
-In Rust, strings are represented using three main types: str, &str, and String.
- str is a primitive type that represents a sequence of Unicode scalar values. It is often used as a slice type to represent borrowed string data. For example, a function that takes a &str argument can accept either a string slice or a string literal.
- &str is a string slice type that represents a borrowed view into a sequence of Unicode scalar values. It is a read-only type that cannot be modified. String slices are often used to pass string data between functions or to extract substrings from a larger string. For example:
```
fn main() {
  let message = "hello, world!";
  let first_word = &message[0..5];
  println!("The first word is {}", first_word);
}
```
- In this code, message is a string literal that is assigned to a variable. The & symbol is used to create a string slice that represents the first five characters of the message string. This string slice is then printed using the println! macro.
- String is a dynamically-sized string type that represents a sequence of Unicode scalar values. It is a mutable type that can be modified by appending, inserting, or removing characters. Strings are often used to represent user input, file data, or other dynamic text content. For example:
```
fn main() {
  let mut message = String::from("hello");
  message.push_str(", world!");
  println!("{}", message);
}
```
- In this code, a new String is created using the String::from function, which takes a string literal as an argument. The push_str method is then used to append another string to the end of the message string. The final println! statement prints the modified message string.
- It's important to note that String and &str are different types that represent different kinds of string data. String is a dynamically-sized string that is owned by the program, while &str is a borrowed view into a string that is owned by another variable or data structure. Understanding the differences between these types and how to use them correctly is an important part of writing Rust programs that handle string data
```
fn main() {
    // STRINGS - Their are several types
    // Likely only use two: String and str/&str
    //str - string slice, &str - borrowed sttring slice- cannot be modified
    // String - data can be modified
    // &str - Essentially a subset of a string

    let greeting = "Hello their";

    let mut name = "adsdadadwqa";

    // String eg 1
    let mut name1 = String::new();
    name1.push_str("Test");
    println!("{}",name1);

    // String eg 2
    let name2 = "asdsd".to_string();
    println!("{}",name2);

    // String eg 3
    let name3 = String::from("awsd");
    println!("{}",name3);
```

### Escaping
- In Rust, character escaping is used to represent special characters in string literals and character literals. A backslash \ is used as an escape character to indicate that the following character has a special meaning.
- The most common escape sequences in Rust are:
```
\\: backslash
\": double quote
\': single quote
\n: newline
\r: carriage return
\t: tab
\0: null character
\xNN: hexadecimal escape sequence, where NN is a two-digit hexadecimal number that represents a Unicode scalar value
\u{NNNN}: Unicode escape sequence, where NNNN is a four-digit hexadecimal number that represents a Unicode scalar value
For example:

let message = "Hello, \"world\"!\n";
println!("{}", message);
```
- In this code, the string literal "Hello, \"world\"!\n" contains two escaped characters: \" to represent a double quote character, and \n to represent a newline character.
- Similarly, character literals can be escaped using the same backslash notation. For example:
```
let heart = '\u{2764}';
println!("{}", heart);
```
- In this code, the \u{2764} escape sequence is used to represent the Unicode scalar value for a heart symbol.
- It's important to properly escape special characters in Rust string and character literals to avoid compilation errors and unexpected behavior. Rust also provides raw string literals, which can be used to include backslashes and special characters without escaping them. Raw string literals are declared using the r#"..."# syntax, where the # characters can be replaced with any other character that does not appear inside the string. For example:
```
let message = r#"Hello, "world"!\n"#;
println!("{}", message);
``` 
- In this code, the raw string literal r#"Hello, "world"!\n"# includes the double quote and newline characters without escaping them.

## Modules and Libraries
- In Rust, modules are used to organize code into groups and provide a way to control visibility and access to different parts of a program. Modules can contain other modules, as well as functions, structs, enums, and other items.
- Modules are declared using the mod keyword, followed by the name of the module and its contents inside curly braces {}. For example:
```
mod my_module {
  fn hello() {
    println!("Hello, world!");
  }
}
```
- In this code, a new module named my_module is declared with a single function hello().
- Modules in Rust are used to control visibility and access to different parts of a program using the pub keyword. Items declared inside a module are private by default and can only be accessed from within the same module. To make an item public, the pub keyword is used. For example:
```
mod my_module { 
  pub fn hello() {
    println!("Hello, world!");
  }
}
```
- In this code, the hello() function is declared as public using the pub keyword. This means that it can be accessed from outside the my_module module.
- Libraries in Rust are collections of code that can be used by other programs or libraries. Rust libraries are compiled into a single file with a .rlib extension and can be used by including them in the Cargo.toml file of another Rust project. For example:
```
[dependencies]
my_library = { version = "1.0", path = "../path/to/my_library" }
```
- In this code, a dependency named my_library is added to the dependencies section of the Cargo.toml file. The version parameter specifies the version of the library to use, and the path parameter specifies the path to the directory containing the library.
- Libraries in Rust can contain one or more modules, as well as other items such as structs, enums, and traits. The pub keyword is used to make items in a library public and accessible to other programs and libraries.
- Overall, modules and libraries in Rust are powerful tools for organizing and sharing code, and they play an important role in the Rust ecosystem.

### User Input 
- In Rust, user input can be obtained from the command line or from the standard input stream (stdin). The std::io module provides functions and types for reading input from these sources.
- To read user input from the command line, the std::env module can be used to access the command line arguments passed to the program. For example:
```
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("The first argument is {}", args[1]);
}
```
- In this code, the env::args() function is used to obtain a vector of command line arguments passed to the program. The first argument is accessed using indexing notation and printed using the println! macro.
- To read user input from the standard input stream, the std::io::stdin() function can be used to obtain a handle to the input stream. The std::io::BufRead trait provides functions for reading lines of input from this stream. For example:
```
use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    println!("{}", line.unwrap());
  }
}
```
- In this code, the io::stdin() function is used to obtain a handle to the standard input stream. The stdin.lock() method is then used to obtain a locked reference to the input stream, which can be used to read lines of input using the lines() method. Each line of input is printed using the println! macro.
- It's important to properly handle user input in Rust to avoid errors and security vulnerabilities. User input should be validated and sanitized to prevent unexpected behavior or malicious attacks. Rust provides several libraries and functions for handling user input in a safe and secure way, such as the std::io::BufReader and std::io::Read traits, the regex crate for regular expressions, and the serde crate for serializing and deserializing data.
```
#![allow(unused)]
// This is comments

use std::io;
// user std::*;

fn main() {
    println!("Who are you?");
    let mut name = String::new();
    io::stdin().read_line( &mut name);
    println!("Welcome {}",name.trim_end());
  }
```

## Math
- In Rust, math operators are used to perform arithmetic operations on numerical values. Rust supports the standard math operators, including addition, subtraction, multiplication, division, and remainder, as well as bitwise operators, shifting operators, and comparison operators.
- The standard math operators in Rust are:
```
+: addition
-: subtraction
*: multiplication
/: division
%: remainder
```
- For example:
```
let x = 10;
let y = 3;
let sum = x + y;
let diff = x - y;
let product = x * y;
let quotient = x / y;
let remainder = x % y;
```
- In this code, the +, -, *, /, and % operators are used to perform arithmetic operations on the x and y variables. The results of these operations are stored in new variables named sum, diff, product, quotient, and remainder, respectively.
```
fn main() {
    let x = 15;
    let y = 4;
    let x_float = x as f64;
    let y_float = y as f64;

    println!("{} + {} = {}", x,y,x+y);
    println!("{} - {} = {}", x,y,x-y); 
    println!("{} * {} = {}", x,y,x*y);
    println!("{} / {} = {}", x,y,x_float/y_float);
    println!("{} % {} = {}", x,y,x%y);
    println!("{}^{} = {}", x,y,i32::pow(x,y.try_into().unwrap()));
  }
```

## Dependencies
- In Rust, dependencies are external packages or libraries that are used by a Rust project. Dependencies can be added to a Rust project using the Cargo.toml file and the Cargo package manager.
- The Cargo.toml file is a configuration file that defines the metadata and dependencies of a Rust project. The dependencies section of the Cargo.toml file is used to list the external packages or libraries that the project depends on. For example:
```
[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
```
- In this code, the dependencies section lists two external packages: rand and serde. The rand package is specified with a version number of 0.8, while the serde package is specified with a version number of 1.0 and a feature flag of derive.
- To add a new dependency to a Rust project, the package name and version number can be added to the dependencies section of the Cargo.toml file. The package version number can be specified as an exact version number, a version range, or a wildcard. For example:
```
"0.8": exact version number
">=0.8,<1.0": version range
"*": wildcard
```
- After adding a new dependency to the Cargo.toml file, the dependency can be installed and included in the project using the cargo build command. The cargo build command checks for any missing dependencies and downloads them from the specified package repository. Once the dependencies are installed, they can be imported and used in the Rust code using the use keyword.
- For example, to use the rand package in a Rust program, the following code can be used:
```
use rand::Rng;

fn main() {
  let mut rng = rand::thread_rng();
  let random_number = rng.gen::<u32>();
  println!("Random number: {}", random_number);
}
```
- In this code, the rand::Rng trait is imported using the use keyword. The rand::thread_rng() function is then used to obtain a random number generator, which is used to generate a random u32 number using the rng.gen::<u32>() method.
- Overall, dependencies in Rust are a powerful tool for reusing existing code, reducing development time, and improving the quality of Rust projects. The Cargo package manager and the Cargo.toml configuration file make it easy to manage dependencies and integrate external packages and libraries into Rust projects.
```
cargo.toml
[dependencies]
rand = "0.8.4"

main.rs
fn main()
{
    let x = rand::thread_rng().gen_range(1..101);
    println!("Random number is {}",x);
}
```
## Control Flow
### Comparison Operators and Truth tables
- Comparison operators are used to compare the values of two variables or expressions and produce a Boolean result. The comparison operators in Rust are:
```
==: equality
!=: inequality
<: less than
<=: less than or equal to
>: greater than
>=: greater than or equal to
```
- For example:
```
let x = 10;
let y = 3;
let is_equal = x == y;
let is_not_equal = x != y;
let is_less_than = x < y;
let is_greater_than_or_equal_to = x >= y;
```
- In this code, the ==, !=, <, and >= operators are used to compare the values of the x and y variables and produce Boolean results stored in new variables named is_equal, is_not_equal, is_less_than, and is_greater_than_or_equal_to, respectively.
- In Rust, truth tables can be used to evaluate the Boolean expressions and logical operations that produce Boolean results. A truth table is a table that lists all possible combinations of input values for a given expression or operation, and the resulting output values. Truth tables can be used to determine the truth value of complex logical expressions, to simplify Boolean expressions using the laws of Boolean algebra, and to verify the correctness of circuits and programs that use Boolean logic.
- In Rust, Boolean expressions are evaluated using the bool type, which can have two possible values: true or false. Logical operations in Rust, such as && (logical AND), || (logical OR), and ! (logical NOT), produce Boolean results based on the truth values of their input expressions.
- A truth table for a logical operation lists all possible combinations of input values for the input expressions, and the resulting output values. For example, the truth table for the && (logical AND) operation is:
![image](https://github.com/L0K3SHP/RUST/assets/42104811/57c88a21-e27b-416a-af71-3f255a7c6b3e)
-This truth table shows that the logical AND operation produces a true result only when both input expressions are true. Otherwise, the result is false.
- Similarly, the truth table for the || (logical OR) operation is:
![image](https://github.com/L0K3SHP/RUST/assets/42104811/bfa78a75-54a9-4fc9-885d-9e4a03af2771)
- This truth table shows that the logical OR operation produces a true result when at least one of the input expressions is true. Otherwise, the result is false.
- The truth table for the ! (logical NOT) operation is:
![image](https://github.com/L0K3SHP/RUST/assets/42104811/b48378fe-e833-46da-a123-ac22ea670d92)
- This truth table shows that the logical NOT operation produces a true result when the input expression is false, and a false result when the input expression is true.
- Overall, truth tables in Rust are a powerful tool for evaluating Boolean expressions and logical operations and for verifying the correctness of programs that use Boolean logic.
```
fn main() {
    let a = 5;
    let b = 10;
    let c = true;
    let d = false;

    println!("a > b: {}", a > b); // false
    println!("a >= b: {}", a >= b); // false
    println!("a < b: {}", a < b); // true
    println!("a <= b: {}", a <= b); // true
    println!("a == b: {}", a == b); // false
    println!("a != b: {}", a != b); // true
    println!("True or False: {}", c || d); //true
    println!("True or True: {}", c || c); //true
    println!("False or False: {}", d || d); //false
    println!("True and False: {}", c && d); //false
    println!("True and True: {}", c && c); //true
    println!("False and False: {}", d && d); //false
}
```
### Conditional Statements
- In Rust, conditional statements are used to control the flow of execution based on Boolean conditions. Rust supports two types of conditional statements: if statements and match statements.
- if statements are used to execute a block of code if a condition is true. An if statement can also include one or more else if clauses and an optional else clause to handle different cases. For example:
```
let x = 10;
if x > 0 {
  println!("x is positive");
} else if x < 0 {
  println!("x is negative");
} else {
  println!("x is zero");
}
```
- In this code, an if statement is used to check whether the value of x is positive, negative, or zero. If x is positive, the first branch of the if statement is executed and the message "x is positive" is printed. If x is negative, the second branch of the if statement is executed and the message "x is negative" is printed. Otherwise, the third branch of the if statement is executed and the message "x is zero" is printed.
- Overall, conditional statements in Rust are a powerful tool for controlling the flow of execution based on Boolean conditions and matching values against patterns. The if statement and match statement provide different ways to handle different cases and make Rust code more expressive and concise.
```
#![allow(unused)]

use std::io;
use rand::Rng;

fn main() {
    //if, else if, else

    println!("How much money do you have?");
    let mut input_money = String::new();
    io::stdin().read_line(&mut input_money);

    let money: i32 = input_money.trim().parse().expect("Entry was not an integer");

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    if (age >= 21) && (money >= 5) {
        println!("We're getting a drink!")
    } else if (age >=21) && (money < 5) {
        println!("Come back with more money!")
    } else if (age < 21) && (money >= 5) {
        println!("Nice try, kid!")
    } else {
        println!("You're too young and too poor.")
    };
}
```

### Match
- In Rust, conditional statements are used to control the flow of execution based on Boolean conditions. Rust supports two types of conditional statements: if statements and match statements.
- match statements are used to match a value against a set of patterns and execute the corresponding code for the first matching pattern. A match statement can include one or more arms, which consist of a pattern and the corresponding code to execute. For example:
```
let x = 3;
match x {
  0 => println!("x is zero"),
  1 | 2 => println!("x is one or two"), 
  _ => println!("x is something else"),
}
```
- In this code, a match statement is used to match the value of x against a set of patterns. The first pattern 0 matches the value of x if it is zero, and the corresponding message "x is zero" is printed. The second pattern 1 | 2 matches the value of x if it is either one or two, and the corresponding message "x is one or two" is printed. The underscore (_) pattern is a catch-all pattern that matches any value that hasn't been matched by the previous patterns, and the corresponding message "x is something else" is printed.
- Overall, conditional statements in Rust are a powerful tool for controlling the flow of execution based on Boolean conditions and matching values against patterns. The if statement and match statement provide different ways to handle different cases and make Rust code more expressive and concise.
```
#![allow(unused)]
// This is comments

use std::cmp::Ordering;
fn main() {
  // Match - matching arm and all possible values must be covered

  //Type 1
  let cadidancy_age = 33;

  match cadidancy_age{
    1..=24 => println!("Cannot ho;d office"),
    25..=29 => println!("Can run for the House"),
    30..=34 => println!("Can run for the Senate"),
    35..=i32::MAX => println!("Can run for President"),
    _=> println!("Are you an infant?")
  };


  // Type 2 use std::cmp::Ordering;
  let age = 33;
  let drink_age = 21;
  match age.cmp(&drink_age){
    Ordering::Less => println!("Cannot drink"),
    Ordering::Equal => println!("Woo, party sharty"),
    Ordering::Greater => println!("Can drink"),
  }
  }
```

### Loops
- In Rust, loops are used to repeat a block of code until a certain condition is met. Rust provides several loop constructs, including loop, while, and for loops.
- The loop statement is used to repeat a block of code indefinitely until a break statement is encountered. For example:
```
let mut count = 0;
loop {
  count += 1;
  println!("Count: {}", count);
  if count == 10 {
    break;
  }
}
```
- In this code, a loop statement is used to repeat a block of code that increments a counter and prints its value. The loop continues until the counter reaches a value of 10, at which point the break statement is encountered and the loop is terminated.
- The while statement is used to repeat a block of code while a certain condition is true. For example:
```
let mut count = 0;
while count < 10 {
  count += 1;
  println!("Count: {}", count);
}
```
- In this code, a while statement is used to repeat a block of code that increments a counter and prints its value while the counter is less than 10. The loop terminates when the counter reaches a value of 10 and the condition becomes false.
- The for statement is used to iterate over a range, an iterator, or a collection of elements. For example:
```
for i in 1..=10 {
  println!("Count: {}", i);
}

let numbers = vec![1, 2, 3, 4, 5];

for num in numbers {
  println!("Number: {}", num);
}
```
- In the first code block, a for statement is used to iterate over a range of values from 1 to 10, and print each value. The ..= operator is used to include the upper bound in the range.
- In the second code block, a for statement is used to iterate over a vector of numbers and print each number. The loop terminates when all elements in the vector have been processed.
- Overall, loops in Rust are a powerful tool for repeating a block of code until a certain condition is met or all elements in a collection have been processed. The loop, while, and for loops provide different ways to handle different cases and make Rust code more expressive and concise.
```
fn main() {
  // Loops - For, While, Infinite loop

  // For loop - start to finish of am iterrate

  let mut veg = ["Cabbage", "Peas", "onion"];
  for i in veg.iter(){
    println!("{}",i);
  }

  //While loop - execute until its true
  let mut y = 0;
  while y <= 10{
    println!("{}",y);
    y +=1;
  }

  //loop - Infinite loops
  let mut a = 0;

  println!("Counting!");
  loop{
    a += 1;
    println!("{}", a);
    if a == 10{
      println!("Reach 10");
      continue;
    }else if a == 20{
      println!("Reach 20");
      break;
    }
  }
  }
```

## Functions
- In Rust, functions are used to encapsulate a block of code that performs a specific task and can be called from other parts of the program. Functions in Rust are defined using the fn keyword, followed by the function name, the list of parameters enclosed in parentheses, and the function body enclosed in curly braces. For example:
```
fn greet(name: &str) {
  println!("Hello, {}!", name);
}
```
- In this code, a function named greet is defined that takes a single parameter name of type &str, which is a reference to a string. The function body simply prints a greeting message that includes the name.
- Functions in Rust can also return a value using the -> syntax, followed by the return type. For example:
```
fn add(x: i32, y: i32) -> i32 {
  x + y
}
```
- In this code, a function named add is defined that takes two parameters x and y of type i32, which are integers. The function body adds the two integers and returns the result as an i32 value.
- Functions in Rust can have default parameter values, which are used when a parameter value is not provided. For example:
```
fn repeat(word: &str, count: u32) -> String {
  word.repeat(count as usize)
} 

fn main() { 
  println!("{}", repeat("hello", 3)); // prints "hellohellohello"
  println!("{}", repeat("world", 5)); // prints "worldworldworldworldworld"
  println!("{}", repeat("bye", 1)); // prints "bye"
}
```
- In this code, a function named repeat is defined that takes two parameters word and count. The count parameter has a default value of 1, which is used when the count value is not provided. The function body repeats the word string count times and returns the result as a new string.
- Functions in Rust can also use the return keyword to explicitly return a value from the function. For example:
```
fn divide(x: f32, y: f32) -> Result<f32, String> { 
  if y == 0.0 { 
    return Err(String::from("Division by zero"));
  }
  Ok(x / y)
}

fn main() { 
  match divide(10.0, 2.0) { 
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err), 
  }
  match divide(10.0, 0.0) { 
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err), 
  }
}
```
- In this code, a function named divide is defined that takes two parameters x and y of type f32, which are floating-point numbers. If y is equal to 0.0, the function returns an error using the Err enum. Otherwise, the function returns the division of x and y as a Result<f32, String> value using the Ok enum. The match statement is used to handle the possible error and result values returned by the function.
- Overall, functions in Rust are a powerful tool for encapsulating a block of code that performs a specific task, returning values, and accepting parameters. Functions can also have default parameter values, use the return keyword to explicitly return a value, and return errors using the Result enum.
```
#![allow(unused)]
// This is comments

use std::io::{self, Read};
use std::cmp::Ordering;
fn main() {
  // Function - small organised code
  whoami();
  calculator(8,9);
  // return value
  println!("{}",mul(5, 9));
  // multiple return
  let (added, multiply) = add_mul(5, 90);
  println!("Add: {}, Mul: {}",added,multiply);
  }

fn whoami(){
  let name = "Lok";
  let age = 24;
  println!("My name is {} and i am {} years old",name,age);
}

// with parameter
fn calculator(x:i32, y:i32)
{
  let mut x_f = x as f64;
  let mut y_f = y as f64;
  println!("Calculation of {} and {}",x,y);
  println!("Addition:  {}",x+y);
  println!("Subtraction:  {}",x-y);
  println!("Multiplication:  {}",x*y);
  println!("Division:  {}",x_f/y_f);
}

// return value
fn mul(a:i32, b:i32) -> i32 {
  a*b
}

// multiple return
fn add_mul(p: i32, q: i32) -> (i32,i32) {
  (p+q,p*q)
}
```

## Vectors
- In Rust, vectors are a dynamic, resizable array type that can store a variable number of elements of the same type. Vectors are defined using the Vec struct and can be used to store and manipulate collections of data.
- To create a new vector, you can use the Vec::new() method or the vec! macro:
```
let mut vec1 = Vec::new(); // creates an empty vector
let vec2 = vec![1, 2, 3]; // creates a vector with three elements
```
- In this code, vec1 is created using the Vec::new() method, which creates an empty vector that can be populated later. vec2 is created using the vec! macro, which creates a vector with three elements: 1, 2, and 3.
- You can add elements to a vector using the push method:
```
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
vec.push(3);
```
- In this code, the push method is used to add the values 1, 2, and 3 to the vec vector.
- You can access elements of a vector using square bracket notation and the index of the element:
```
let vec = vec![1, 2, 3];
let second_element = vec[1];
println!("The second element is {}", second_element);
```
- In this code, the vec vector is created using the vec! macro, and the second element is accessed using square bracket notation and the index 1. The resulting value, 2, is then printed to the console.
- You can also use methods like len and iter to get the length of a vector and iterate over its elements:
```
let vec = vec![1, 2, 3];
println!("The length of the vector is {}", vec.len());

for element in vec.iter() {
  println!("Element: {}", element);
} 
```
- In this code, the len method is used to get the length of the vec vector, which is then printed to the console. The iter method is used to create an iterator over the elements of the vector, which is then used in a for loop to print each element to the console.
- Overall, vectors in Rust are a powerful and flexible tool for storing and manipulating collections of data. Vectors are dynamically resizable, can store a variable number of elements of the same type, and support a variety of useful methods for adding, accessing, and iterating over their elements
```
fn main() {
  //Vector-  Similar to Array
  // Slower then array, but more flexible

  let mut vec1 = Vec::new();
  let mut vec2 = vec![1,2,3,4,5,6];

  // Push data at the nth position
  vec1.push(1);
  vec2.push(8);

  //Length

  println!("length of Vector 2 {}",vec2.len());

  // Accees element in vector
  let ele_2 = vec2[5];
  println!("{}",ele_2);

  for i in vec1.iter(){
    println!("{}",i);
  }

  for j in vec2.iter(){
    println!("{}",j);
  }
  }
```

## Structures
- In Rust, a structure, also known as a struct, is a custom data type that groups together zero or more variables of different types under a single name. A struct is defined using the struct keyword followed by the name of the struct and the variables it contains.
Here's an example of a struct definition:
```
struct Rectangle { width: u32, height: u32, }
```
- In this code, a Rectangle struct is defined that contains two variables: width and height. Both variables are of type u32, which is an unsigned 32-bit integer type.
- You can create an instance of a struct using the struct's name and the values for its variables:
```
let rect = Rectangle { width: 30, height: 50 };
```
- In this code, an instance of the Rectangle struct is created with a width of 30 and a height of 50. The resulting instance is stored in the rect variable.
You can access the variables of a struct using dot notation:
```
let area = rect.width * rect.height;
println!("The area of the rectangle is {}", area);
```
- In this code, the width and height variables of the rect instance are accessed using dot notation, and their product is used to calculate the area of the rectangle.
- Structs in Rust can also have methods defined on them, which can be used to perform operations on the struct's variables:
```
impl Rectangle { 
  fn area(&self) -> u32 {
    self.width * self.height
   }
}
let rect = Rectangle {width: 30, height: 50 };
let area = rect.area();

println!("The area of the rectangle is {}", area);
```
- In this code, an impl block is used to define a method named area on the Rectangle struct. The area method takes a reference to the self instance and returns the product of its width and height variables. The area method is then called on the rect instance, and its return value is printed to the console.
- Overall, structs in Rust are a powerful tool for defining custom data types that group together related variables under a single name. Structs can be used to create instances that store data, access and modify the variables of an instance, and define methods that operate on the instance's variables.
```
fn main() {
  // Structures

  struct Rectangle{
    width: u32,
    height: u32,
  }

  let  rect = Rectangle{width: 30, height: 80};

  let area = rect.width*rect.height;

  impl Rectangle{
    fn area(&self) -> u32{
      self.width * self.height
    }
      
  }

  let area = rect.area();

  println!("Area of Rectangle with width {} x height {} = {}", rect.width,rect.height,area);
 
  struct car {
    make: String,
    model: String,
    year: u32,
    price: f64,
  }

  let mut Car = car{
    make: String::from("Lamborgini"),
    model: String::from("Huracan"),
    year: 2020,
    price: 320000.00
  };

  println!("The cost of a {} {} {} is ${}", Car.year,Car.model,Car.make,Car.price);
  }
```
## Enums
- In Rust, an enumeration, also known as an enum, is a custom data type that allows you to define a set of named values. Enums are defined using the enum keyword followed by the name of the enum and the possible values it can take.
Here's an example of an enum definition:
```
enum Direction {
  Up,
  Down, 
  Left,
  Right,
}
```
- In this code, a Direction enum is defined that contains four possible values: Up, Down, Left, and Right. Each of these values is an enum variant, and can be thought of as a distinct value of the Direction type.
You can create an instance of an enum by specifying one of its variant names:
```
let direction = Direction::Up;
```
- In this code, an instance of the Direction enum is created with the Up variant. The resulting instance is stored in the direction variable.
Enums in Rust can also contain data associated with each variant:
```
enum Shape { 
  Circle(f32),
  Rectangle(f32, f32),
}
let circle = Shape::Circle(10.0);
let rectangle = Shape::Rectangle(30.0, 40.0);
```
- In this code, a Shape enum is defined that contains two possible values: Circle and Rectangle. The Circle variant contains a single f32 value representing the radius of the circle, and the Rectangle variant contains two f32 values representing the width and height of the rectangle. An instance of the Shape enum is created for a circle with a radius of 10.0, and another instance is created for a rectangle with a width of 30.0 and a height of 40.0.
Enums in Rust can also have methods defined on them, which can be used to perform operations on the enum's variants:
```
impl Direction { 
  fn opposite(&self) -> Direction {
    match *self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
     }
  }
}

let direction = Direction::Up;
let opposite_direction = direction.opposite();
```
- In this code, an impl block is used to define a method named opposite on the Direction enum. The opposite method takes a reference to the self instance and returns the opposite direction. The match expression is used to pattern match on the current direction and return the opposite direction. The opposite method is then called on the direction instance, and its return value is stored in the opposite_direction variable.
- Overall, enums in Rust are a powerful tool for defining custom data types that represent a set of named values. Enums can be used to create instances that store data associated with each variant, access and modify the data associated with an instance, and define methods that operate on the enum's variants.
```
fn main() {
  // Enum
  // defined set of named values

  enum Direction{
    Up,
    Down,
    Left,
    Right,
  }

  let up = Direction::Up;

 #[derive(Debug)]
  enum Shape{
    Circle(f32),
    Rectangle(f32,f32),
  }

  let circle= Shape::Circle(12.0);
  let rectangle = Shape::Rectangle(1052.56, 455.2);

  print!("{:?}, {:?}",circle,rectangle);
  }
```

## Generics
- In Rust, generics are a language feature that allows you to define a function or a data type without specifying the specific types that it will work with. Instead, generic functions and types can work with any type that satisfies a set of constraints defined by the generic.
- Generics are denoted using angle brackets <...>, and the type parameter is typically given a name that starts with a capital letter. Here's an example of a generic function that takes two arguments of the same type and returns their sum:
```
fn sum<T: std::ops::Add<Output=T>>(a: T, b: T) -> T { 
  a + b
}

let x = sum(1, 2);
let y = sum(1.0, 2.0);
println!("{} {}", x, y); // prints "3 3"
```
- In this code, a sum function is defined that takes two arguments of type T and returns their sum, where T is a generic type parameter that satisfies the std::ops::Add trait. The std::ops::Add trait is a trait that defines the + operator and is implemented by many numeric types in Rust. The Output=T syntax is used to specify that the return type of the Add operation is the same as the input type T.
- The sum function is then called with two integer arguments 1 and 2, and with two floating-point arguments 1.0 and 2.0. In both cases, the function returns the sum of the two arguments as a value of the same type.
Generics can also be used to define generic data types, such as a generic vector type:
```
struct Vector<T> { 
  x: T,
  y: T,
}

let v = Vector { x: 1.0, y: 2.0 };
println!("({}, {})", v.x, v.y); // prints "(1, 2)"
```
- In this code, a Vector struct is defined that contains two variables x and y, both of type T, which is a generic type parameter. An instance of the Vector struct is then created with x and y values of 1.0 and 2.0, respectively, and the values of x and y are printed to the console.
- Overall, generics in Rust provide a powerful tool for defining functions and data types that can work with a wide range of types, without needing to specify the exact type at compile time. This allows for greater flexibility and code reuse, as generic functions and types can be used with any type that satisfies a set of constraints defined by the generic.
```
use std::ops::*;

fn main() {
  //Generics

  fn sum<T: Add<Output=T>>(a: T,b:T)->T{
    a + b
  }

  let x = sum(5,6);
  let y = sum(54.64,84.65);
  println!("The value of x is {}",x);
  println!("The value of y is {}",y);
  print!("Adding of numbers {}",sum(54,98));

  struct Item<T>{
    a:T,
    b:T,
  }

  let i = Item{a: 5, b: 55};

  println!("{}, {}", i.a,i.b);
  }
```

## Traits
- In Rust, traits are a language feature that allows you to define a set of methods that can be implemented by any type that satisfies the requirements of the trait. Traits provide a way to define a common interface for types that may have different implementations, and allow you to write generic code that can work with a wide range of types.
- Traits are defined using the trait keyword followed by the name of the trait and a set of method signatures. Here's an example of a trait definition:
```
trait Drawable { 
  fn draw(&self);
}
``` 
- In this code, a Drawable trait is defined that contains a single method draw. The draw method takes a reference to self and does not return a value.
- Traits can be implemented for any type that satisfies the requirements of the trait. Here's an example of a struct that implements the Drawable trait:
```
struct Circle { 
  radius: f32,
}

impl Drawable for Circle {
  fn draw(&self) { 
    println!("Drawing a circle with radius {}", self.radius);
   }
} 

let circle = Circle { radius: 10.0 }; 
circle.draw(); // prints "Drawing a circle with radius 10"
```
- In this code, a Circle struct is defined that contains a single variable radius. The Drawable trait is then implemented for the Circle struct by defining the draw method for the struct. The draw method is called on an instance of the Circle struct, and the result is printed to the console.
- Traits can also be used to define generic functions and types that work with any type that satisfies the requirements of the trait. Here's an example of a generic function that takes a reference to any type that implements the Drawable trait and calls its draw method:
```
fn draw_shape<T: Drawable>(shape: &T) { 
  shape.draw();
}

let circle = Circle { radius: 10.0 };
draw_shape(&circle); // prints "Drawing a circle with radius 10" 
```
- In this code, a draw_shape function is defined that takes a reference to any type T that implements the Drawable trait. The function calls the draw method on the shape and prints the result to the console. The draw_shape function is then called with an instance of the Circle struct, and the result is printed to the console.
- Overall, traits in Rust provide a powerful tool for defining a common interface for types that may have different implementations. Traits allow you to write generic code that can work with a wide range of types, and can be used to define generic functions and types that work with any type that satisfies the requirements of the trait.
```
eg 1

trait Damage {
  fn damage(self: &mut Self);
    
}

#[derive(Debug)]

struct HP{
  hp_remain: i32,
}

impl Damage for HP {
  fn damage(self: &mut Self){
    self.hp_remain -= 1;
  }
}

fn main() {
  // Traits -  A group od methods that are defined for any particular type.
  let mut hp = HP{hp_remain: 100};
  hp.damage();
  println!("You took a damage! HP remaing:{:?}",hp);
  }

eg 2
trait Drawable{
  fn draw(&self);
}

struct Circle{
  radius: i32,
}

impl Drawable for Circle{
  fn draw(&self) {
    println!("Drawable a circle with radius {}", self.radius);
  }
}

// Calling traits via generics
fn draw_shape<T: Drawable>(shape: &T){
  shape.draw();
}

fn main() {
  let circle = Circle{radius: 10};
  circle.draw();

  //Genrics
  draw_shape(&circle);
  }
```

## Memory Management
### Ownership
- Ownership is a central concept in Rust's memory management system. In Rust, every value has an owner, which is a variable that is responsible for managing the memory used by that value. When the owner goes out of scope, the memory is automatically freed.
Here's an example of ownership in Rust:
```
fn main() { 
  let s = String::from("hello"); // s is the owner of the string "hello"
  println!("{}", s); // prints "hello" 
} // s goes out of scope and the memory is freed
```
- In this code, a new String value is created using the String::from function, and s is assigned as the owner of the String value. When s goes out of scope at the end of the function, the memory used by the String value is automatically freed.
- Ownership can also be transferred between variables using the move keyword:
```
fn main() {
  let s1 = String::from("hello"); // s1 is the owner of the string "hello" 
  let s2 = s1; // s2 takes ownership of the string "hello" println!("{}", s2); // prints "hello" 
} // s2 goes out of scope and the memory is freed, s1 is invalid since ownership was transferred
```
- In this code, ownership of the String value is transferred from s1 to s2 using the move keyword. When s2 goes out of scope, the memory used by the String value is freed, and s1 is no longer valid since ownership was transferred.
- Overall, ownership is a key concept in Rust's memory management system that allows for safe and efficient memory management without requiring garbage collection or manual memory management. By assigning ownership to values and automatically freeing memory when the owner goes out of scope, Rust provides a memory management system that is both safe and efficient.
```
/* In Rust, memorty is managed through a system of ownership and borrowing.
  Each value in Rust has an owner.
  which is responsible for managing the memoty used by the value.
  When a value goes out of scop, its memory is automatically freed.
  This eleminates the need for manual memory management or garbage collection.
  which can lead to bugs, performance issue, and security vulnerbilities.

  STACK vs HEAP
  Stack is dast, values are stored inorder and all are fuix-sized. Uses LIFO.
  Heap is slow, values are unorders and of a variable size.
  The heap uses a return address for requested space called a pointer.

  Ownership - has 3 rules.
  Each value has a owner (owned by a variable)
  There can only be one owner at atime
  When the owner goes out of scope, the memoty becomes free */

fn main() {
  let name = String::from("Random");
  let new_name = name;
  let new_name2 = new_name.clone();
  print!("Hello, my nme is {}.",new_name);
  //print!("Hello, my nme is {}.",name);
  print!("Hello, my nme is {}.",new_name2);
  }

``
