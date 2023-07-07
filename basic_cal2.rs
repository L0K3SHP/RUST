#![allow(unused)]
// This is comments

use std::io;
use std::cmp::Ordering;
fn main() {
  let mut x = String::new();
  let mut y = String::new();
  let mut op = String::new();
  
  // 1st number and conversion
  println!("Enter 1st number");
  io::stdin().read_line(& mut x);
  let mut x : i32 = x.trim().parse().unwrap();
  let x_f = x as f64;

   // 2nd number and conversion
   println!("Enter 2st number");
   io::stdin().read_line(& mut y);
   let mut y : i32 = y.trim().parse().unwrap();
   let y_f = y as f64;

   // Operations
   println!("Enter Operation to perform(+,/,*,-,)"); 
   io::stdin().read_line(& mut op);
   let mut op = op.trim();

  if op == "+"{
    println!("Addition of {} and {} = {}",x,y,add(x,y));
  }
  else if op == "-" {
    println!("Addition of {} and {} = {}",x,y,sub(x,y));
  }
  else if op == "*" {
    println!("Addition of {} and {} = {}",x,y,mul(x,y));
  }
  else if op == "/"{
    println!("Addition of {} and {} = {}",x,y,div(x_f,y_f));
  }
  else {
      println!("Enter Valid operatior")
  }
  }

fn add(a:i32,b:i32) -> i32{                                      
  a+b
}

fn sub(a:i32,b:i32) -> i32{                                      
  a-b
}

fn mul(a:i32,b:i32) -> i32{                                      
  a*b
}

fn div(a:f64,b:f64) -> f64{                                      
  a/b 
}
