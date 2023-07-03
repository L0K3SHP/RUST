#![allow(unused)]
// This is comments

use std::io;

fn main() {
    let mut x = String::new();
    println!("Enter 1st number: ");
    io::stdin().read_line(& mut x);
    let mut x: f64 = x.trim().parse().unwrap();

    let mut y = String::new();
    println!("Enter 2nd number: ");
    io::stdin().read_line(& mut y);
    let mut y: f64 = y.trim().parse().unwrap();

    println!("{} + {} = {}", x,y,x+y);
    println!("{} - {} = {}", x,y,x-y);
    println!("{} * {} = {}", x,y,x*y);
    println!("{} / {} = {}", x,y,x/y);
  }
