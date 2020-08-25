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

