// Author: Alex Ball
// I am creating this project as a set of notes on rust.

// These notes were created while following along with a rust tutorial
// by Tensor Programming on Youtube:
// https://www.youtube.com/watch?v=y7iSQ3s_yms&index=3&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW

mod intro_1;

// use intro_1::_intro_1_notes;

fn main() {
  // Note that functions that do not return anything will implicitly return an empty tuple.

  // _intro_1_notes();

  // _traits();
  generic_types();
}

fn _traits() {
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
  struct Alpha(i32);

  // Comparison operators
  // #[derive(Eq, PartialEq, PartialOrd, Ord)]
  struct Beta(f32);

  let _a = Alpha(32);
  let _b = Beta(12.13);
  // c would get ownership of the value that a owns (a loses its value
  // and can't be used until c gives the value back).
  // let c = a;

  // we can use clone to avoid giving away ownership. c will get a clone of a.
  let _c = _a.clone();
  println!("{:?}", _a);

  // The Copy trait basically does the same thing as Clone except that it
  // happens any time something tries to borrow from a (you do not call clone(),
  // but you can no longer control when NOT to clone).
  // Think Java - Copy causes you to "opt-in" to Java style references that can
  // be used after passing them to a function.
  let _d = _a;
  println!("{:?}", _a);

  // traits can override basic operators (think of + - * /).
  use std::ops;
  struct A;
  struct B;
  #[derive(Debug)]
  struct AB;
  #[derive(Debug)]
  struct BA;

  impl ops::Add<B> for A {
    type Output = AB;

    fn add(self, _rhs: B) -> AB {
      AB
    }
  }

  impl ops::Add<A> for B {
    type Output = BA;

    fn add(self, _rhs: A) -> BA {
      BA
    }
  }

  println!("Trait overriding ops::Add(): ");
  println!("{:?}", A + B);
  println!("{:?}", B + A);

  // implement Drop, which is a function (from a trait) automatically called
  // when a variable gets dropped. A variable is dropped when it leaves scope.
  struct DropStruct {
    a: String,
  }

  impl Drop for DropStruct {
    fn drop(&mut self) {
      println!("dropped {}", self.a)
    }
  }

  println!("Drop example:");
  let _outer = DropStruct {
    a: String::from("Outer scope variable"),
  };
  {
    let _middle = DropStruct {
      a: String::from("Middle scope variable"),
    };
    {
      let _inner = DropStruct {
        a: String::from("Inner scope variable"),
      };
      println!("leaving innermost scope");
    }
    println!("leaving middle scope");
  }
  // drop function can be called to explicitly drop a variable.
  drop(_outer);
  println!("End of drop example.");

  // Iterator trait can be implemented. Used in for loops, allowing
  // iteration over a collection.
  println!("Start Iterator example: ");
  struct Fib {
    c: u32,
    n: u32,
  }

  impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
      let n = self.c + self.n;
      self.c = self.n;
      self.n = n;

      Some(self.c)
    }
  }

  fn fib() -> Fib {
    Fib { c: 1, n: 1 }
  }

  // Because we implemented the Iterator trait, Fib also inherits the take()
  // and skip() methods.
  for j in fib().take(10) {
    println!("{}", j);
  }

  println!("skip method example: ");

  for j in fib().skip(14).take(10) {
    println!("{}", j);
  }

  // example of manually calling the next() method
  println!("next method example: ",);
  let mut f = fib();
  println!("{:?}", f.next());
  println!("{:?}", f.next());
  println!("{:?}", f.next());
  println!("{:?}", f.next());
  println!("{:?}", f.next());

  println!("End Iterator examples");
}

fn generic_types() {
  // Generics look similar to Java generics.
  #[derive(Debug)]
  struct Square<T> {
    _x: T,
  }

  // Except that you don't have to declare what
  // type is used when you declare a variable.
  let _s = Square { _x: 10 };
  let _s = Square { _x: 1.0 };
  let _s = Square { _x: "Hello" };
  let _s = Square { _x: 'c' };

  // define what a parameter in a method must implement in order to be a valid parameter:
  use std::fmt;
  fn print_debug<T: fmt::Debug>(x: T) {
    println!("{:?}", x);
  }

  // Since Square derives Debug, it can be accepted as a parameter for print_debug().
  print_debug(_s);

  struct _A<T> {
    x: T,
  }

  /* 
  the first <J> here is declaring that we need <J> to be in scope for this
  impl block. The A<J> is referencing the struct above, and denoting that
  the J used in this impl block is the same J as in the A struct.
  
  Remember T and J are just denoting that there is a generic involved,
  so they don't have to match between the struct and the impl block.
  */
  impl<J> _A<J> {
    fn print_me(&self) -> &J {
      &self.x
    }
  }

  let a = _A { x: "I am a string" };
  println!("A<T> print_me example: {}", a.print_me());

  // Multiple generics
  struct _B<U, V> {
    x: U,
    y: V,
  }

  // 1 generic, multiple members of that same type (cannot create this
  // struct with members of different types).
  struct _C<Z> {
    x: Z,
    y: Z,
  }

  // get access to multiplication trait
  use std::ops::Mul;

  trait Shape<T> {
    fn area(&self) -> T;
  }

  // The types stored in x and y must be the same type and implement
  // the Mul trait (they must be able to be multiplied).
  struct _Rectangle<T: Mul> {
    x: T,
    y: T,
  }

  impl<T> Shape<T> for _Rectangle<T>
  // where clause lets you move the trait bounds (the list of
  // traits that T must implement) so that the code looks cleaner.
  // note how the trait bounds are delimited by plus (+) signs.
  where
    T: Mul<Output = T> + Copy,
  {
    // Since T has to implement Mul such that the output is T,
    // and T implements Copy, we know we can multiple x and y
    // and will get a value of type T we can return (will be copied).
    fn area(&self) -> T {
      self.x * self.y
    }
  }

  struct _Circle<T: Mul> {
    radius: T,
  }

  impl<T> Shape<T> for _Circle<T>
  where
    T: Mul<Output = T> + Copy,
  {
    fn area(&self) -> T {
      // This does not work because we don't know if T will be the same type as 3.141
      // 3.141 * (self.radius * self.radius)
      self.radius * self.radius
    }
  }
}
