// Author: Alex Ball
// I am creating this project as a set of notes on rust.

// These notes were created while following along with a rust tutorial
// by Tensor Programming on Youtube:
// https://www.youtube.com/watch?v=y7iSQ3s_yms&index=3&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW

mod intro_1;

// use intro_1::intro_1_notes;

fn main() {
  // Note that functions that do not return anything will implicitly return an empty tuple.

  // intro_1_notes();

  traits_and_generic_types();
}

pub fn traits_and_generic_types() {
  // traits are similar to interfaces (think Java)
  trait Shape {
    fn area(&self) -> u32;
  }

  struct Rectangle {
    x: u32,
    y: u32,
  }

  struct Circle {
    radius: f64,
  }

  // a struct can implement a trait by fulfilling its required methods.
  impl Shape for Rectangle {
    fn area(&self) -> u32 {
      self.x * self.y
    }
  }

  impl Shape for Circle {
    fn area(&self) -> u32 {
      (3.141 * self.radius * self.radius) as u32
    }
  }

  println!("Basic trait example with shape area() implementation:");
  let c = Circle { radius: 100.1 };
  let r = Rectangle { x: 30, y: 20 };
  println!("Circle area: {} Rectangle area: {}", c.area(), r.area());

  // derive annotatino can also be used to implement traits with basic
  // implementations that are provided by the compiler.
  // Clone vs Copy: StackOverflow discussion summarizes the difference as:
  // Clone can be used for an arbitrarily complex duplication (deep copy, 
  // shallow copy, whatever). Copy is a shallow (reference, memcpy) copy.
  // https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone
  #[derive(Debug, Clone, Copy)]
  struct A(i32);
  struct B(f32);

  let a = A(32);
  let b = B(12.13);
  // c would get ownership of the value that a owns (a loses its value
  // and can't be used until c gives the value back).
  // let c = a;
  
  // we can use clone to avoid giving away ownership. c will get a clone of a.
  let c = a.clone();
  println!("{:?}", a);

  // The Copy trait basically does the same thing as Clone except that it
  // happens any time something tries to borrow from a (you do not call clone(),
  // but you can no longer control when NOT to clone).
  // Think Java - Copy causes you to "opt-in" to Java style references that can
  // be used after passing them to a function.
  let d = a;
  println!("{:?}", a);
}
