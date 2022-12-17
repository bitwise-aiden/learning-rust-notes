# "The Rust Programming Language" notes
link: https://doc.rust-lang.org/stable/book/


## [Chapter 01.3](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html)
Creating a new project:
``` shell
cargo new <name>
cd <name>
```

Build a project:
``` shell
cargo build (--release)
```

Run a project:
``` shell
cargo run
```

Check a project is in compile state:
``` shell
cargo check
```

## [Chapter 02](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)
Update dependencies:
``` shell
cargo update
```

View docs of current project + dependencies:
``` shell
cargo doc --open
```

## [Chapter 03.2](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html)
Shadowing values:
``` rust
let x = 5;

let x = x + 1;

{
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
}

println!("The value of x is: {x}");

// output:
// The value of x in the inner scope is: 12
// The value of x is: 6
```

Accessing tuple members:
``` rust
let x: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0;

let six_point_four = x.1;

let one = x.2;
```

Empty tuple is called a `unit`:
``` rust
let unit = ();
```

## [Chapter 03.5](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html)
Values can be returned from a loop:
``` rust
let value = loop {
    break 1;
};
```

Loop labels help control where to break/continue:
``` rust
'outer_loop: loop {
    'inner_loop: loop {
        break 'outer_loop;
    }
}
```

## [Chapter 04.1](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)
Variables stored on the stack can be copied and cloned:
``` rust
let x = 5;
let y = x;
let z = x.clone();

println!("x = {x}, y = {y}, z = {z}");
```

Variables stored on the heap can be cloned and not copied:
``` rust
let x = String::from("Hello");
// let y = x; <-- this is invalid
let y = x.clone();

println!("x = {x}, y = sad face, z = {z}");
```

Implementing the `Copy` trait on a type will mean it gets copied instead of moved:
``` rust
#[derive(Copy)]
struct MyCoolType {}
```

Cannot implement `Copy` on a type that contains a part that has implemented the `Drop` trait.
``` rust
#[derive(Drop)]
struct MyDropType {}

// #[derive(Copy)] <-- this is invalid
struct MyCoolType {
    my_drop_type: MyDropType,
}
```

## [Chapter 04.2](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)
Multible mutable borrow references are not allowed in same scope:
``` rust
let mut s = String::from("Hello");

let r1 = &mut s;
let r2 = &mut s; // <-- this is invalid

println!("{r1}, {r2}");
```

Multiple regular borrow references are fine until mutable reference in same scope:
``` rust
let mut s = String::from("Hello");

let r1 = &s;
let r2 = &s; // <-- this is valid
let r3 = &mut s; // <-- this is invalid

println!("{r1}, {r2}, {r3}");
```

Reference scope is from when it is declared until when it is used:
``` rust
let mut s = String::from("Hello");

let r1 = &s;
let r2 = &s;
print("{r1}, {r2}");

let r3 = &mut s;
println!("{r3}");
```

Dangling references will error:
``` rust
fn dangle() -> &String {
    let s = String::from("Hello");

    &s // <-- this is invalid
}
```

Function scoped variables will be moved as retuns:
``` rust
fn no_dangle() -> String {
    let s = String::from("Hello");

    s
}
```

## [Chapter 04.3](https://doc.rust-lang.org/stable/book/ch04-03-slices.html)
Array/String slices reference original:
``` rust
let mut s = String::from("Hello world");

let word = s[..5];

s.clear() // <-- this is invalid

println!("The first word is: {word}");
```

## [Chapter 05.1](https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html)
Struct update syntax copies data not named in init:
``` rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1,
};
```

Using update syntax for fields on the heap will result in a move, invalidating original:
``` rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1,
}; // user1 is no longer valid as username was moved

let user3 = User {
    email: String::from("yetanother@example.com"),
    username: String::from("anotherusername"),
    ..user1,
} // user2 is still valid as remaining fields were copied
```

Tuple structs give meaning to data through name:
``` rust
struct Point(i32, i32, i32);

let origin = Point(0, 0, 0);
```

Unit structs give meaning without data:
``` rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

## [Chapter 05.2](https://doc.rust-lang.org/stable/book/ch05-02-example-structs.html)
Use `{:?}` to envoke debug printing and `{:#?}` for pretty print:
``` rust
#[derive(Debug)]
struct Recatangle {
    width: i32,
    height: i32,
}

let r = Rectangle {
    width: 30,
    height: 50,
};

println!("{:?}", r);
// Rectangle { width: 30, height: 50 }

println!("{:#?}", r);
// Rectangle {
//     width: 30,
//     height: 50,
// }
```

Use `dbg!` macro for printing while assinging. It takes and returns ownership + prints file and line:
``` rust
let scale = 2;
let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
};
// [src/main.rs:3] 30 * scale = 60
```

Pass value by reference to have `dbg!` not take ownership:
``` rust
dbg!(&rect1);
// &rect1 = Rectangle {
//     width: 30,
//     height: 50,
// }
```

## [Chapter 05.3](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)
Use raw `self` for methods when you want to transform data and prevent caller from using original.
``` rust
impl Rectanlge {
    fn to_polygon(self) -> Polygon {
        Polygon {
            width: self.width,
            heigth: self.height,
        }
    }
}

// ...

let polygon = rect1.to_polygon();

dbg!(&rect1); // <-- this is invalid
```

Rust automatically adds the necessary `&`, `&mut`, or `*` to method calls:
``` rust
p1.distance(&p2);
(&p1).distance(&p2);
```

`impl` has `Self`keyword as an alias for the type being implemented:
``` rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
