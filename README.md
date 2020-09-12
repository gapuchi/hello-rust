# Notes

This repo contains the code snippets and notes I have written/taken from [The Rust Book](https://doc.rust-lang.org/book/).

## Variable and Mutability

`let` declares a variable. This is **immutable** by default. If you want to make it mutable, add `mut` keyword:

```rust
let x = 5;
let mut y = 6;

x = 7; //Compile Error
y = 7; //No error
```

`const` creates a **constant**. This can never change, and must be set to a constant expression, not something that has to be determined at runtime.

```rust
const MAX_POINTS: u32 = 100_000;
```

**Shadowing** is when you declare a new variable with the same name as a previous one. (The first is **shadowed** by the second variable.) You shadow by using `let` repeatedly:

```rust
let x = 5;
let x = 7;
let x = "howdy";
```

This is different from `mut` because the latter doesn't use `let`, and because of this shadowing allows us to create the variable with a different type. This wouldn't work:

```rust
let mut x = 5;
x = 7; //No error
x = "howdy"; //Error
```

## Data Types

Rust is **statically typed**, which means it knows all the types at compile time. The compiler can infer (usually) what type a variable is, but there are cases where it cannot.

An example is parsing a string:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Without the declared type, compiler will throw an error.

### Scalar Types

**Scalar types** represent a single value. Rust has four primary scalar types:

1. integer
1. floating point numbers
1. boolean
1. characters

### Compound Types

**Compound types** can group multiple values into one type. Rust has two primary compound types:

1. Tuple
1. Array

## Functions

* Statement - instructions that perform some action and do not return a value.
* Expression - evaluate to a resulting value.

This is a statement:

```rust
let x = 5;
```

This is an expression:

```rust
5
```

Statements do not return values. Therefore, you can’t assign a let statement to another variable.

```rust
let x = (let y = 6); //Error
```

Calling a function is an expression (`hello()`), calling a macro is an expression (`println!("Guess the number!")`), and the block used to create new scopes `{}` is an expression:

```rust
{
    let x = 5;
    x - 2
}
```

The above expression evaluates to `3`.

**Expressions do not include semi-colons.** If you add a semi-colon, it becomes a statement and will not return a value.

## Control Flow

### `if` Expressions

`if` statements are pretty common:

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

but in rust, they are expressions!

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

### `loop`

`loop` runs code repeatedly until there is a command to break:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

The `break` keyword can be followed by an expression that is returned from the loop. A `;` follows the `loop` block because it is a statement of assigning a variable.

### `while`

`while` loop is straighforward:

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### `for`

`for` loop is straightforward:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## Ownership

**Ownership Rules**:

1. Each value in Rust has a variable called its *owner*.
1. There can only be one owner at a time.
1. When the owner goes out of scope, the value will be dropped.

### Variable Scope

```rust
let s = "hello";
```

The variable is valid from the point at which it’s declared until the end of the current scope.

```rust
{
    //s is invalid
    let s = "hello"; //s is valid
    //s is valid
} //end of scope, s is invalid
```

### The `String` Type - An Example

The data types mentioned above are stored on the stack and are popped off when moved out of scope. Let's take a look at an example of something stored on the *heap*.

There are string literals, but there are cases where we can't use a literal. Rust has `String` type for this case. We can create one from a literal:

```rust
let s = String::from("hello");
```

Unlike literals, this can be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

Why can this be mutated but not literals? This can be explained by how these are stored.

### Memory and Allocation

With a string literal, we know the contents at compile time. The text is hardcoded directly into the final executable. We can do this because a **literal is immuatable**. We cannot allocate memory for each text that we do not know the size of, or those that might change.

We need allocate memory on the heap to allow storage of unknown memory that may be changing. So

1. The memory must be requested from the memory allocator **at runtine**.
1. The memory needs to be returned to the allocator when we're done with the `String`.

`String::from` does the first part for us. (This basically happens for all languages).

The second part is different. Rust doesn't have a garbage collector like other languages, that clean up for them. We need to do this ourselves.

Well, "by ourselves" really means Rust. The memory is automatically returned once the variable that owns it goes out of scope.

```rust
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no longer valid
}
```

When a variable goes out of scope, Rust calls a special function, `drop`. The author of `String` puts the code to return the memory in this method. Rust calls it automatically.

#### Ways Variables and Data Interact: Move

Multiple variables can interact with the same data:

```rust
let x = 5;
let y = x;
```

`5` is bound to `x` and a copy is bound to `y`. This data is known and stored onto the stack.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```

In this case, `s1` isn't copied to `s2`, because it isn't stored on the stack.

A `String` is made up of three things:

1. A pointer to the memory that holds the content of the string
1. Length
1. Capacity

These three are stored on the stack. The content of the string is stored on the heap. When we assign `s1` to `s2`, the `String` data is copied, meaning the pointer, length, and capacity are assigned to `s2`.

When a variable goes out of scope, Rust calls `drop`. **It is a bug to drop the same memory twice**. This is called a **double free error**. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities. So what happens with `s1` and `s2`, since they both look at the same location in memory?

Rust ensures memory safety by *invalidating `s1`*:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); //Error
}
```

Instead of a *shallow copy*, where the pointer is copied, this is a **move** because `s1` is invalidated. `s1` was *moved* to `s2`. Rust will never automatically *deep copy* your data, as this is expensive.

#### Ways Variables and Data Interact: Clone

If we want to do a **deep copy**, we can use a common method called **clone**:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

#### Stack-Only Data: Copy

You know what's weird:

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

The above doesn't error. Didn't `x` move to `y`? Well this these data types have known size at compile time and are stored on the stack, the copies are made quickly. No need to invalidate the previous variable. There is no such thing as a shallow copy because the actual value gets copied in the stack.

Rust has a `Copy` trait that we can put on data types such as integers. If a type has a `Copy` trait, the older variable is still usable after reassignment. Rust won't let us annotate a type with `Copy` trait if the type or any part of it has implemented the `Drop` trait.

### Ownership and Functions

Passing a value to the function similar is assigning:

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

* `s` is moved to `some_string`. We cannot use `s` after this.

### Return Values and Scope

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

What if we want to let a function use a value but not take ownership? We can return the original variable back:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

It is extra code and an extra process. For these cases, Rust has a concept called *references*.

### References and Borrowing

Here's how a function would take a parameter as a reference without taking ownership:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

We pass `&s1` to the function and the function definition takes `&String`.

The `&` are references (and `*` is a dereference operator, more on that later).

`s: &String` means that `s` is a reference to a String. Once it goes out of scope, the value it reference **does not**, because `s` doesn't own it.

Having references as function parameters is called **borrowing**. Because the function doesn't own it, it cannot modify it.

References are immutable, like variables are by default.

#### Mutable References

To make a reference mutable, we have to:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Couple of things:

1. Change `s` to be `mut`
1. Create mutable reference, `&mut s`
1. Change the function to accept a mutable reference, `some_string: &mut String`

You can have **only one mutable reference to a particular piece of data**:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; //Error

    println!("{}, {}", r1, r2);
}
```

This restriction prevents *data races* from occurring.

We can use `{}` to create a new scope to allow multiple mutable references, just not at the same time:

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
```

You cannot combine mutable and immutable references:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; //Error

    println!("{}, {}, and {}", r1, r2, r3);
}
```

A reference's scope starts from where it is introduced to where it is last used.

So this is fine:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

#### To Summarize

1. You can either have one mutable or any number of immutable references.
1. References must always be valid.

### The Slice Type

An example - Write a function that takes a string and returns the first word it finds in that string. If no word is found, then the whole string should be returned.

The signature of the function:

```rust
fn first_word(s: &String) -> ?
```

We have one parameter that takes in a reference to a string. We do not want ownership, so that's fine. What do we return?

One example is to return an index:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //Convert a string into an array to check every element

    for (i, &item) in bytes.iter().enumerate() { //Create an iterator
        if item == b' ' { //Compare  to the byte literal space
            return i;
        }
    }

    s.len()
}
```

The only problem here - the `usize` returned here only has meaning in the context of `&String`. We have no guarantee that it will be valid in the future. Something could modify the `&String`:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

This compiles without any issues. We need to worry about keeping `word` in sync with `s`. Plus if we decided to get the second word from the string, it becomes more complicated, having to return the start and end indices.

The solution? String slices

#### String Slices

**Slice** references a sequence of elements in a collection, instead of a whole collection. It **does not have ownership**.

A *string slice* is a reference to part of a `String`

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

With Rust's range syntax `..`, you can drop the value before the dots if you want to start at the 0 index. If you want to include the last byte, you can drop the value after `..`. The type that signifies “string slice” is written as `&str`.

A function to find the first word:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Rust compiler ensures that references to the string remain valid. So now this would error at compile time:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

Rust doesn't let you have a mutable reference if you already have an immutable reference. `clear` needs a mutable reference (because it is modifying the string), but we passed an immutable reference to `first_word`.

#### String Literals Are Slices

```rust
let s = "Hello, world!";
```

Recall string literals being stored inside the binary. The type of `s` here is `&str`: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an immutable reference.

#### String Slices as Parameters

One improvement to the signature:

```rust
fn first_word(s: &String) -> &str {
```

can be written as:

```rust
fn first_word(s: &str) -> &str {
```

This allows us to use the same function for `&String` and `&str`.

If we have a string slice, we can pass it directly. If we have a string, we can pass a slice of the whole string.

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

#### Other Slices

We can take a slice of arrays besides strings:

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

The slice has the type `&[i32]`.

## Using Structs to Structure Related Data

A *struct* is a custom data type. It is like an object's data attributes if you're thinking about object-oriented programming.

### Defining and Creating Structs

Structs are similar to tuples. They both can contain elements of different types. In struct, these elements are named. To define the struct:

```rust
struct User {
    username: String, //This is a field
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

We create an *instance* of a struct by defining each of the *fields*.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

The order doesn't matter, because the fields are named. (An advantage over tuples.)

We can use dot notation to get specific attributes from a struct (`user1.email`). If this is mutable we can change it via this way of accessing:

```rust
user1.email = String::from("anotheremail@example.com")
```

The **entire struct must be mutable**. We cannot mark specific fields as mutable.

#### Using the Field Init Shorthand when Variables and Fields Have the Same Name

Let's say we have a function:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

we can just simply have it to be:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### Creating Instances From Other Instances With Struct Update Syntax

There will be cases where we want to create a struct with most of an old struct's field with some changed. We can use the *struct update syntax*.

Instead of:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

we can have:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

The `..` specifies that the remaining fields not set should have the same values as the fields in the given instance.

#### Using Tuple Structs without Named Fields to Create Different Types

You can define structs that look like tupes, called *tupled structs*. They do not have names associated to the fields, rather they types associated to the fields. This is useful when you want to give a tuple a meaning, and make it a different type from other tupes, where field names are redundant.

```rust
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

#### Unit-Like Structs Without Any Fields

You can have structs without any fields. This is useful for when you want to implement a trait that doesn't any data itself.

#### Ownership of Struct Data

The examples above, the fields are owned by the struct. We can have structs with fields that are owned by something else, but to do so we need to use *lifetimes*. Lifetimes esnure that the data references by a struct is valid for as long as the struct is. This will be further explained later.

### An Example Program

[Example Programs](src/ch-5)

### Method Syntax

#### Defining Methods

Let's modify the example in the previous section:

```rust
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

To define the function within a context of `Rect`, we have to define an `impl` (implementation) block. We move the `area` function to this block and change the param to be `self`.

We use `self` instead of `rect: &Rect` because Rust knows the type for `&self`. (We still need to pass in a reference because methods can take ownership, reference immutably, or reference mutably.)

Having a method that takes ownership is rare. Usually we see this only when the method transforms `self` into something else and you want to prevent the caller from using the original.

#### Methods with More Parameters

If you want to use more parameters:

```rust
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### Associated Functions

We can define functions that do not take in `self` in the `impl` block. These are called *associated functions* because they are associated with the struct. They are functions, not methods because they are not associated with an instance. `String::from` is an example.

Associated functions are often used for constructors:

```rust
impl Rect {
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}
```

To call an associated function, we use `::` - `let sq = Rect::square(3);`

#### Multiple Impl Blocks

We have multiple `impl` blocks, and all will be considered. Not sure why we'd want to do this, but it is possible.

## Enums and Pattern Matching

### Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

#### Enum Values

We can create instances like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

We can store additional data inside enums:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

One cool advantage, each instance can have different types of data associated with it:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

There is a [standard library](https://doc.rust-lang.org/std/net/enum.IpAddr.html) for this!

---

Another example of an enum with a wide variety of types in its variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

* `Quit` has no data associated with it at all
* `Move` has a struct associated with it
* `Write` has a string.
* `ChangeColor` has 3 `i32`.

**You can** just create structs for the variants above, but they wouldn't be grouped together:

```rust
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
```

**but** it's more annoying to create a function that takes these 4 types. With an enum, its a single type, `Message`.

**Another similarity** - we can define methods on enums using `impl`.

```rust
impl Message {
    fn call(&self) {
        //method
    }
}

let m = Message::Write(String::from("hello"))
m.call()
```

#### The `Option` Enum and Its Advantage Over Null Values

Basically, the argument for `Option` in Scala. Rust has defined its own [Option](https://doc.rust-lang.org/std/option/enum.Option.html):

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` is an enum, and `Some(T)` and `None` are variants of the enum. (`<T>` is the syntax for generics. Later chapter.)

### The `match` Control Flow Operator

Basically the `match` in Scala.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Pattern that Bind to Value

We can extract values using `match`, kinda like how we can extract values from `case class`es in Scala.

Let's say we have

```rust
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

We can extract the `UsState`:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

#### Matching with `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### Matches Are Exhaustive

**Unlike Scala**, matches in Rust **has to be exhaustive**. There will be a compile error if it isn't:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
} //Won't compile
```

#### The `_` Placeholder

Similar to Scala, `_` can be used as a catch all:

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

### Concise Control Flow with `if let`

Rust combines `if` and `let` to allow a more concise way to handle values that match one pattern while ignoring the rest.

For example, this is pretty wordy:

```rust
let some_value = Some(u08);
match some_value {
    case Some(3) => println!("triple"),
    _ => (),
}
```

We only care for `Some(3)`. We can use `if let`:

```rust
if let Some(3) = some_value {
    println!("triple");
}
```

`if let` takes a pattern and an expression separated by an `=`.

You can use `else` in addition:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

## Managing Growing Projects with Packages, Crates, and Modules

* So far we've written code in one **module** in one **file**.
* As code gets bigger, we can split our code into multiple modules and then in multiple files.
* **Package** contains **multiple binary crater** and **optionally one library crate**. (Why only one library? Idk).
* As your project grows, you can split code into different crates.
* This leads to the concept of **scope**. The context where code is written has a set of names that are defined as *in scope*. You can create scopes and change which names are in and out of the scope. You can't have two itesm with the same name in the same scope.

Rust provides introduces various tools:

<dl>
    <dt>Packages</dt>
    <dd>A Cargo feature that lets you build, test, and share crates</dd>
    <dt>Crates</dt>
    <dd>A tree of modules that produces a library or executable</dd>
    <dt>Modules, use</dt>
    <dd>Lets you control the organization, scope, and privacy of paths</dd>
    <dt>Paths</dt>
    <dd>A way of naming an item, such as a struct, function, or module</dt>
</dl>

### Packages and Crates

<dl>
    <dt>Crate</dt>
    <dd>A binary or library</dd>
    <dt>crate root</dt>
    <dd>A source file that the Rust compiler starts from and makes up the root module of your crate.</dd>
    <dt>Package</dt>
    <dd>One or more crates that provide a set of functionality. It contains a *Cargo.toml* file that explains how to build these crates.</dd>
</dl>

A *package*:

1. **Must contain** at least one crate (either a library crate or binary crate)
1. Cannot contain more than one library crate
1. Can contain as many binary crates as you want

Let's create a new project:

```zsh
~ $ cargo new my-new-project
     Created binary (application) `my-new-project` package
~ $ ls my-new-project 
Cargo.toml src
~ $ ls my-new-project/src
main.rs
~ $ cat my-new-project/Cargo.toml 
[package]
name = "my-new-project"
version = "0.1.0"
authors = ["Spongebob Squarepants <bob.sponge@krustykrab.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
~ $
```

Couple of things to note here:

1. There is a *Cargo.toml* file, indicating that this is a package.
1. *Cargo.toml* has no mention of `src/main.rs`. Cargo follows the convention of:
    * `src/main.rs` - crate root of a binary crate with the same name as the package.
    * `src/lib.rs,` - crate root of a library crate with the same name as the package.
1. Cargo passes the crate root files to `rustc` to build the library or binary.
1. This package only has `src/main.rs` so it only contains a binary crate named `my-project`.
1. If the package contains both `src/main.rs` and `src/lib.rs`, it has two crates:
    1. A binary crate named `my-project`
    1. A library crate named `my-project`
1. A package can have multiple binary crates by placing files under `src/bin`. Each file is a separate binary crate.

*A crate will group related functionality together in a scope so its easy to share among projects.*

For example, the `rand` crate provides the functionality of generating random numbers. We can use this by bringing `rand` crate into our project's scope. All functionality can be through the crate's name, `rand`.

Keeping a crate's functionality in its own scope prevents conflicts. For example, `rand` provides a trait `Rng`. We can also create a struct `Rng` in our own crate. We can bring in `rand` as a dependency and the compiler wouldn't be confused on which `Rng` we're using. In our crate it refers to our struct `Rng`. If we wanted to use the one in `rand`, we'd access it by saying `rand::Rng`.

### Defining Modules to Control Scope and Privacy

**Modules** let us organize code within a crate into groups for readability and easy reuse. It also controls **privacy**.

Example, a module for restaurant functionality. We create `src/lib.rs` to create a library crate. Inside this class, we can define modules and functions:

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

We use `mod` keyword to define a module and use `{}` to define the mody of the module. 

Inside a module, we can define other modules and hold definitions for other items, such as structs, enums, constants, traits, or functions.

You know how `src/lib.rs` and `src/main.rs` are called *crate root*s? This is because the contents of these files form the a module called `crate` that is at the root of the module tree.

```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Things to note:

1. `hosting` *nests* inside `front_of_house`
1. `hosting` is *siblings* with `serving`
1. `hosting` is the *child* of `front_of_house`
1. `front_of_house` is the *parent* of `hosting`

### Paths for Referring to an Item in the Module Tree

We use paths to find an item in the module tree structure.

A path can take two forms:

1. *Absolute path* - starts from a crate root by using a crate name or a literal `crate`
1. *Relative path* - starts from the current module and uses `self`, `super`, or an identifier in the current module.

The identifiers are separated by `::` in a path.

So if we wanted to access `add_to_waitlist` from the root:

```rust
mod front_of_house { ... }

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}
```

This actually won't compile because **`hosting` is private**.

* Modules define privacy boundaries. All items are private by default.
* Parent modules cannot access private items inside child modules
* Child modules can access private items in their ancestor modules.
    * This is because child hides implementation from the parent, but the child is aware of the context they're defined in.
* Use `pub` to make an item public.

#### Exposing Paths with the pub Keyword

Let's make that function accessible:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Note:

* We had to make both the `hosting` and `add_to_waitlist` public.
* We didn't have to make `front_of_house` public. `eat_at_restaurant` is siblings with `front_of_house` so it can access it.

#### Starting Relative Paths with `super`

tldr - use `super` to up one module when using relative paths.

#### Making Structs and Enums Public

We use `pub` to make structs and enum public. Things to note:

1. Marking a struct public doesn't make the fields public. We have to mark whichever fields we want to be public with `pub` as well.
1. If a struct has private fields, it needs a public associated function that constructs an instance of the struct.
    1. Otherwise we can't create an instance outside of the module. (Is this required if we don't want to access it outside of the module? My guess is it is because why else would we mark the struct as `pub`?)

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

1. Marking an enum public makes all its variants public. You only need to mark the enum as `pub`.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### Bringing Paths into Scope with the `use` Keyword

Instead of declaring the whole path each time we want to use an item, we can bring the path into scope with `use`.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

With this, `hosting` can be treated as if *it was defined in the scope*

You can also use with relative paths.

#### Creating Idiomatic use Paths

Why not specify the `use` path all the way to `add_to_waitlist`? *This is possible*, but the **idiomatic way is to bring the module into scope, not the function**. This way we can make it apparent that the function isn't defined in the scope.

**The idiomatic way to bring in strucst, enums, and other items is to use the full path**. Weird right?

There really isn't a reason, just the convention that formed.

The only limitation is that we can't bring two items with the same name into the same scope:

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
}
```

We clarify `Result` by using the `fmt` or `io` module. We couldn't do 

```rust
use std::fmt::Result;
use std::io::Result;
```

because Rust wouldn't know which one to use if we were to refer to `Result`.

#### Providing New Names with the as Keyword

A work around to bring in items with the same name is to rename an item with `as` keyworkd:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

#### Re-exporting Names with pub use

* `use` brings a name into the scope, but the name is private
* If we want to allow external code to access this name in given module, we can slap on a `pub` in front of `use`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

Things to note:

1. External code can call `add_to_waitlist` by `hosting::add_to_waitlist`.
1. External code *cannot* call `add_to_waitlist` because `front_of_house` is not public.

<dl>
    <dt>When to use?</dt>
    <dd>The internal structure of your code differs from how users would think about the domain. (e.g. users wouldn't not distinguish "back of house" and "front of house")</dd>
</dl>

#### Using External Packages

To use (for example) `rand` package, we add this line to *Cargo.toml*:

```rust
[dependencies]
rand = "0.0.5"
```

then to bring `rand` into scope, we add a `use` line:

```rust
use rand::Rng; //Starts with the crate name (rand) and then followed by the item/module etc.
```

`std` (standard library) is an external package that is shipped with Rust, so we do not neet to declare it as a dependency, but we do need to declare a `use` statement if we want to use something from it.

#### Using Nested Paths to Clean Up Large `use` Lists

```rust
use std::cmp::Ordering;
use std::io;
```

can be simplified to:

```rust
use std::{cmp::Ordering, io};
```

and

```rust
use std::io;
use std::io::Write;
```

can be simplified to:

```rust
use std::io::{self, Write};
```

#### Glob Operator

If we want to bring in all public items in a path, use `*` (the *glob operator*):

```rust
use std::collections::*;
```

### Separating Modules into Different Files

We want to split modules into different files when it gets too big.

Let's move `front_of_house` into its own file. We create `src/front_of_house.rs`:

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

and change `src/lib.rs` to be:

```rust
mod front_of_house; //The semi-colon instead of a body tells Rust to load the contents from another file with the same name as the module.

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

Let's extract `hosting` to its own file:

```rust
// src/front_of_house.rs
pub mod hosting;
```

and we create `src/front_of_house/hosting.rs`:

```rust
pub fn add_to_waitlist() {}
```

A way to think about the structure is to look at the path: `crate::front_of_housing::hosting` can be converted to `src/front_of_housing/hosting.rs`.
