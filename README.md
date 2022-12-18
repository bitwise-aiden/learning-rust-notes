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

## [Chapter 07.2](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
[Modules cheat sheet](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)


## [Chapter 07.4](https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
For functions, specify `use` paths up to the parent to show the function isn't defined in scope:
``` rust
use crate::some::path;

path::some_func();
```

For structs, enums, and other items, specify the full `use` path:
``` rust
use crate::some::path::SomeType;

let a = SomeType::new();
```

Exception to above would be if two items are named the same:
``` rust
use std::fmt::Result;
use std::io::Result; // <-- This is invalid
```

Shared `use` paths that include a module and it's child can be declared with `self`:
``` rust
use std::io::{self, Write};
```

## [Chapter 08.1](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)
Vectors can be initialized with empty:
``` rust
let v: Vector<i32> = Vector::new();
```

Vectors can be initialized with a shorthand for known values:
``` rust
let v = vec![1, 2, 3];
```

Add elements to a vector with push:
``` rust
let mut v: Vector<i32> = Vector::new();

v.push(5);
```

Accessing elements can be done via indexing or `.get` method:
``` rust
let third: &i32 = &v[2];

let third: Option<&i32> = v.get(2);
```

Borrow rules apply with mutable/immutable access to array:
``` rust
let first = &v[0];

v.push(6); // <-- this is invlaid

println!("The first element is: {first}");
```

Iterate with for:
``` rust
for i in &v {
    println!("{i}");
}
```

Iteration can be mutable:
``` rust
for i in &mut v {
    *i += 50;
}
```

## [Chapter 08.2](https://doc.rust-lang.org/stable/book/ch08-02-strings.html)
Append to string without taking ownership:
``` rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

`push` takes a single character and adds it to the string:
``` rust
let mut s = String::from("Hell");
s.push('o');
```

Using `+` moves first element:
``` rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2; // <-- s1 was moved and is no longer accesible

println!("{s3}");
```

`&String` values get coerced when passed to methods expecting `&str`:
``` rust
fn add(self, s: &str) -> String {}

s1.add(&s2)
s1.add(&s2[..]) // The same as
```

For more complicated concatenation, use `format!`, (doesn't take ownership of parameters):
``` rust
format!("{s1}-{s2}-{s3}");
```

Be specific about bytes or chars when accessing strings:
``` rust
let hello = "Здравствуйте";

let c = hello[1]; // <-- this is invalid

let s = &hello[0..4]; // Зд

for c in "Зд".chars() {
    println!("{c}");
}
// З, д

for b in "Зд".bytes() {
    println!("{b}");
}
// 208, 151, 208, 180
```

Grapheme clusters from strings are difficult and not a part of the standard library. Available on crates.io if needed.

## [Chapter 08.3](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html)
Iterate over hash maps with for:
``` rust
for (key, value) in &some_map {
    println!("{key}: {value}");
}
```

Copy values are copied, ownef values are moved:
``` rust
let name = String::from("Favourite color");
let value = String::from("Blue");

let mut map = HashMap::new();
map.insert(name, value);

// name, value are invalid at this point
```

References inserted into map must be valid for lifetime of map:
``` rust
let mut map = HashMap::new();

{
    let value = String::from("Blue");

    map.insert(1, value); // <-- this is invalid
}
```

Add values optionally if doesn't already exist:
``` rust
map.entry(String::from("Blue")).or_insert(50);
```

Update existing value in map:
``` rust
let text = "Hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

## [Chapter 09.1](https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html)

To have your program not unwind on panic use:
``` toml
# Cargo.toml

[profile.release]
panic = 'abort'
```

Halt program when in an unrecoverable state:
``` rust
fn main() {
    panic!("Crash and burn!");
}
```

To see backtrace on panic:
``` sh
RUST_BACKTRACE=1 cargo run
```

## [Chapter 09.2](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html)
Use `unwrap` to get value from result (will panic if error):
``` rust
let text = File::Open("hello.txt").unwrap();
```

Use `except` to get value from result with own error message:
``` rust
let text = File::Open("hello.txt").except("hello.txt should be present");
```

Use `?` to propagate errors (calls from to transform error to return type):
``` rust
fn read_username_from_file() -> Result<String, MyErrorType> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}
```

`?` calls can be chained together:
``` rust
fn read_username_from_file() -> Result<String, MyErrorType> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

Common functionality can be found in standard library:
``` rust
fn read_username_from_file() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
```

`?` can be used for `Option<T>` returns as well:
``` rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

Mixing Option / Result can't be done implicitly, but can with `.ok` and `.ok_or`:
``` rust
fn read_username_from_file() -> Option<String> {
    std::fs::read_to_string("hello.txt").ok()
}
```

To use `?` in main:
``` rust
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::Open("hello.txt")?;

    Ok(())
}
```
