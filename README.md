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
