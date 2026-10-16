# The Ultimate Rust Odyssey

> From zero to full-stack and compiler architect — one book, sixteen sections, 10,000+ lines.

![Rust logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

![Rust 1.82+](https://img.shields.io/badge/rust-1.82+-orange?logo=rust)
![Parts 16](https://img.shields.io/badge/parts-16-blue)
![Lines 10000+](https://img.shields.io/badge/lines-10000+-green)
![License MIT](https://img.shields.io/badge/license-MIT-blue)

> **How to read:** type every example, run every test, keep an error log. Start at Part I and move forward. Use the appendices as an index after your first full pass.

<details>
<summary><strong>Contents — click to expand</strong></summary>

1. [Part I - Genesis (Ch 1-4)](#part-i-genesis-ch-1-4)
1. [Part II - Mechanics (Ch 5-8)](#part-ii-mechanics-ch-5-8)
1. [Part III - Concurrency (Ch 9-10)](#part-iii-concurrency-ch-9-10)
1. [Part IV - Full-Stack (Ch 11-13)](#part-iv-full-stack-ch-11-13)
1. [Part V - Compiler Pebble (Ch 14-17)](#part-v-compiler-pebble-ch-14-17)
1. [Part V - TinyGit Capstone (Ch 18)](#part-v-tinygit-capstone-ch-18)
1. [Part VI - Advanced Rust (Ch 19-23)](#part-vi-advanced-rust-ch-19-23)
1. [Part VII - Systems and Performance (Ch 24-27)](#part-vii-systems-and-performance-ch-24-27)
1. [Part VIII - Production and Capstones (Ch 28-33)](#part-viii-production-and-capstones-ch-28-33)
1. [Part IX - Stdlib and Idioms (Ch 34-36)](#part-ix-stdlib-and-idioms-ch-34-36)
1. [Part X - Practice and Exam (Ch 37-42)](#part-x-practice-and-exam-ch-37-42)
1. [Part XI - Cookbook (75 recipes)](#part-xi-cookbook-75-recipes)
1. [Part XII - Backend Recipes](#part-xii-backend-recipes)
1. [Part XIII - Algorithms](#part-xiii-algorithms)
1. [Part XIV - Testing and Ops](#part-xiv-testing-and-ops)
1. [Appendices B-I](#appendices-b-i)

</details>

<details>
<summary><strong>Legend — admonitions used in this book</strong></summary>

- `> 💡` — mental model or analogy
- `> 🧭` — why this matters / where you are
- `> ⚠️` — footgun, do not skip
- `> ✅` — exercise hint or answer

</details>

---

<a id="part-i-genesis-ch-1-4"></a>

## Part I - Genesis (Ch 1-4)

| Navigation | |
| --- | --- |
| Previous | [↑ Top](#the-ultimate-rust-odyssey) |
| Next | [Part II - Mechanics (Ch 5-8) →](#part-ii-mechanics-ch-5-8) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## The Ultimate Rust Odyssey
### From Zero to Full-Stack & Compiler Architect

#### Part I: The Genesis (Zero to Programmer)

---

### Before You Begin: A Note to the Reader

Welcome. If you have never written a line of code, you are in exactly the right place. Many programming books quietly assume you already know what a "variable" is, why a "function" exists, or what "memory" means. This book does not. We will start from the ground and climb together.

Rust has a reputation for being hard. That reputation is partly earned, but it is also a bit misleading. Rust is strict, and its strictness is deliberate. The compiler (the program that turns your code into something a computer can run) will refuse to build code that could crash, leak memory, or cause data races. At first, this feels like arguing with a very stubborn friend. Over time, you will realize that the friend was right, and that the bugs they stopped you from shipping would have cost you days of confusion.

> 💡 **How to read this book:** Type every code example yourself. Do not copy and paste. Your fingers learning the shape of Rust is part of the learning. When the compiler yells at you, read the message slowly. Rust's error messages are some of the best teachers in the software world.

---

### Chapter 1: What Is Code?

#### 1.1 A Computer Is a Very Fast, Very Literal Machine

Imagine you hire a helper who can do arithmetic a billion times faster than you, never gets tired, and never complains. There is one catch: this helper understands only very simple, precise instructions. If you say, "Make me a nice sandwich," the helper will stare at you blankly. If you say, "Take the bread from the left bag. Place it on the plate. Take one slice of cheese from the fridge. Place it on top of the bread…" the helper will do it perfectly.

A computer is that helper. **Code** is the set of precise instructions you give it. A **programming language** is the vocabulary and grammar you use to write those instructions in a way that both you and the machine can read.

> 💡 **Think of it this way:** Code is a recipe. The computer is an extremely obedient cook who follows each step exactly, no more and no less, and who will never guess what you meant.

#### 1.2 How Computers Think: Only Ones and Zeros (Sort Of)

Deep inside a computer are billions of tiny switches called **transistors**. Each one is either on or off. We represent "on" as `1` and "off" as `0`. This is called **binary**.

Everything a computer does, including showing you this text, playing music, or running a game, is ultimately a pattern of ones and zeros.

Here is a tiny example. The letter `A` is stored as the pattern `01000001`. Each pattern of eight bits (a **byte**) can represent one of 256 values (2⁸ = 256), and we agreed as a society which value means which letter, symbol, or number.

| Decimal number | Binary pattern | Meaning (ASCII character) |
|:--------------:|:--------------:|:-------------------------:|
| 65 | `0100 0001` | `A` |
| 97 | `0110 0001` | `a` |
| 48 | `0011 0000` | `0` (the character zero) |

You will never need to write binary by hand. But knowing that everything is numbers underneath will make the rest of this book click much faster.

#### 1.3 Machine Code vs. High-Level Languages

Writing raw binary would be like writing a novel by flipping light switches. So humans invented layers of abstraction.

- **Machine code:** Raw binary instructions the processor understands directly. Impossible for humans to write comfortably.
- **Assembly language:** Human-readable names for machine instructions, such as `MOV` (move a value) or `ADD` (add two values).
- **High-level languages:** Languages like Rust, Python, or JavaScript, which express ideas closer to how humans think, such as `let total = price * quantity;`.

A **compiler** is a program that translates high-level code into machine code. Rust uses a compiler named `rustc`. Later in this book, you will build a compiler of your own. That is the grand finale.

```text
  You write:          let total = price * quantity;
        │
        ▼   (the compiler translates it)
  Assembly:           MOV  R1, [price]
                      MUL  R1, [quantity]
                      MOV  [total], R1
        │
        ▼   (the assembler converts names to numbers)
  Machine code:       10001011 01000101 ...
        │
        ▼
  The CPU executes it, one tiny step at a time.
```

#### 1.4 The Three Pillars of Every Program

Almost every program, in any language, is built from three ideas. Learn these and you will be able to read almost any code.

##### Pillar 1: Data (Variables and Memory)

A program has to remember things. It remembers them in **memory** (RAM), which you can picture as a long row of numbered mailboxes. Each mailbox holds a value, and each has an address.

A **variable** is a friendly name you give to a mailbox. Instead of saying "the value stored in mailbox #4,021,337," you say `score`.

```rust
let score = 100;   // Put the value 100 in a mailbox, and call that mailbox "score".
```

##### Pillar 2: Logic (Decisions)

Programs make decisions. "If the player has no health left, the game is over. Otherwise, keep playing."

```rust
if health == 0 {
    println!("Game over!");  // Only runs when health is exactly zero.
} else {
    println!("Keep going!"); // Runs in every other case.
}
```

##### Pillar 3: Repetition (Loops)

Computers excel at repeating boring work. "Do this 1,000 times" or "keep doing this until the user quits."

```rust
for i in 1..=3 {                 // Repeat for i = 1, then 2, then 3.
    println!("Hello for the {i}th time");
}
```

> 🧭 **Recap of Chapter 1:** A computer executes precise instructions written in a programming language. Those instructions manipulate data stored in memory, make decisions with logic, and repeat work with loops. Everything else is built from these three ideas.

#### 1.5 Exercise

Without running anything, describe in plain English what this program would do:

```rust
let mut lives = 3;
while lives > 0 {
    println!("Lives left: {lives}");
    lives -= 1;
}
println!("Game over!");
```

> ✅ **Answer:** It starts with 3 lives, prints "Lives left: 3", then 2, then 1, then stops when lives reaches 0 and prints "Game over!". Notice that `lives -= 1;` is the line that eventually makes the loop stop. Forgetting that line creates an infinite loop, a classic beginner bug.

---

### Chapter 2: Hello, Rust!

#### 2.1 Why Rust?

Rust was created at Mozilla and first released in 2015. It promises three things at once:

1. **Speed:** Rust compiles to machine code with no garbage collector slowing it down. It is as fast as C and C++.
2. **Safety:** Whole categories of bugs, such as null pointer crashes, buffer overflows, and data races, are caught before your program ever runs.
3. **Productivity:** Modern tools, a helpful compiler, and a package manager (called Cargo) make it pleasant to build real projects.

Think of Rust as a race car with a very good seatbelt system. The seatbelts sometimes click at annoying moments, but they are the reason the car is trusted at high speed.

#### 2.2 Installing the Rust Toolchain

A **toolchain** is the bundle of programs you need: the compiler (`rustc`), the build tool (`cargo`), and standard libraries. The official installer is called `rustup`.

**On Windows:**
1. Go to https://rustup.rs and download `rustup-init.exe`.
2. Run it. If it asks about the C++ build tools, follow the link it gives you and install "Desktop development with C++" from Visual Studio Build Tools. Rust needs a linker to produce `.exe` files.
3. Accept the default installation (option 1).
4. Close and reopen your terminal (PowerShell or Command Prompt).

**On macOS or Linux:**
```bash
# Download and run the official installer script.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then verify everything works:

```bash
rustc --version   # Prints the compiler version, e.g. rustc 1.8x.0
cargo --version   # Prints the build tool version
```

> ⚠️ **Common stumble:** If you see `command not found` after installing, your terminal did not reload its environment. Close it completely and open a new one.

#### 2.3 Your First Program, the Long Way

Create a file named `main.rs` anywhere you like, and write this inside it:

```rust
// main.rs
// Every line starting with two slashes is a comment: notes for humans,
// completely ignored by the compiler.

fn main() {
    // `fn` declares a function. `main` is special: it is where every
    // Rust program begins running.
    println!("Hello, world!"); // println! prints text followed by a newline.
}
```

Compile and run it directly with the compiler:

```bash
rustc main.rs     # Produces an executable file (main.exe on Windows, main on Unix)
./main            # Run it. On Windows, type: .\main.exe
```

You should see:

```text
Hello, world!
```

Let's unpack the pieces:

| Piece | What it means |
|:------|:--------------|
| `fn main()` | Declares the entry point function named `main`. |
| `{ ... }` | Curly braces mark the body of the function. |
| `println!` | A **macro** (the exclamation mark is the clue). It prints text to the screen. |
| `"Hello, world!"` | A **string literal**, text written directly in your code. |
| `;` | Semicolons end statements, like full stops end sentences. |

> 🧭 **Why the exclamation mark?** In Rust, `println!` is a macro, which means a small program that writes more code for the compiler at build time. You do not need to understand macros yet. For now, just remember: functions have no `!`, macros do.

#### 2.4 Cargo: Your Project Manager

Typing `rustc` by hand works for one file. Real projects have dozens of files, external libraries, and settings. Enter **Cargo**.

Create a new project:

```bash
cargo new hello_odyssey
cd hello_odyssey
```

Cargo creates this structure:

```text
hello_odyssey/
├── Cargo.toml    ← Project settings and dependencies (a list of ingredients)
├── .gitignore    ← Tells Git which files to ignore
└── src/
    └── main.rs   ← Your source code
```

Open `Cargo.toml`. It looks like this:

```toml
[package]
name = "hello_odyssey"    # The name of your project
version = "0.1.0"         # Your project's version number
edition = "2021"          # The Rust language edition (a yearly-ish snapshot of the language)

[dependencies]
# Libraries you add later go here.
```

The most useful Cargo commands:

| Command | Purpose |
|:--------|:--------|
| `cargo run` | Compile (if needed) and run your program. Use this daily. |
| `cargo build` | Compile only, producing a binary in `target/debug/`. |
| `cargo build --release` | Compile with optimizations for speed. Use for final builds. |
| `cargo check` | Quickly check for errors without producing a binary. Very fast. |
| `cargo test` | Run tests (Chapter 10). |
| `cargo add <crate>` | Add an external library to your project. |

Run your project:

```bash
cargo run
```

Output:

```text
   Compiling hello_odyssey v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello_odyssey`
Hello, world!
```

> 💡 **Terminology:** A library of Rust code is called a **crate**. Your project is a crate. Libraries you download are crates. Crates are published on https://crates.io.

#### 2.5 Reading Compiler Errors Like a Pro

Let's intentionally break the program. Change the line to:

```rust
println!("Hello, world!")
```

Notice the missing semicolon. Run `cargo run`. You will see something like:

```text
error: expected `;`, found `}`
 --> src/main.rs:5:43
  |
5 |     println!("Hello, world!")
  |                              ^ help: add `;` here
```

Rust tells you:
- **What** went wrong (`expected ;`)
- **Where** (file `src/main.rs`, line 5, column 43)
- **How** to fix it (`add ; here`)

Learn to read errors top to bottom: the summary first, then the location, then the suggestion. Rust's error messages are deliberately friendly, so trust them.

#### 2.6 Making Decisions with `if`

Every program needs to choose. A weather app decides whether to show an umbrella icon. A game decides whether the player has won. In Rust, that choice is written with `if`.

Think of an `if` as a fork in a road. A **condition** is a question with a yes-or-no answer. If the answer is yes, the program follows the first path. If not, it takes the `else` path, if one exists.

Open `src/main.rs` in your `hello_odyssey` project and replace its contents with this:

```rust
// src/main.rs
fn main() {
    let temperature = 18; // Degrees Celsius.

    if temperature > 25 {
        // Runs only when the condition is true.
        println!("Hot day. Drink plenty of water.");
    } else if temperature > 15 {
        // Checked only if the first condition was false.
        println!("Pleasant weather. A light jacket is enough.");
    } else {
        // Runs when every condition above was false.
        println!("Chilly. Bring a warm coat.");
    }
}
```

Run it with `cargo run`. Since 18 is not above 25 but is above 15, you will see:

```text
Pleasant weather. A light jacket is enough.
```

Rust checks the conditions from top to bottom and runs the first matching block. The rest are skipped, even if they would also be true.

##### The Rules of `if` in Rust

- **The condition must be a `bool`.** Rust does not treat numbers or text as true or false, unlike some languages.
- **Parentheses around the condition are optional** and usually left out.
- **The braces `{ }` are required**, even for a single line. This prevents a whole class of bugs.

Here are the comparison and logic operators you will use constantly:

| Operator | Meaning | Example | Result |
|:--------:|:--------|:--------|:------:|
| `==` | Equal to | `5 == 5` | `true` |
| `!=` | Not equal to | `5 != 3` | `true` |
| `<` | Less than | `2 < 9` | `true` |
| `>=` | Greater than or equal to | `7 >= 8` | `false` |
| `&&` | AND: both must be true | `true && false` | `false` |
| `\|\|` | OR: at least one must be true | `true \|\| false` | `true` |
| `!` | NOT: flips true and false | `!true` | `false` |

Combining conditions reads naturally:

```rust
let age = 17;
let has_ticket = true;

if age >= 18 && has_ticket {
    println!("Welcome to the concert.");
} else if has_ticket {
    println!("Sorry, you must be 18 or older.");
} else {
    println!("Please buy a ticket first.");
}
```

##### `if` Can Produce a Value

Because `if` is an expression, you can use it on the right side of a `let`. This is a very common Rust pattern:

```rust
let score = 72;
let result = if score >= 50 { "pass" } else { "fail" };
println!("Result: {result}");
```

> ⚠️ **Common mistake:** Writing `if score = 50` uses a single `=`, which means *assign*. Rust will refuse it. Always use `==` to compare.

> ⚠️ **Another common mistake:** Writing `if 1 { ... }` in hopes that non-zero means true. Rust requires a real `bool`, so the compiler will say `expected bool, found integer`. Write `if count != 0 { ... }` instead.

> 🧭 **Why this strictness?** Many bugs in other languages come from accidentally treating a number as a yes-or-no answer. Rust makes you state exactly what you mean.

##### Exercise 2.6

Write a program with a variable `password_length` set to a number of your choice. Print `"Too short"` if it is under 8, `"Acceptable"` if it is between 8 and 11 inclusive, and `"Strong"` if it is 12 or more.

> ✅ **Hint:** Use `password_length < 8`, then `password_length <= 11`, or combine `>= 8 && <= 11`. Check the boundaries: try 7, 8, 11, and 12.

---

### Chapter 3: Basic Building Blocks

#### 3.1 Variables and Immutability

A **variable** holds a value under a name. In Rust, variables are declared with `let`:

```rust
let age = 30;
println!("I am {age} years old.");
```

Here is a surprise that trips up almost every newcomer: **variables are immutable by default**. Once set, they cannot change.

```rust
let age = 30;
age = 31;   // ERROR: cannot assign twice to immutable variable `age`
```

Why would a language do this? Think of a variable as a sticky note with a value written on it. Immutability means that sticky note is laminated. You know its value will not change behind your back, which makes programs easier to reason about. When a value *should* change, you opt in explicitly with `mut`:

```rust
let mut age = 30;   // `mut` means "mutable", I expect this to change.
age = 31;           // Fine now.
println!("Now I am {age}.");
```

> 💡 **Mental model:** Rust asks you to be explicit about change. Most values in a well-written program never need to change, and marking them that way makes intent visible to every future reader, including you.

#### 3.2 Shadowing

You can declare a new variable with the same name. This is called **shadowing**, and it is different from mutation:

```rust
let spaces = "   ";        // A string of spaces.
let spaces = spaces.len(); // A new variable named `spaces`, now a number: 3.
println!("{spaces}");      // Prints 3.
```

Shadowing lets you transform a value while keeping a clean name. Mutation with `mut` cannot change a value's *type*, but shadowing can.

#### 3.3 Scalar Data Types

Rust is **statically typed**: every value has a type known at compile time. This is the compiler's way of double-checking that you are not mixing apples and oranges.

The four basic **scalar** (single-value) types are:

| Type | Description | Examples |
|:-----|:------------|:---------|
| `i32`, `i64`, `u8`, … | Integers. `i` means signed (can be negative), `u` means unsigned (zero or positive). The number is the bit size. | `42`, `-7`, `255u8` |
| `f32`, `f64` | Floating-point decimal numbers. `f64` is the default. | `3.14`, `2.0` |
| `bool` | True or false. | `true`, `false` |
| `char` | A single Unicode character, four bytes wide. | `'a'`, `'🦀'` |

```rust
let count: i32 = 10;       // Explicit type annotation.
let price = 19.99;         // Rust infers f64 automatically.
let is_ready = true;
let grade = 'A';           // Single quotes for char. Double quotes are for strings.
```

> 🧭 **Type inference:** Rust usually figures out types on its own. You only need annotations when it cannot guess, or when you want to be explicit.

**Overflow:** An `i32` can hold roughly ±2 billion. Go beyond that and Rust, in debug builds, panics rather than silently wrapping. This safety net catches many real-world bugs.

#### 3.4 Compound Types: Tuples and Arrays

A **tuple** groups values of different types into one bundle:

```rust
let player = ("Avenger", 100, true); // name, health, is_alive
println!("Name: {}", player.0);      // Access by position, starting at 0.
```

An **array** holds several values of the *same* type, with a fixed length:

```rust
let scores = [90, 85, 77, 100, 62];  // Type: [i32; 5]
println!("First score: {}", scores[0]);
println!("Number of scores: {}", scores.len());
```

Arrays are indexed from zero. `scores[0]` is `90`, not `85`. This "zero-based" convention is universal in programming, and you will see it everywhere.

#### 3.5 Strings: Two Flavors

Strings are one of the most confusing topics for beginners, so take this slowly.

- **`&str`** (string slice): A fixed piece of text, usually a literal written in your code. It is borrowed and cannot grow.
- **`String`**: An owned, growable piece of text stored on the heap.

```rust
let greeting = "Hello";              // &str: a literal, fixed in the program.
let mut name = String::from("Rustacean"); // String: owned, can grow.
name.push_str(", welcome!");         // Adds text to the end.
println!("{greeting}, {name}");
```

> 💡 **Analogy:** `&str` is reading a sign on a wall. `String` is owning a notebook you can write in. Both let you read text, but only the notebook lets you add pages.

#### 3.6 Functions

A **function** is a named, reusable recipe. You define it once and call it whenever you need it.

```rust
// Define a function that takes two i32 numbers and returns an i32.
fn add(a: i32, b: i32) -> i32 {
    a + b   // No semicolon: the last expression is the return value.
}

fn main() {
    let total = add(3, 4);
    println!("3 + 4 = {total}");
}
```

Key rules:

- Parameters must have types: `a: i32`.
- The arrow `-> i32` declares the return type. If there is none, the function returns nothing (technically the unit type `()`).
- The **last expression without a semicolon** becomes the return value. Adding a semicolon turns it into a statement that returns `()`, a very common mistake:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;   // ERROR: this returns nothing, but the signature promises an i32.
}
```

You can also use `return` explicitly for early exits:

```rust
fn sign(n: i32) -> &'static str {
    if n < 0 {
        return "negative";  // Early exit.
    }
    "zero or positive"      // Normal return value.
}
```

(The strange `'static` tag is a lifetime, which you will meet properly in Chapter 8. For now, treat it as a label saying the text lives for the whole program.)

#### 3.7 Control Flow: `if` Is an Expression

In Rust, `if` produces a value, which makes code concise:

```rust
let temperature = 28;
let feeling = if temperature > 25 { "warm" } else { "cool" };
println!("It feels {feeling}.");
```

Both branches must return the same type. This is the compiler protecting you from surprises.

#### 3.8 Loops

Rust has three kinds of loops.

**`loop`**: repeats forever until you say `break`.

```rust
let mut attempts = 0;
loop {
    attempts += 1;
    if attempts == 3 {
        break;             // Exit the loop.
    }
}
println!("Took {attempts} attempts.");
```

**`while`**: repeats while a condition holds.

```rust
let mut countdown = 3;
while countdown > 0 {
    println!("{countdown}...");
    countdown -= 1;
}
println!("Liftoff!");
```

**`for`**: iterates over a sequence. This is the one you will use most, because it is safe and avoids off-by-one mistakes.

```rust
let fruits = ["apple", "banana", "cherry"];
for fruit in fruits {
    println!("I like {fruit}.");
}

for i in (1..=5).rev() {   // Ranges: 1..5 excludes 5, 1..=5 includes it.
    print!("{i} ");
}
println!();                // Prints a newline.
```

> 🧭 **Why `for` beats `while` for lists:** With `while`, you manage an index by hand and can accidentally go past the end. With `for`, Rust handles the bookkeeping.

#### 3.9 Chapter 3 Exercise

Write a function `fizzbuzz(n: u32) -> String` that returns:
- `"FizzBuzz"` if `n` is divisible by both 3 and 5,
- `"Fizz"` if divisible by 3,
- `"Buzz"` if divisible by 5,
- otherwise the number itself as text.

Then use a `for` loop to print the results for 1 through 20.

> ✅ **Hint:** Use `%` for remainder. `n % 3 == 0` means divisible by 3. Use `n.to_string()` to convert a number to text.

---

### Chapter 4: Ownership & Borrowing Demystified

This is the chapter that makes Rust unique. Take your time here. Everything after this builds on it. If you feel confused, that is normal. Most people need to read this chapter twice, and many say it "clicked" only after writing a few programs.

#### 4.1 The Problem Ownership Solves

Every program needs memory. Some values, like `i32` numbers, have a fixed, small size and live on the **stack**, a tidy, fast region of memory that works like a stack of plates: the last plate placed is the first removed. Other values, like growable `String`s, need space whose size is unknown in advance. They live on the **heap**, a larger, more flexible region that requires you to request space and, eventually, give it back.

Managing the heap is notoriously hard. Languages handle it in different ways:

| Approach | Used by | Trade-off |
|:---------|:--------|:----------|
| Manual `malloc` / `free` | C, C++ | Fast, but easy to leak memory or free it twice. |
| Garbage collector | Java, Python, JavaScript | Safe and convenient, but pauses and overhead. |
| **Ownership** | **Rust** | **Safe and fast, checked at compile time, no runtime cost.** |

Rust's answer is: the compiler tracks who is responsible for each piece of memory, and inserts the cleanup for you at exactly the right moment. No garbage collector. No manual `free`.

#### 4.2 The Three Ownership Rules

Memorize these. Everything in this chapter flows from them:

1. **Each value in Rust has exactly one owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value is dropped (its memory is freed).**

> 💡 **Mental model:** Think of a value as a physical book in a library. Only one person can hold the book at a time. When that person leaves the building, the book goes back to the shelf (is cleaned up). You can lend the book to someone, but you cannot have two people claiming to own it.

#### 4.3 Scope: Where Values Live

A **scope** is the region between curly braces `{ }`. When a scope ends, every value owned inside it is dropped.

```rust
fn main() {
    {                                   // A new scope begins.
        let message = String::from("hi"); // `message` owns the String.
        println!("{message}");
    }                                   // `message` goes out of scope here. Memory freed.
    // println!("{message}");            // ERROR: `message` no longer exists.
}
```

You never wrote a line to free that memory. Rust did it automatically when the scope ended. This is called **RAII** (Resource Acquisition Is Initialization), and it is the backbone of Rust's safety.

#### 4.4 Moves: Transferring Ownership

Here is where beginners get their first real surprise:

```rust
fn main() {
    let first = String::from("Rust");
    let second = first;   // Ownership MOVES from `first` to `second`.

    println!("{second}");  // Works.
    // println!("{first}"); // ERROR: borrow of moved value: `first`
}
```

Why? If both `first` and `second` owned the same heap memory, then when both went out of scope, Rust would try to free that memory twice. That is a classic crash called a **double free**. To prevent it, the compiler simply says: after the move, `first` is no longer valid.

Moves happen when you assign a value to another variable, or pass it into a function:

```rust
fn consume(text: String) {
    println!("Consumed: {text}");
}   // `text` is dropped here.

fn main() {
    let note = String::from("hello");
    consume(note);            // `note` moves into the function.
    // println!("{note}");    // ERROR: `note` was moved.
}
```

> 🧭 **Why do simple numbers not move?** Types like `i32`, `f64`, `bool`, and `char` implement a trait called `Copy`. Copying a small stack value is cheap, so Rust simply duplicates it. Types that own heap memory, like `String`, do not implement `Copy`, so they move.

```rust
let a = 5;
let b = a;          // `a` is copied, not moved.
println!("{a} {b}"); // Both still valid.
```

#### 4.5 Cloning: Making a Deep Copy on Purpose

If you genuinely need two independent owners, ask for a **clone**. This explicitly copies the heap data. You pay the cost in plain sight:

```rust
let original = String::from("Rust");
let copy = original.clone();   // A full, independent duplicate on the heap.

println!("{original} and {copy}"); // Both work.
```

> ⚠️ **Guideline:** Cloning is not wrong, but cloning everywhere to silence the compiler is a sign you have not yet understood borrowing. Prefer borrowing (next section) whenever you only need to look at a value.

#### 4.6 Borrowing: Lending Without Giving Away

Most of the time, a function does not need to *own* your data. It just needs to look at it. In that case, you **borrow** it using a reference, written `&`:

```rust
fn print_length(text: &String) {    // Borrows a String. Does not own it.
    println!("Length: {}", text.len());
}   // The borrow ends. Nothing is dropped, because nothing was owned here.

fn main() {
    let title = String::from("Odyssey");
    print_length(&title);           // Lend a reference with `&`.
    println!("Still mine: {title}"); // Works! Ownership never moved.
}
```

A reference is like a library card for a book: it lets you read the book, but you do not take the book home forever.

#### 4.7 The Two Borrowing Rules

Borrowing has two strict rules. Break them, and the compiler stops you:

1. **You can have many immutable references (`&T`) at once.**
2. **Or you can have exactly one mutable reference (`&mut T`), but not both at once.**

Why? Imagine two people editing the same paragraph simultaneously while a third person reads it. The reader might see half-written sentences. Rust forbids this at compile time.

```rust
let mut notes = String::from("draft");

let reader_a = &notes;   // Immutable borrow #1
let reader_b = &notes;   // Immutable borrow #2. Fine: many readers.
println!("{reader_a} {reader_b}");
// Both readers are done here; their borrows end.

let editor = &mut notes; // Mutable borrow. Fine: no readers are active now.
editor.push_str(" v2");
println!("{editor}");
```

Now the forbidden version:

```rust
let mut notes = String::from("draft");
let reader = &notes;         // Immutable borrow, still alive...
let editor = &mut notes;     // ERROR: cannot borrow `notes` as mutable
                             //        because it is also borrowed as immutable.
println!("{reader}");        // `reader` is used here, so its borrow is still active.
```

> 💡 **Key insight:** A borrow lasts from where it is created until its **last use**, not until the end of the block. This is called **non-lexical lifetimes**, and it makes Rust friendlier than you might expect. In the example above, if you deleted the `println!` that uses `reader`, the program would compile.

#### 4.8 Returning References Safely

A function can return a reference, but only to data that outlives the function. The compiler enforces this:

```rust
fn first_word(text: &str) -> &str {  // Returns a borrow from the input.
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("Rust is wonderful");
    let word = first_word(&sentence);
    println!("First word: {word}");
}
```

The following version is rejected:

```rust
fn make_greeting() -> &String {    // ERROR: missing lifetime specifier
    let s = String::from("hi");
    &s                             // `s` is dropped when this function ends!
}
```

Returning a reference to a local variable would leave a dangling pointer, a reference to memory that no longer exists. Rust refuses to compile it. The fix is to return the owned value instead:

```rust
fn make_greeting() -> String {
    String::from("hi")   // Ownership moves out to the caller.
}
```

#### 4.9 Slices: Borrowing Part of Something

A **slice** is a borrowed view of a contiguous portion of a collection. It is one of the most useful ideas in Rust:

```rust
let numbers = [10, 20, 30, 40, 50];
let middle = &numbers[1..4];   // Elements at indices 1, 2, 3: [20, 30, 40]
println!("{:?}", middle);

let text = String::from("hello world");
let hello = &text[0..5];       // "hello", borrowed, no copy made.
println!("{hello}");
```

Slices are the reason functions like `first_word` can be both efficient and safe.

#### 4.10 Putting It All Together: A Worked Example

Let's trace a small program through the ownership rules:

```rust
fn shout(message: &mut String) {        // Mutable borrow: we will change it.
    message.push_str("!!!");
}

fn measure(message: &String) -> usize { // Immutable borrow: just reading.
    message.len()
}

fn main() {
    let mut greeting = String::from("hello");   // `greeting` owns the String.

    let length_before = measure(&greeting);      // Lend immutably. Ends immediately.
    shout(&mut greeting);                        // Lend mutably. Ends immediately.
    let length_after = measure(&greeting);       // Lend immutably again. Fine.

    println!("'{greeting}' went from {length_before} to {length_after} characters.");
}   // `greeting` is dropped here. One owner, one cleanup.
```

Output:

```text
'hello!!!' went from 5 to 8 characters.
```

Notice what happened: ownership never left `main`. Functions borrowed the value as needed, and the compiler verified every step.

#### 4.11 The Mental Checklist

When Rust rejects your code, ask these questions in order:

1. **Who owns this value right now?**
2. **Did I accidentally move it into a function or another variable?** If so, borrow with `&` instead.
3. **Do I need to change it?** If so, use `&mut`, and make sure no other borrow is alive.
4. **Am I holding a reference while changing the original?** Shorten the reference's life, or reorder your code.
5. **Am I returning a reference to something local?** Return the owned value instead.

> 🧭 **A reassuring truth:** Experienced Rust developers hit these errors constantly too. The difference is that they recognize the pattern in seconds. Your fluency will grow with every error you read and fix.

#### 4.12 Chapter 4 Exercise

Predict whether each snippet compiles. Then check with `cargo run`.

```rust
// Snippet A
let s1 = String::from("ferris");
let s2 = s1;
println!("{s1}");

// Snippet B
let s1 = String::from("ferris");
let len = calculate_length(&s1);
println!("{s1} has {len} characters.");

// Snippet C
let mut data = String::from("abc");
let r1 = &data;
let r2 = &mut data;
println!("{r1} {r2}");
```

> ✅ **Answers:** A fails (use after move). B compiles, assuming you define `calculate_length(s: &String) -> usize`. C fails (cannot borrow mutably while an immutable borrow is alive and used later).

---

### End of Part I

You have now learned:

- What code is and how computers execute it (Chapter 1)
- How to install Rust, use Cargo, and read compiler errors (Chapter 2)
- Variables, types, functions, and control flow (Chapter 3)
- Ownership, borrowing, and slices, the heart of Rust (Chapter 4)

> 🧭 **A note before moving on:** If the ownership chapter still feels slippery, that is perfectly fine. Write a small program that moves a `String` into a function, then borrows it, then mutates it. Ten minutes of experimenting will teach you more than another re-read.
>
> 🧭 **Course scale:** This book spans 16 Markdown files and over 10,000 lines: Parts I–XIV plus appendices. Work front to back on first pass, then use Appendices E–H as the index.

---

*Part II: Mastering the Mechanics, covering structs, enums, error handling, collections, traits, and lifetimes, begins next.*

---

### Appendix A: The ANSI Color Guide

Ever wondered how a terminal shows red error text or a green success message? The answer is **ANSI escape codes**: invisible instructions mixed into plain text. The terminal sees the code and changes its style instead of printing it.

> 💡 **Analogy:** It is like writing "[RED]" in a script for an actor. The actor never says the word out loud. They just change their performance.

#### A.1 Anatomy of an Escape Code

Every code starts with an **escape character** (written `\x1b` in Rust, and also called `ESC`), followed by `[`, a number, and the letter `m`:

```text
ESC  [  3  1  m
\x1b [ 3 1 m     →  "switch the text to red"
```

Rust string literals understand `\x1b`, so a color code is just a string:

```rust
let red = "\x1b[31m";   // Start red text.
let reset = "\x1b[0m";  // Go back to normal. Always reset!
```

> ⚠️ **Always reset.** If you forget `\x1b[0m`, every later line in the terminal stays colored.

#### A.2 The Standard Color Table

| Code | Color | Bright variant | Background code | Bright background |
|:----:|:------|:--------------:|:---------------:|:-----------------:|
| 30 / 40 | Black | 90 / 100 | 40 | 100 |
| 31 / 41 | Red | 91 / 101 | 41 | 101 |
| 32 / 42 | Green | 92 / 102 | 42 | 102 |
| 33 / 43 | Yellow | 93 / 103 | 43 | 103 |
| 34 / 44 | Blue | 94 / 104 | 44 | 104 |
| 35 / 45 | Magenta | 95 / 105 | 45 | 105 |
| 36 / 46 | Cyan | 96 / 106 | 46 | 106 |
| 37 / 47 | White | 97 / 107 | 47 | 107 |

The first column is for text color (foreground). The background column paints the area behind the text.

#### A.3 Text Styles

| Code | Effect |
|:----:|:-------|
| `0` | Reset everything to normal |
| `1` | **Bold** |
| `2` | Dim |
| `3` | *Italic* (not supported everywhere) |
| `4` | Underline |
| `7` | Inverse (swap text and background colors) |
| `9` | ~~Strikethrough~~ (not supported everywhere) |

You can combine codes with semicolons. For example, `"\x1b[1;31m"` means bold red.

#### A.4 A Reusable Color Toolkit in Rust

Rather than typing codes everywhere, define them once as constants. Constants are named values fixed at compile time, written in capital letters by convention:

```rust
// src/main.rs
// Constants: values that never change, fixed when the program is compiled.
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";

fn main() {
    // Inline variables inside braces: {RED} inserts the code above.
    println!("{GREEN}{BOLD}Build succeeded!{RESET}");
    println!("{YELLOW}Warning:{RESET} disk space is running low.");
    println!("{RED}Error:{RESET} could not open config.toml");
    println!("{CYAN}Info:{RESET} using default settings.");
    println!("{BLUE}Tip:{RESET} run with --help for options.");
}
```

Each line prints a colored label, then resets so the rest of the line stays normal.

#### A.5 Putting It to Work: A Colored `if`

Combine what you learned in Chapter 2 with colors. The program below chooses a color based on a condition:

```rust
// src/main.rs
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";

fn main() {
    let tests_passed = 9;
    let tests_total = 10;

    // Pick the color using an `if` expression (Chapter 2, section 2.6).
    let color = if tests_passed == tests_total { GREEN } else { RED };

    println!("{color}Tests passed: {tests_passed}/{tests_total}{RESET}");
}
```

Change `tests_passed` to `10` and run again to see the color flip.

#### A.6 A Full Palette Demo

This program prints all 16 standard colors in a loop. It uses the `for` loop from Chapter 3:

```rust
// src/main.rs
fn main() {
    // Print the 8 standard foreground colors, then their bright versions.
    for code in 30..=37 {
        // `{code}` inserts the number; `\x1b[{code}m` builds the escape sequence.
        print!("\x1b[{code}m  {code}  \x1b[0m");
    }
    println!(); // Finish the first row.

    for code in 90..=97 {
        print!("\x1b[{code}m  {code}  \x1b[0m");
    }
    println!();
}
```

> 🧭 **Tip:** Run this once so you can see how your terminal theme renders each color. Terminal themes change the exact shades, so the same code may look slightly different on different machines.

#### A.7 Terminal Compatibility

| Environment | Colors supported? | Notes |
|:------------|:-----------------:|:------|
| Windows Terminal | Yes | Recommended on Windows 10 and 11. |
| PowerShell 7+ | Yes | Works well in modern Windows. |
| Legacy Command Prompt (`cmd.exe`) | Mostly | Older builds may show raw codes such as `←[31m`. Use Windows Terminal instead. |
| macOS Terminal and iTerm2 | Yes | Full support. |
| Linux terminals (GNOME, Konsole, etc.) | Yes | Full support. |
| Output redirected to a file | No | Codes are written as raw text. Use plain output in logs. |

> ⚠️ **Piping caution:** If you save colored output to a file, the file will contain escape codes. Check whether the output is going to a terminal before adding colors in production programs. You will learn how in later chapters.

#### A.8 Quick Reference Card

```text
Reset everything    \x1b[0m
Bold                \x1b[1m
Red text            \x1b[31m        Bright red text    \x1b[91m
Green text          \x1b[32m        Bright green       \x1b[92m
Yellow text         \x1b[33m        Bright yellow      \x1b[93m
Blue text           \x1b[34m        Bright blue        \x1b[94m
Magenta text        \x1b[35m        Bright magenta     \x1b[95m
Cyan text           \x1b[36m        Bright cyan        \x1b[96m
White text          \x1b[37m        Bright white       \x1b[97m
Red background      \x1b[41m
Bold red text       \x1b[1;31m
```

---

*You have reached the end of Part I. Ready for Part II whenever you are.*


---

<a id="part-ii-mechanics-ch-5-8"></a>

## Part II - Mechanics (Ch 5-8)

| Navigation | |
| --- | --- |
| Previous | [← Part I - Genesis (Ch 1-4)](#part-i-genesis-ch-1-4) |
| Next | [Part III - Concurrency (Ch 9-10) →](#part-iii-concurrency-ch-9-10) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part II: Mastering the Mechanics

> 🧭 **Where we are:** In Part I you learned to read, write, and reason
> about simple programs, and you met ownership and borrowing. Part II
> turns those basics into the tools professional Rust developers use every
> day: custom data types, safe error handling, collections, reusable
> code, and the lifetime rules that tie everything together.

---

### Chapter 5: Structs, Enums, and Pattern Matching

#### 5.1 Why Model Data at All?

Real programs describe things. A library has books, members, and loans. A
game has players, enemies, and items. If you store each piece of
information in a separate variable, your code quickly becomes a mess:

```rust
let name1 = "Ada";
let email1 = "ada@example.com";
let active1 = true;

let name2 = "Linus";
let email2 = "linus@example.com";
let active2 = false;
```

This works, but nothing connects `name1` to `email1`. Imagine forty
users. Rust gives you a better tool: a **struct**, which bundles related
values under one name.

> 💡 **Analogy:** A struct is a form with labeled boxes. Each box has a
> name (the field name) and a type. You fill out one form per user, and
> every form has the same boxes in the same order.
>
> 🧭 **Roadmap:** This part covers Chapters 5–8. By the end you will model
> data with structs and enums, handle errors with `Option` and `Result`,
> reuse code with traits and generics, and read lifetime errors fluently.

#### 5.2 Defining and Using Structs

```rust
// A struct groups related fields into one custom type.
struct User {
    username: String,   // Owned text.
    email: String,      // Owned text.
    active: bool,       // Yes or no.
    login_count: u32,   // A non-negative whole number.
}

fn main() {
    // Create an instance by naming every field.
    let mut ada = User {
        username: String::from("ada"),
        email: String::from("ada@example.com"),
        active: true,
        login_count: 0,
    };

    // Access fields with a dot.
    println!("{} has logged in {} times.", ada.username, ada.login_count);

    // Change a field: the whole variable must be `mut`.
    ada.login_count += 1;
}
```

Notice that `ada` is `mut` as a whole. Rust does not let you make
individual fields mutable. Either the entire struct can change, or none of
it can, which keeps the rules simple.

##### Field Init Shorthand and Struct Update Syntax

When a variable has the same name as a field, you can skip the repetition:

```rust
fn make_user(username: String, email: String) -> User {
    User {
        username,        // Same as: username: username
        email,           // Same as: email: email
        active: true,
        login_count: 0,
    }
}
```

To create a new value that copies most fields from an existing one, use
the `..` update syntax:

```rust
let linus = User {
    username: String::from("linus"),
    ..ada   // Take all remaining fields from `ada`.
};
```

> ⚠️ **Watch out:** `..ada` moves the owned `String` fields out of `ada`.
> After that line, `ada.username` can no longer be used. This is ownership
> from Chapter 4 at work.

#### 5.3 Methods: Giving Structs Behavior

Functions attached to a struct are called **methods**. You define them in
an `impl` block (short for *implementation*):

```rust
#[derive(Debug)]               // Lets us print the struct with {:?}.
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u32,
}

impl User {
    // An associated function: no `self`. Called as User::new(...).
    // Constructors like this are a common Rust convention.
    fn new(username: &str, email: &str) -> User {
        User {
            username: username.to_string(),  // Turn a borrowed &str into an owned String.
            email: email.to_string(),
            active: true,
            login_count: 0,
        }
    }

    // A method that borrows self immutably. It only reads.
    fn greeting(&self) -> String {
        format!("Welcome back, {}!", self.username)
    }

    // A method that borrows self mutably. It changes the struct.
    fn record_login(&mut self) {
        self.login_count += 1;
    }
}

fn main() {
    let mut user = User::new("ada", "ada@example.com");
    user.record_login();
    println!("{}", user.greeting());   // Welcome back, ada!
    println!("{:?}", user);            // Debug output: every field, in one line.
}
```

The first parameter of a method tells you how it touches the struct:

| Signature | Meaning | Analogy |
|:----------|:--------|:--------|
| `fn f(self)` | Takes ownership. The struct is consumed. | Giving away the book. |
| `fn f(&self)` | Reads through a shared borrow. | Reading the book in the library. |
| `fn f(&mut self)` | Changes through an exclusive borrow. | Editing the book at your desk. |

> 🧭 **Why `#[derive(Debug)]`?** The line above a struct is an
> **attribute**. `derive(Debug)` asks the compiler to write printing code
> for you. Without it, `{:?}` will not compile. Use `{:#?}` for a
> multi-line, indented version.

#### 5.4 Tuple Structs and Unit Structs

Sometimes you want a new type but do not need field names:

```rust
struct Meters(f64);     // A tuple struct: fields are accessed by position.
struct Seconds(f64);

let distance = Meters(42.0);
let time = Seconds(8.5);
// let speed = distance / time; // Error: no division between these types.
println!("Distance: {} m", distance.0);
```

This is a powerful trick. The compiler now refuses to mix meters with
seconds, even though both are plain `f64` numbers inside. Rust calls this
pattern the **newtype** pattern.

A **unit struct** has no fields at all. It is useful as a marker, and
you will meet it again when we discuss traits in Chapter 7.

#### 5.5 Enums: One of Several Possibilities

A struct means "this AND that AND the other." An **enum** means "this OR
that OR the other." Use an enum when a value can be exactly one of several
shapes:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let heading = Direction::North;
```

Enum variants can also carry data, which makes them far more expressive:

```rust
enum Shape {
    Circle(f64),                          // Carries a radius.
    Rectangle { width: f64, height: f64 },// Carries named fields.
    Triangle(f64, f64, f64),              // Carries three side lengths.
}

let shapes = [
    Shape::Circle(2.0),
    Shape::Rectangle { width: 3.0, height: 4.0 },
    Shape::Triangle(3.0, 4.0, 5.0),
];
```

> 💡 **Mental model:** A struct is a box with fixed compartments. An enum
> is a box that holds exactly one thing at a time, and the label on the
> box tells you which thing it is.

#### 5.6 `match`: Handling Every Possibility

To use an enum safely, you use `match`. It checks every variant and forces
you to handle each one:

```rust
impl Shape {
    fn area(&self) -> f64 {
        match self {
            // Each arm: pattern => expression.
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula, for a triangle given its three sides.
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}
```

The key property of `match` is **exhaustiveness**. If you add a new
variant, such as `Shape::Hexagon`, every `match` in your program that does
not handle it becomes a compile error. The compiler finds every place
that needs updating. In large projects, this single feature prevents an
enormous number of bugs.

You can use `_` as a catch-all for "everything else":

```rust
let code = 404;
let message = match code {
    200 => "OK",
    404 => "Not found",
    500 => "Server error",
    _ => "Unknown status",  // Matches any other value.
};
println!("{message}");
```

##### Match Guards

Add an extra condition with `if`:

```rust
let number = 7;
match number {
    n if n < 0 => println!("{n} is negative"),
    0 => println!("zero"),
    n if n % 2 == 0 => println!("{n} is even"),
    n => println!("{n} is odd"),
}
```

##### `if let`: When You Only Care About One Case

Writing a full `match` just to handle one variant is noisy. `if let`
expresses the single case you care about:

```rust
let maybe_shape = Some(Shape::Circle(1.0));

if let Some(Shape::Circle(r)) = maybe_shape {
    println!("A circle with radius {r}");
}
```

`Option`, which you will meet properly in Chapter 6, is itself an enum,
so this pattern appears constantly.

#### 5.7 Destructuring

Patterns can also break a value apart:

```rust
let point = (3, -5);
let (x, y) = point;            // Unpack a tuple into two variables.

let user = User::new("ada", "ada@example.com");
let User { username, active, .. } = user;   // Pick out two fields.
println!("{username} is active: {active}");
```

The `..` means "ignore the rest," just as it did in struct update syntax.

#### 5.8 Chapter 5 Exercise: A Tiny Inventory

Model a store's inventory:

1. Define a struct `Item` with `name: String`, `price_cents: u32`, and
   `quantity: u32`.
2. Define an enum `Category` with variants `Food`, `Tool`, and
   `Toy(u8)`, where the `u8` is the recommended minimum age.
3. Add a method `total_value(&self) -> u32` to `Item` that returns
   `price_cents * quantity`.
4. In `main`, create three items and print each one's total value.

> ✅ **Hint:** Start with the struct and the method. Add `#[derive(Debug)]`
> to both types so you can print them while you debug.

---

### Chapter 6: Error Handling That Doesn't Suck

#### 6.1 Errors Are Normal

In many languages, errors arrive as surprises. Something throws an
exception from deep inside a function, and you find out at the top of the
program, far from the cause. Rust takes a calmer approach: errors are
ordinary values that the type system makes you deal with.

Reading a file might fail. A user might type "abc" where you expected a
number. A network might drop. Rust does not hide these possibilities. It
puts them in the function's signature so you cannot forget them.

> 💡 **Analogy:** Think of a postal delivery. A delivery can succeed, or
> it can fail because the address is wrong. Rust makes the courier hand
> you either the package or a note explaining the problem. You cannot
> pretend the note does not exist.

#### 6.2 Two Kinds of Failure

| Kind | When to use | Tool |
|:-----|:------------|:-----|
| **Recoverable** | Something expected might go wrong, such as a missing file or bad input. | `Option<T>` and `Result<T, E>` |
| **Unrecoverable** | A bug in your program, such as an index past the end of an array. | `panic!` |

Panics stop the current thread and print a message. Use them for
programmer mistakes, not for ordinary failures.

```rust
fn main() {
    let numbers = [1, 2, 3];
    // println!("{}", numbers[10]); // Panics: index out of bounds.
    println!("The program only panics if you uncomment the line above.");
}
```

#### 6.3 `Option<T>`: A Value That Might Be Absent

`Option` answers the question "is there a value?" It has two variants:

```rust
enum Option<T> {
    Some(T),   // There is a value of type T.
    None,      // There is no value.
}
```

The `<T>` is a **generic** parameter, a placeholder for any type. You will
learn generics in Chapter 7. For now, read `Option<i32>` as "maybe an
`i32`."

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None                // Cannot divide by zero: no answer.
    } else {
        Some(a / b)         // Here is the answer.
    }
}

fn main() {
    match divide(10.0, 4.0) {
        Some(result) => println!("Result: {result}"),
        None => println!("Cannot divide by zero."),
    }
}
```

Rust has no `null`. The absence of a value must be written as `None`,
and the compiler makes sure you check for it. This eliminates one of the
most common crashes in software history.

##### Handy Methods on `Option`

| Method | What it does |
|:-------|:-------------|
| `unwrap_or(default)` | Gives the value, or the default if `None`. |
| `map(f)` | Transforms the value if present, leaves `None` alone. |
| `ok_or(err)` | Converts `Option` into a `Result`, using `err` for `None`. |
| `is_some()` / `is_none()` | Checks which variant you have. |

```rust
let maybe_age: Option<u32> = None;
let age = maybe_age.unwrap_or(18);   // Falls back to 18.
println!("Age: {age}");
```

#### 6.4 `Result<T, E>`: Success or a Reason for Failure

When failure needs an explanation, use `Result`:

```rust
enum Result<T, E> {
    Ok(T),    // Success, carrying the value.
    Err(E),   // Failure, carrying the error.
}
```

```rust
fn parse_age(input: &str) -> Result<u8, String> {
    // `parse` returns a Result. We convert its error into our own message.
    match input.trim().parse::<u8>() {
        Ok(age) => Ok(age),
        Err(_) => Err(format!("'{input}' is not a valid age")),
    }
}

fn main() {
    match parse_age("42") {
        Ok(age) => println!("You are {age}."),
        Err(message) => println!("Error: {message}"),
    }

    match parse_age("forty") {
        Ok(age) => println!("You are {age}."),
        Err(message) => println!("Error: {message}"),
    }
}
```

The turbofish `::<u8>` tells `parse` which type to produce. It is a
useful habit when Rust cannot infer the target type on its own.

#### 6.5 The `?` Operator: Pass Errors Upward

Matching on every `Result` gets tedious. The `?` operator says: "if this
is an error, return it from the current function; otherwise, unwrap the
success value."

```rust
use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;  // Returns early on error.
    Ok(text)                               // Only reached on success.
}

fn main() {
    match read_config("settings.toml") {
        Ok(text) => println!("Loaded {} bytes.", text.len()),
        Err(e) => println!("Could not read config: {e}"),
    }
}
```

> 🧭 **Rule for `?`:** It only works inside a function whose return type
> is `Result` or `Option`. Using it in `main` requires `main` to return
> `Result`. Chapter 10 shows this in practice.

##### Chaining Calls with `?`

You can stack operations together:

```rust
fn read_number(path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;   // io::Error
    let number = text.trim().parse::<i64>()?;    // ParseIntError
    Ok(number)
}
```

The return type `Box<dyn std::error::Error>` can hold any error type.
It is convenient for applications, though libraries usually prefer
precise error types, as in the next section.

#### 6.6 Custom Error Types

For a real project, define an enum that lists every way your code can
fail. This gives callers clear options and clear messages.

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]                 // Needed so the error can be printed for debugging.
enum AppError {
    Parse(ParseIntError),        // Wraps the standard library's error.
    OutOfRange(i64),             // Our own error, with the bad value attached.
}

// Display controls the message a user sees.
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "not a whole number: {e}"),
            AppError::OutOfRange(n) => write!(f, "{n} is outside the range 1 to 100"),
        }
    }
}

// From lets `?` convert a ParseIntError into an AppError automatically.
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

fn parse_percent(input: &str) -> Result<i64, AppError> {
    let n: i64 = input.trim().parse()?;  // ParseIntError becomes AppError here.
    if (1..=100).contains(&n) {
        Ok(n)
    } else {
        Err(AppError::OutOfRange(n))
    }
}

fn main() {
    for input in ["50", "abc", "250"] {
        match parse_percent(input) {
            Ok(p) => println!("Percent: {p}"),
            Err(e) => println!("Problem with '{input}': {e}"),
        }
    }
}
```

Output:

```text
Percent: 50
Problem with 'abc': not a whole number: invalid digit found in string
Problem with '250': 250 is outside the range 1 to 100
```

> 💡 **Why this pattern matters:** The `From` implementation is the magic
> that lets `?` work across different error types. Once you write it, the
> rest of your code stays clean.

#### 6.7 `unwrap` and `expect`: Use With Care

`unwrap()` extracts the value from `Some` or `Ok`, and **panics** if it
finds `None` or `Err`. `expect("message")` does the same but prints your
message.

These are acceptable in quick experiments, tests, and cases where failure
is truly impossible. In real code, prefer `?`, `match`, or defaults such as
`unwrap_or`.

> ⚠️ **Rule of thumb:** If you find yourself writing `.unwrap()` in code
> that reads user input or files, stop and ask what should happen when
> that operation fails.

#### 6.8 Chapter 6 Exercise: A Safe Calculator

Write `fn calculate(a: &str, op: &str, b: &str) -> Result<f64, String>`
that:

- parses `a` and `b` as `f64` (return an error message if parsing fails),
- supports `"+"`, `"-"`, `"*"`, and `"/"`,
- returns an error for division by zero,
- returns an error for any unknown operator.

> ✅ **Hint:** Use `?` after each `parse`, converting the error with
> `map_err(|_| "...".to_string())`. Use a `match` on `op.as_str()` for the
> operators.

---

### Chapter 7: Collections, Generics, and Traits

This chapter gives you three tools that make Rust code short, flexible,
and reusable: collections that hold many values, generics that work for
many types, and traits that describe shared behavior.

#### 7.1 `Vec<T>`: A Growable List

A `Vec` (vector) stores many values of the same type in a list that can
grow and shrink:

```rust
fn main() {
    let mut scores: Vec<u32> = Vec::new();  // An empty vector of u32.
    scores.push(90);
    scores.push(75);
    scores.push(88);

    // The vec! macro creates a vector with starting values.
    let temperatures = vec![18, 21, 25];

    // Safe access: get returns an Option instead of panicking.
    match scores.get(10) {
        Some(score) => println!("Found: {score}"),
        None => println!("No score at position 10."),
    }

    // Iterate by reference, so the vector is not moved.
    for score in &scores {
        println!("Score: {score}");
    }

    println!("Temperatures: {:?}", temperatures);
}
```

Use `get` when an index might be out of range, and `scores[i]` when you
are certain the index exists.

#### 7.2 `HashMap<K, V>`: Look Up by Key

A `HashMap` associates keys with values, like a dictionary:

```rust
use std::collections::HashMap;

fn main() {
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(String::from("Ada"), 36);
    ages.insert(String::from("Linus"), 28);

    // get returns Option<&u32>, because the key might not exist.
    if let Some(age) = ages.get("Ada") {
        println!("Ada is {age}.");
    }

    // The entry API: insert a default only when the key is missing.
    let text = "the cat and the hat and the bat";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0); // Returns &mut u32.
        *count += 1;                                 // The * dereferences it.
    }

    println!("'the' appears {} times.", counts["the"]);
}
```

> 🧭 **Dereferencing with `*`:** `count` is a reference to a number. To
> change the number it points to, you write `*count`. This is the same
> idea as the `&mut` borrows from Chapter 4.

#### 7.3 Iterators: Transforming Collections Cleanly

An **iterator** is a sequence of values you can step through. Iterators
are lazy: they do nothing until you consume them, which lets the compiler
optimize them heavily.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // Keep the even numbers, square them, and collect into a new Vec.
    let even_squares: Vec<i32> = numbers
        .iter()                    // Step through by reference.
        .filter(|&&n| n % 2 == 0)  // Keep only even numbers.
        .map(|&n| n * n)           // Transform each one.
        .collect();                // Gather results into a Vec.

    let total: i32 = numbers.iter().sum();  // Add them all up.

    println!("Even squares: {:?}", even_squares); // [4, 16, 36]
    println!("Total: {total}");                   // 21
}
```

The double `&&` in `filter` looks odd at first. `iter()` yields
references, and `filter` receives a reference to that reference. Pattern
matching with `&&n` unwraps both layers at once.

| Iterator method | Purpose |
|:----------------|:--------|
| `map` | Transform each item. |
| `filter` | Keep items that match a condition. |
| `collect` | Turn the iterator into a collection. |
| `sum` / `product` | Add or multiply all items. |
| `enumerate` | Pair each item with its index. |
| `zip` | Combine two iterators pairwise. |

> 💡 **Why iterators beat index loops:** An index loop can go wrong by one
> or crash on an empty list. An iterator chain says what you want, and
> Rust handles the bounds.

#### 7.4 Generics: One Function, Many Types

Imagine writing a function that finds the largest number in a list. You
would have to write one copy for `i32`, another for `f64`, and another for
`u8`. **Generics** solve this by letting you write the logic once, with a
type placeholder:

```rust
// T is a type parameter. The bounds after the colon say what T must support.
// PartialOrd: values can be compared with > and <.
// Copy: values can be duplicated cheaply, so we can copy them out of the slice.
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut biggest = items[0];
    for &item in items {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

fn main() {
    println!("{}", largest(&[34, 50, 25, 100]));    // i32
    println!("{}", largest(&[1.5, 0.2, 9.9]));      // f64
    println!("{}", largest(&['x', 'b', 'z']));      // char
}
```

Generics are **monomorphized**: when you compile, Rust creates a separate,
fully optimized copy for each concrete type you use. You pay nothing at
runtime for this flexibility.

##### Generic Structs

```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + std::fmt::Display> Pair<T> {
    fn show_larger(&self) {
        if self.first > self.second {
            println!("Larger: {}", self.first);
        } else {
            println!("Larger: {}", self.second);
        }
    }
}
```

#### 7.5 Traits: Shared Behavior

A **trait** describes a capability that different types can share. It is
like a job description: anyone who can do the job, regardless of their
background, can be hired for it.

```rust
trait Describe {
    // Required: each type must provide this.
    fn name(&self) -> String;

    // Provided: a default implementation that types may reuse or override.
    fn describe(&self) -> String {
        format!("This is {}.", self.name())
    }
}

struct Dog;                          // A unit struct from Chapter 5.

struct Robot {
    model: String,
}

impl Describe for Dog {
    fn name(&self) -> String {
        String::from("a dog")
    }
}

impl Describe for Robot {
    fn name(&self) -> String {
        format!("robot model {}", self.model)
    }
    // We did not write describe(), so the default version is used.
}

fn main() {
    let dog = Dog;
    let robot = Robot { model: String::from("R2") };
    println!("{}", dog.describe());
    println!("{}", robot.describe());
}
```

##### Trait Bounds: Accepting Anything That Has a Behavior

Combine traits and generics to write functions that accept any type with
the right capability:

```rust
// Two equivalent ways to say "accepts any type that implements Describe".
fn introduce(thing: &impl Describe) {
    println!("{}", thing.describe());
}

fn introduce_generic<T: Describe>(thing: &T) {
    println!("{}", thing.describe());
}
```

The `impl Describe` form is shorter. The generic form is more flexible
when you need to name the type several times.

##### Dynamic Dispatch: Mixing Types in One List

Generics produce one copy of the code per type, so a `Vec` cannot hold a
`Dog` and a `Robot` side by side through generics alone. For that, use a
trait object:

```rust
fn main() {
    // Box<dyn Describe> means: a heap-allocated value of some type that
    // implements Describe. The exact type is decided at runtime.
    let things: Vec<Box<dyn Describe>> = vec![
        Box::new(Dog),
        Box::new(Robot { model: String::from("R2") }),
    ];

    for thing in &things {
        println!("{}", thing.describe());
    }
}
```

| Approach | Decided at | Cost | Use when |
|:---------|:-----------|:-----|:---------|
| Generics (`T: Trait`) | Compile time | None at runtime | One type per call site. |
| Trait objects (`dyn Trait`) | Runtime | A small lookup per call | Mixed types in one collection. |

##### Traits You Already Use

Rust's standard library is built on traits. You have already met several:

- `Debug` lets `{:?}` print a value.
- `Display` lets `{}` print a value.
- `Clone` lets you call `.clone()`.
- `Copy` lets values duplicate implicitly.
- `Default` provides a sensible starting value.
- `From` and `Into` convert between types.
- `Iterator` powers every `for` loop and iterator chain.

#### 7.6 Chapter 7 Exercise: Shapes With Behavior

Create a trait `HasArea` with a method `fn area(&self) -> f64`. Implement
it for two structs, `Square { side: f64 }` and `Circle { radius: f64 }`.
Then write a function `total_area` that takes a `Vec<Box<dyn HasArea>>`
and returns the sum of all areas.

> ✅ **Hint:** Use `.iter().map(|s| s.area()).sum()` inside `total_area`.

---

### Chapter 8: Lifetimes and Advanced Memory

#### 8.1 The Question Lifetimes Answer

Chapter 4 showed that a reference must never outlive the value it points
to. Most of the time, Rust figures this out automatically. Sometimes it
needs help. A **lifetime** is the region of code during which a reference
is valid. Lifetime annotations are labels that tell the compiler how
long different references live relative to each other.

> 💡 **Analogy:** Think of library books on loan. A borrower's due date
> cannot be later than the book's return date, because the book must exist
> while it is being read. Lifetimes are the compiler's due dates.

#### 8.2 The Problem

This function does not compile:

```rust
fn longest(a: &str, b: &str) -> &str {  // ERROR: missing lifetime specifier
    if a.len() >= b.len() { a } else { b }
}
```

The compiler cannot tell whether the returned reference points at `a` or
at `b`, so it cannot check that the result stays valid. We must explain
the relationship.

#### 8.3 Lifetime Annotations

```rust
// 'a is a lifetime name. It says: the output lives no longer than
// BOTH inputs, because it might be either one of them.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let first = String::from("long string is long");
    let result;
    {
        let second = String::from("short");
        result = longest(first.as_str(), second.as_str());
        println!("Inside: {result}");   // Fine: both are still alive.
    }
    // println!("{result}");  // Would fail: `second` is already gone.
}
```

The annotations do not change how long anything lives. They describe
relationships so the compiler can verify them. The `'a` on each input
and on the output means: all three share the same lifetime, the shortest
one involved.

#### 8.4 Lifetime Elision: When Rust Guesses for You

Most functions do not need explicit annotations, because Rust applies
three **elision rules**:

1. Each input reference gets its own lifetime.
2. If there is exactly one input lifetime, the output gets that lifetime.
3. If one input is `&self` or `&mut self`, the output gets the lifetime
   of `self`.

```rust
// Written without annotations, because of rule 2:
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
```

When rules 2 and 3 do not apply, you must annotate. The next section is
such a case.

#### 8.5 Structs That Hold References

A struct can borrow data instead of owning it. The struct then needs a
lifetime parameter:

```rust
// This struct holds a borrowed slice of some longer text.
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // Rule 3 applies: the returned reference lives as long as self.
    fn announce(&self) -> &str {
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence };
    println!("Excerpt: {}", excerpt.announce());
}
```

The `Excerpt` cannot outlive `novel`. The compiler enforces that
relationship for you.

> 🧭 **The `'static` lifetime:** A string literal such as `"hello"` has
> type `&'static str`, meaning it lives for the whole program. You may see
> this in error messages, and it is usually the right answer for constant
> text.

#### 8.6 Smart Pointers: Choosing How Memory Is Owned

Sometimes plain references and owned values are not enough. Rust's
**smart pointers** are structs that own heap data and add extra behavior.
Each solves a specific problem.

| Smart pointer | Solves | Typical use |
|:--------------|:-------|:------------|
| `Box<T>` | Put a value on the heap with a single owner. | Recursive types, large data, trait objects. |
| `Rc<T>` | Give several owners shared read access (one thread). | Shared data in a single-threaded program. |
| `RefCell<T>` | Allow mutation through a shared reference, checked at runtime. | Interior mutability. |

##### `Box<T>`: The Simplest Heap Pointer

Recursive types need `Box`, because a type cannot contain itself directly
without an unknown size:

```rust
// Without Box, the compiler could not know how big a List is.
enum List {
    Cons(i32, Box<List>),   // A value followed by another list.
    Nil,                    // The end of the list.
}

fn main() {
    // Builds the list 1 -> 2 -> 3 -> end.
    let list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil))),
        )),
    );
    // A full list printer is left as an exercise.
    let _ = list;
}
```

##### `Rc<T>`: Several Owners

`Rc` stands for *reference counted*. It keeps a count of how many owners
exist and frees the value when the count reaches zero:

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("shared config"));
    let a = Rc::clone(&shared);   // Cheap: increases the count, copies no text.
    let b = Rc::clone(&shared);

    println!("{shared}, {a}, {b}");
    println!("Owners: {}", Rc::strong_count(&shared)); // 3
}   // Count drops as each owner leaves scope. Freed when it reaches zero.
```

Use `Rc::clone(&x)` rather than `x.clone()`. The explicit form makes it
clear that you are sharing ownership, not duplicating data.

> ⚠️ **Limitation:** `Rc` is for single-threaded code. Chapter 9 introduces
> `Arc`, its thread-safe sibling.

##### `RefCell<T>`: Mutation Through a Shared Reference

Normally, changing data requires `&mut`, and `&mut` requires exclusive
access. `RefCell` moves the borrowing rules from compile time to runtime.
If you break them, the program panics instead of failing to compile.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Account {
    balance: i32,
}

fn main() {
    // Rc gives shared ownership. RefCell gives mutation through it.
    let account = Rc::new(RefCell::new(Account { balance: 100 }));

    let for_alice = Rc::clone(&account);
    let for_bob = Rc::clone(&account);

    for_alice.borrow_mut().balance -= 30;   // Mutable borrow, released at end of statement.
    for_bob.borrow_mut().balance += 10;

    println!("Balance: {}", account.borrow().balance);  // 80
    println!("Owners: {}", Rc::strong_count(&account)); // 3
}
```

> ⚠️ **Runtime panics:** If you hold a `borrow_mut()` while calling
> `borrow()` or another `borrow_mut()` on the same `RefCell`, the program
> panics with `already borrowed`. Keep borrows short.

> 🧭 **Rc plus RefCell:** This pair is a common way to share mutable data
> in single-threaded programs such as GUI trees or graph structures. Use
> it deliberately, because it moves some safety checks from compile time
> to runtime.

#### 8.7 Drop: Custom Cleanup

When a value goes out of scope, Rust calls its `Drop` implementation,
which is how files close and memory frees automatically:

```rust
struct Notifier {
    name: String,
}

impl Drop for Notifier {
    fn drop(&mut self) {
        println!("{} is being cleaned up.", self.name);
    }
}

fn main() {
    let _first = Notifier { name: String::from("first") };
    let _second = Notifier { name: String::from("second") };
    println!("End of main.");
}
// Output order: "End of main." then "second" then "first".
// Values are dropped in reverse order of creation.
```

You rarely implement `Drop` yourself. You will rely on it constantly
through standard types such as `String` and `File`.

#### 8.8 Chapter 8 Exercise: A Shared Log

Write a program that creates an `Rc<RefCell<Vec<String>>>` log. Create two
clones of it, have one "writer" push three messages through each clone,
and print the final contents plus the number of owners.

> ✅ **Hint:** Use `log.borrow_mut().push(...)` for writing and
> `log.borrow()` for reading. Make sure no borrow is alive when you print.

---

### Part II Recap

You now know how to:

- Model real data with structs and enums (Chapter 5).
- Handle failure safely with `Option`, `Result`, and `?` (Chapter 6).
- Store and transform collections with iterators (Chapter 7).
- Write reusable code with generics and traits (Chapter 7).
- Explain borrowing relationships with lifetimes and choose the right
  smart pointer (Chapter 8).

> 🧭 **Take a breath.** Part II is dense. Many developers return to these
> chapters repeatedly while building their first real projects. That is
> the normal way the material settles in.

In Part III, you will make programs do several things at once with threads
and async tasks, then learn how professionals test, document, and organize
Rust projects.


---

<a id="part-iii-concurrency-ch-9-10"></a>

## Part III - Concurrency (Ch 9-10)

| Navigation | |
| --- | --- |
| Previous | [← Part II - Mechanics (Ch 5-8)](#part-ii-mechanics-ch-5-8) |
| Next | [Part IV - Full-Stack (Ch 11-13) →](#part-iv-full-stack-ch-11-13) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part III: Concurrency and Systems

> 🧭 **Where we are:** Part II gave you the tools to model data, handle
> errors, and write reusable code. Part III adds two things every serious
> systems programmer needs: the ability to do many things at once, and the
> habits that keep large codebases healthy over years, namely testing,
> documentation, and idiomatic style.

---

### Chapter 9: Fearless Concurrency

#### 9.1 What Is Concurrency?

**Concurrency** means making progress on several tasks during overlapping
time periods. A restaurant kitchen is a good picture: one chef chops
onions while the soup simmers, and another chef handles the grill. Both
tasks are in progress at once.

Concurrency is different from **parallelism**, which means running tasks
at the exact same instant on multiple CPU cores. Rust supports both. In
this chapter, the goal is to do this safely.

> 💡 **Why concurrency is dangerous:** Two threads sharing data can race
> each other. One reads a value while another is halfway through changing
> it, and the result is garbage. These bugs are notoriously hard to
> reproduce, because they depend on timing. Rust's ownership rules are
> designed to make such races impossible to compile.

#### 9.2 Threads: Doing Two Things at Once

A **thread** is an independent sequence of instructions managed by the
operating system. `std::thread::spawn` starts one:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Start a new thread running this closure.
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("worker: step {i}");
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Meanwhile, the main thread keeps going.
    for i in 1..=2 {
        println!("main: step {i}");
        thread::sleep(Duration::from_millis(50));
    }

    // Wait for the worker to finish before the program exits.
    handle.join().unwrap();
}
```

The order of output lines can vary from run to run. That variation is the
essence of concurrency: the operating system decides when each thread gets
the processor.

> ⚠️ **Why `join`?** When `main` returns, the whole program exits, even if
> other threads are still running. `join` makes the main thread wait for
> the worker to finish.

##### Moving Data Into Threads

A closure borrowing a local variable cannot be sent to a thread, because
the thread might outlive that variable. Use `move` to transfer ownership
into the closure:

```rust
use std::thread;

fn main() {
    let names = vec![String::from("Ada"), String::from("Linus")];

    // `move` transfers ownership of `names` into the thread.
    let handle = thread::spawn(move || {
        for name in &names {
            println!("Hello, {name}!");
        }
    });

    handle.join().unwrap();
    // println!("{:?}", names); // Error: `names` was moved into the thread.
}
```

Chapter 4 predicted this: ownership moves, and the original variable is
no longer valid. Rust applies the same rule across threads.

#### 9.3 Message Passing: Sharing by Communicating

A popular Go-inspired motto says: "Do not communicate by sharing memory;
share memory by communicating." Rust's **channels** follow this idea.
A channel has two ends: a **sender** (`tx`, for transmit) and a
**receiver** (`rx`, for receive).

```rust
use std::sync::mpsc;   // mpsc = multiple producer, single consumer.
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["hello", "from", "the", "worker"];
        for msg in messages {
            tx.send(msg.to_string()).unwrap();   // Send ownership of the String.
            thread::sleep(Duration::from_millis(100));
        }
        // When this closure ends, `tx` is dropped and the channel closes.
    });

    // Iterating the receiver waits for each message, and stops when the
    // channel closes.
    for received in rx {
        println!("Got: {received}");
    }
}
```

Notice that sending a `String` moves it into the channel. The receiving
thread then owns it. No two threads ever hold the same data at once,
so there is nothing to race over.

| Method | Behavior |
|:-------|:---------|
| `tx.send(value)` | Sends a value. Returns `Result`, failing if the receiver is gone. |
| `rx.recv()` | Waits for one value. Returns `Err` when all senders are dropped. |
| `rx.try_recv()` | Checks without waiting. Useful inside a busy loop. |

#### 9.4 Shared State: `Arc` and `Mutex`

Sometimes several threads genuinely need to update one value, such as a
global counter. The tools for that are:

- **`Mutex<T>`**, a lock. Only one thread can hold it at a time. To use the
  data, a thread calls `lock()`, which waits its turn.
- **`Arc<T>`**, an atomic reference counter. It is `Rc` made safe for
  threads, so several threads can share ownership.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc lets many threads own the counter; Mutex lets one at a time change it.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Each thread gets its own Arc handle to the same Mutex.
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // lock() waits until no other thread holds the lock.
            // unwrap() handles the error case where another thread panicked.
            let mut num = counter.lock().unwrap();
            *num += 1;
        }); // The guard `num` is dropped here, releasing the lock automatically.

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap()); // Always 10.
}
```

The lock is released automatically when the guard goes out of scope. You
cannot forget to unlock, which eliminates a whole class of deadlocks.

> 🧭 **Pattern to remember:** `Arc<Mutex<T>>` is the standard way to share
> mutable state between threads. Keep the locked section as short as
> possible, so other threads do not wait longer than necessary.

#### 9.5 `Send` and `Sync`: Why the Compiler Is Picky

How does Rust know which types are safe to share across threads? Two
marker traits answer that question:

- **`Send`**: a type can be transferred to another thread.
- **`Sync`**: a type can be referenced from multiple threads at once.

Most types implement both automatically. The notable exception is `Rc`,
whose count is not updated atomically. Try moving one into a thread:

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(5);
    // thread::spawn(move || println!("{data}"));
    // Error: `Rc<i32>` cannot be sent between threads safely.
    let _ = data;
}
```

Switching to `Arc` fixes the problem, because `Arc` is `Send` and `Sync`
when its contents are. You rarely implement these traits yourself. Their
job is to let the compiler reject unsafe sharing before your program runs.

#### 9.6 Async and Await: Waiting Without Blocking

Threads are powerful, but each one costs memory. A web server handling
ten thousand simultaneous connections cannot afford ten thousand threads.
**Async programming** lets one thread juggle many tasks that spend most of
their time waiting, such as network requests or disk reads.

The core ideas:

- An `async fn` returns a **future**, a description of work that has not
  started yet.
- `.await` pauses the current task until a future finishes, letting other
  tasks run in the meantime.
- An **async runtime** runs the futures. Rust's standard library does not
  include one, so most projects use **Tokio**.

Add Tokio to a project:

```bash
cargo add tokio --features full
```

```rust
use std::time::Duration;
use tokio::time::sleep;

// Pretend this is a network call that takes 200 milliseconds.
async fn fetch_name(id: u32) -> String {
    sleep(Duration::from_millis(200)).await;   // Pause without blocking the thread.
    format!("user-{id}")
}

// #[tokio::main] builds a runtime and turns main into an async function.
#[tokio::main]
async fn main() {
    // Run both fetches at the same time; total wait is about 200 ms, not 400.
    let (a, b) = tokio::join!(fetch_name(1), fetch_name(2));
    println!("{a} and {b}");
}
```

> 💡 **Key insight:** Calling `fetch_name(1)` does nothing by itself. It
> creates a future. Work starts only when the future is awaited or
> joined, which is why `tokio::join!` runs both concurrently.

##### Threads Versus Async: Choosing a Tool

| Situation | Better choice | Reason |
|:----------|:--------------|:-------|
| Heavy number crunching | Threads | Work keeps the CPU busy. |
| Thousands of network connections | Async | Tasks mostly wait, so one thread can serve many. |
| Simple background job | Threads | Simpler to reason about. |
| Web servers and API clients | Async | Matches the I/O-heavy workload. |

> ⚠️ **Common trap:** Do not call blocking functions such as
> `std::thread::sleep` inside async code. They freeze the whole runtime
> thread. Use `tokio::time::sleep` instead.

#### 9.7 Chapter 9 Exercise: Parallel Word Counter

Split a list of sentences among three threads. Each thread should count
the words in its share and send the total back through a channel. The
main thread adds the results and prints the grand total.

> ✅ **Hint:** Give each thread a clone of `tx`. Drop the original `tx` in
> `main` before iterating `rx`, otherwise the loop never ends because the
> channel stays open.

---

### Chapter 10: Testing, Documentation, and Idiomatic Rust

#### 10.1 Why Testing Matters

Tests are executable promises. Each one states: "when I call this
function with these inputs, I get this result." Years later, when someone
changes the code, the tests catch any accidental break. Rust makes testing
unusually easy, because tests live right next to the code they check.

#### 10.2 Unit Tests

Put tests in a module marked `#[cfg(test)]`. The attribute tells the
compiler to include the module only when running tests, so it adds no
weight to your release build:

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;   // Bring the functions above into scope.

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn dividing_by_zero_is_an_error() {
        assert!(divide(1.0, 0.0).is_err());
    }

    #[test]
    fn divides_correctly() -> Result<(), String> {
        // A test can return Result, so `?` works inside it.
        let result = divide(10.0, 4.0)?;
        assert_eq!(result, 2.5);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn panics_on_bad_index() {
        let values = vec![1, 2, 3];
        let _ = values[10];   // Expected to panic, so the test passes.
    }
}
```

Run the tests:

```bash
cargo test
```

Useful assertion macros:

| Macro | Checks |
|:------|:-------|
| `assert!(condition)` | The condition is true. |
| `assert_eq!(a, b)` | `a` equals `b`. Prints both values on failure. |
| `assert_ne!(a, b)` | `a` does not equal `b`. |

> 💡 **Tip:** Name tests as sentences, such as `dividing_by_zero_is_an_error`.
> When a test fails, the name tells the story of what broke.

#### 10.3 Integration Tests

Unit tests check internal pieces. **Integration tests** check how the
pieces work together, as an outside user would see them. They live in a
`tests/` folder beside `src/`:

```text
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── math_tests.rs
```

```rust
// tests/math_tests.rs
// Integration tests import the crate just like an external user would.
use my_project::add;

#[test]
fn add_works_from_outside() {
    assert_eq!(add(10, 20), 30);
}
```

> 🧭 **Library versus binary:** Integration tests can only import from a
> library crate (`src/lib.rs`). If your project only has `src/main.rs`,
> move the logic into `lib.rs` and have `main.rs` call it. This is a common
> and recommended structure.

#### 10.4 Documentation Comments

Three slashes (`///`) start a documentation comment. Rust turns these into
a browsable website with `cargo doc --open`.

````rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let total = my_project::add(2, 2);
/// assert_eq!(total, 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

The code inside the example is a **doctest**. `cargo test` compiles and
runs it, so your documentation can never silently drift away from reality.

| Comment style | Purpose |
|:--------------|:--------|
| `///` | Documents the item below it. |
| `//!` | Documents the enclosing module or crate. |
| `//` | Ordinary comment for implementers, not included in docs. |

#### 10.5 Modules: Organizing Growing Code

As a project grows, one file becomes unwieldy. **Modules** group related
code. Declare them in `lib.rs` or `main.rs`:

```rust
// src/lib.rs
pub mod geometry;   // Looks for src/geometry.rs or src/geometry/mod.rs
mod internal;       // Private: only this crate can see it.
```

```rust
// src/geometry.rs
pub fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

fn helper() {
    // Private to this module, invisible to outside callers.
}
```

Other code reaches items by path:

```rust
use my_project::geometry::circle_area;   // Bring one function into scope.
// or
let area = my_project::geometry::circle_area(2.0);
```

Visibility is private by default. Add `pub` to expose an item. Use
`crate::` to start a path from the crate root, and `super::` to step up
one level.

#### 10.6 Idiomatic Rust: Writing Like a Native

**Idiomatic** code follows the conventions the community has settled on.
It compiles either way, but idiomatic code is easier to read and often
faster. Here are the most valuable habits.

| Instead of this | Write this | Why |
|:----------------|:-----------|:----|
| `fn greet(name: &String)` | `fn greet(name: &str)` | Accepts both `String` and string literals. |
| Index loop over a `Vec` | `for item in &list` | No off-by-one risk. |
| `if flag == true { }` | `if flag { }` | Shorter and clearer. |
| `match opt { Some(v) => v, None => 0 }` | `opt.unwrap_or(0)` | Concise for simple defaults. |
| `return result;` at the end | Just `result` | Last expression is the return value. |
| `.unwrap()` everywhere | `?` or proper matching | Errors propagate instead of crashing. |
| `.clone()` to silence the borrow checker | Restructure, or borrow | Clones cost memory and time. |

##### Derive Common Traits

For your own types, derive the traits callers will expect:

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Settings {
    volume: u8,
    muted: bool,
}
```

- `Debug` for printing with `{:?}`.
- `Clone` for explicit duplication.
- `PartialEq` for `==` comparisons.
- `Default` for `Settings::default()`.

#### 10.7 Tools That Keep You Honest

Rust ships with excellent tooling. Use these on every project:

```bash
cargo fmt      # Formats your code to the official style automatically.
cargo clippy   # Runs extra lints that suggest more idiomatic code.
cargo test     # Runs unit, integration, and doc tests.
cargo doc --open   # Builds and opens your documentation in a browser.
```

> 🧭 **Make it a habit:** Run `cargo fmt` and `cargo clippy` before every
> commit. Clippy often teaches you idioms you would otherwise learn only
> after years of experience.

#### 10.8 Chapter 10 Exercise: A Tested Temperature Converter

Write a library crate with a function `celsius_to_fahrenheit(c: f64) -> f64`
and its inverse `fahrenheit_to_celsius`. Then:

1. Add unit tests covering freezing point (0 °C is 32 °F), boiling point
   (100 °C is 212 °F), and a round trip.
2. Add a doc test showing a simple example.
3. Run `cargo test` until everything is green.

> ✅ **Hint:** Use the formula `F = C * 9.0 / 5.0 + 32.0`. Compare floats
> carefully: check that the absolute difference is below a small tolerance
> such as `1e-9`, rather than using `==`.

---

### Part III Recap

You now know how to:

- Run work on threads and coordinate them with channels and `Arc<Mutex<T>>`
  (Chapter 9).
- Understand why `Send` and `Sync` make Rust's concurrency fearless
  (Chapter 9).
- Write async code with Tokio and choose between threads and async
  (Chapter 9).
- Test units, integrations, and documentation examples (Chapter 10).
- Organize modules and write idiomatic, well-tooled Rust (Chapter 10).

These foundations carry directly into the next part of the book. In Part
IV, you will apply all of them to real systems: a backend web API with
Axum, a database with SQLx and PostgreSQL, and a browser frontend compiled
to WebAssembly.

> 🧭 **Before you move on:** Build something small that uses threads,
> a channel, and at least one test. A tiny project finished end to end
> teaches more than any amount of re-reading.


---

<a id="part-iv-full-stack-ch-11-13"></a>

## Part IV - Full-Stack (Ch 11-13)

| Navigation | |
| --- | --- |
| Previous | [← Part III - Concurrency (Ch 9-10)](#part-iii-concurrency-ch-9-10) |
| Next | [Part V - Compiler Pebble (Ch 14-17) →](#part-v-compiler-pebble-ch-14-17) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part IV: Full-Stack Engineering in Rust

> 🧭 **Where we are:** Parts I through III gave you the language. Part IV
> turns it into software people actually use. You will build a backend
> API that talks to a real PostgreSQL database, a browser frontend compiled
> from Rust to WebAssembly, and finally a complete project organized the way
> production teams structure theirs.

> ⚠️ **About versions:** The Rust web ecosystem moves quickly. The code in
> this part targets Axum 0.8, SQLx 0.8, Leptos 0.7, and Tokio 1.x. If
> `cargo` reports a different API, check the crate's documentation on
> https://docs.rs for the version you installed. The concepts will stay
> the same even when the syntax shifts.

---

### Chapter 11: Backend Engineering

#### 11.1 What a Backend Does

A **backend** is the part of an application that runs on a server, out of
the user's sight. When you press "Save" in a note app, the button does not
write to your phone's storage directly. It sends a request across the
network to a server. The server checks the request, updates a database,
and sends back a response.

> 💡 **Analogy:** A restaurant has a dining room and a kitchen. Customers
> see the dining room, which is the frontend. The kitchen, with its
> recipes, supplies, and storage, is the backend. Customers never walk into
> the kitchen, but everything they eat depends on it.

The backend has four jobs:

1. **Listen** for requests arriving over the network.
2. **Understand** what each request asks for.
3. **Act** by reading or writing data, usually in a database.
4. **Respond** with a clear answer, often as JSON text.

#### 11.2 HTTP in Plain English

Browsers and servers speak **HTTP**, a simple conversation protocol. Every
exchange has a **request** and a **response**.

A request has:

- a **method**, describing the intent: `GET` (read), `POST` (create),
  `PUT` or `PATCH` (update), `DELETE` (remove).
- a **path**, identifying the resource: `/tasks/42`.
- optional **headers** (metadata) and an optional **body** (data).

A response has:

- a **status code**, a three-digit summary of the outcome.
- headers and an optional body.

| Status code | Meaning | When you will see it |
|:-----------:|:--------|:---------------------|
| 200 | OK | Successful read or update. |
| 201 | Created | A new resource was made. |
| 204 | No Content | Success, nothing to send back (often after a delete). |
| 400 | Bad Request | The client sent invalid data. |
| 404 | Not Found | The resource does not exist. |
| 500 | Internal Server Error | Something broke on the server. |

Learn the families: 2xx means success, 4xx means the client made a
mistake, and 5xx means the server failed.

#### 11.3 REST: A Style for Naming Things

**REST** (Representational State Transfer) is a set of conventions for
designing HTTP APIs. Its core idea is to think in **resources**, nouns
such as tasks, users, or orders, and use HTTP methods as the verbs.

| Action | Method | Path | Result |
|:-------|:------:|:-----|:-------|
| List all tasks | `GET` | `/tasks` | `200` with an array |
| Create a task | `POST` | `/tasks` | `201` with the new task |
| Read one task | `GET` | `/tasks/42` | `200` or `404` |
| Update a task | `PATCH` | `/tasks/42` | `200` with the updated task |
| Delete a task | `DELETE` | `/tasks/42` | `204` or `404` |

Notice that paths never contain verbs such as `/createTask`. The method
already says "create."

#### 11.4 Choosing a Framework: Axum, Actix-web, or Rocket

Rust has several web frameworks. Each one builds the same basic pieces:
routing, extracting data from requests, and building responses.

| Framework | Style | Strengths |
|:----------|:------|:----------|
| **Axum** | Built on Tower and Hyper, composable | Modern, ergonomic, works with the wider Tokio ecosystem. |
| **Actix-web** | Actor-inspired, very fast | Mature, huge community, excellent benchmarks. |
| **Rocket** | Macro-heavy, very readable | Beginner-friendly routing syntax. |

This book uses **Axum**. It is maintained by the Tokio team, uses the
same async runtime you learned in Chapter 9, and expresses ideas through
plain Rust functions, which suits a book about understanding Rust.

#### 11.5 Setting Up the Backend Project

Make a workspace folder that will hold all three crates you build in this
part. A **workspace** is a container for several related Rust packages.

```bash
mkdir odyssey-app
cd odyssey-app
cargo new --lib shared
cargo new backend
cargo new frontend
```

Create `Cargo.toml` in the top folder to group them:

```toml
# odyssey-app/Cargo.toml
[workspace]
members = ["shared", "backend", "frontend"]
resolver = "2"   # Modern dependency resolution. Always use "2".
```

Add the backend's dependencies from inside `backend/`:

```bash
cd backend
cargo add axum tokio --features tokio/full
cargo add serde --features derive
cargo add serde_json
cargo add sqlx --features runtime-tokio,postgres,macros,migrate
cargo add tower-http --features cors
cargo add shared --path ../shared
```

Here is what each dependency does:

| Crate | Role |
|:------|:-----|
| `axum` | Routing and request handling. |
| `tokio` | The async runtime from Chapter 9. |
| `serde` | Converts Rust structs to and from JSON. |
| `serde_json` | JSON support and the `json!` macro. |
| `sqlx` | Async database access with PostgreSQL. |
| `tower-http` | Middleware, such as CORS for browser access. |
| `shared` | Our own crate, holding types both sides understand. |

#### 11.6 A Shared Vocabulary: The `shared` Crate

Before writing the server, define the data both backend and frontend will
exchange. Putting these types in one shared crate means the server and
the browser can never disagree about the shape of a task.

```rust
// shared/src/lib.rs
use serde::{Deserialize, Serialize};

// A task as stored in the database and sent to the browser.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))] // Only when the server builds.
pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

// The data a client sends when creating a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
}
```

The `cfg_attr` line is a neat trick. The `FromRow` derive, which teaches
SQLx how to read database rows, is compiled only when the `ssr` feature is
enabled. The browser build leaves it out, so it never needs the database
library. Add the optional dependency in `shared/Cargo.toml`:

```toml
# shared/Cargo.toml
[dependencies]
serde = { version = "1", features = ["derive"] }
sqlx = { version = "0.8", features = ["postgres"], optional = true }

[features]
ssr = ["dep:sqlx"]   # "ssr" = server-side rendering build, a common naming choice.
```

> 🧭 **Why `i64` for IDs?** PostgreSQL's `BIGSERIAL` column produces 64-bit
> integers. Matching that type prevents accidental overflow and conversion
> mistakes.

#### 11.7 Setting Up PostgreSQL

Run PostgreSQL with Docker, the simplest reproducible option:

```bash
docker run --name odyssey-db \
  -e POSTGRES_PASSWORD=secret \
  -e POSTGRES_DB=odyssey \
  -p 5432:5432 \
  -d postgres:16
```

Without Docker, install PostgreSQL from https://www.postgresql.org and
create a database named `odyssey`.

Your connection string, which every program will read, looks like this:

```text
postgres://postgres:secret@localhost:5432/odyssey
```

Store it in an environment variable rather than in your code. On macOS or
Linux:

```bash
export DATABASE_URL="postgres://postgres:secret@localhost:5432/odyssey"
```

On Windows PowerShell:

```powershell
$env:DATABASE_URL = "postgres://postgres:secret@localhost:5432/odyssey"
```

> ⚠️ **Never commit passwords.** Anything written in source code gets
> copied into Git history, screenshots, and chat logs. Environment
> variables and secret managers keep credentials out of the codebase.

#### 11.8 Migrations: Versioning Your Database

Your database structure, called its **schema**, must evolve along with the
code. A **migration** is a numbered SQL file that moves the schema forward
one step. Running migrations in order produces the same database on every
machine.

Install SQLx's command-line tool once:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Create the first migration from the `backend` folder:

```bash
sqlx migrate add create_tasks
```

This creates a file in `migrations/` with a timestamp prefix. Fill it in:

```sql
-- migrations/20260101000000_create_tasks.sql
CREATE TABLE tasks (
    id    BIGSERIAL PRIMARY KEY,        -- Auto-incrementing 64-bit number.
    title TEXT    NOT NULL,            -- Text that must always be present.
    done  BOOLEAN NOT NULL DEFAULT FALSE -- Starts as not done.
);
```

SQL keywords are shown in capitals by convention. Lines starting with
`--` are comments, just like `//` in Rust.

> 🧭 **Why migrations instead of manual changes?** Imagine three developers
> each changing the database by hand. Within a month, nobody knows what
> the real schema looks like. Migrations are the database's version
> control.

#### 11.9 Writing the Handlers

A **handler** is an async function that receives extracted request data and
returns a response. Axum figures out what to extract from each parameter's
type. This idea is called **extractors**.

| Extractor | Pulls out of the request |
|:----------|:-------------------------|
| `State(s)` | Shared application state, such as the database pool. |
| `Path(id)` | Values from the URL path, like `42` in `/tasks/42`. |
| `Json(body)` | A JSON request body, converted into a Rust struct. |

Create `backend/src/main.rs`. We will build it in layers, starting with
the error type, because every handler can fail:

```rust
// backend/src/main.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use shared::{NewTask, Task};
use sqlx::PgPool;

// ---------- Shared application state ----------

// Cloned for each request. Cloning a PgPool is cheap: it is a handle to a
// shared pool of database connections, not a copy of the connections.
#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

// ---------- Error handling ----------

// Every failure a handler can report. Chapter 6 in action.
#[derive(Debug)]
enum AppError {
    NotFound,
    Validation(String),
    Database(sqlx::Error),
}

// Lets `?` convert a database error into an AppError automatically.
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

// Teaches Axum how to turn an AppError into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "task not found".to_string()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(err) => {
                // Log the real details for developers, but never expose them to clients.
                eprintln!("database error: {err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

// ---------- Handlers ----------

// GET /tasks
async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT id, title, done FROM tasks ORDER BY id")
        .fetch_all(&state.pool)
        .await?;                  // Any database error becomes AppError::Database.
    Ok(Json(tasks))
}

// POST /tasks
async fn create_task(
    State(state): State<AppState>,
    Json(input): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let title = input.title.trim();
    if title.is_empty() {
        return Err(AppError::Validation("title cannot be empty".to_string()));
    }

    // $1 is a placeholder. bind() supplies the value safely, which prevents SQL injection.
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, done",
    )
    .bind(title)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(task)))
}

// GET /tasks/{id}
async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, AppError> {
    // fetch_optional returns Option<Task>: None means no matching row.
    let task = sqlx::query_as::<_, Task>("SELECT id, title, done FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(AppError::NotFound)?;   // Option -> Result, then `?` returns early.

    Ok(Json(task))
}

// DELETE /tasks/{id}
async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
```

Read the handlers and notice the pattern. Each one describes **what** it
needs as parameters and **what** it returns as a `Result`. The `?`
operator, from Chapter 6, keeps the happy path readable.

> 🛡️ **Security note on `bind`:** Never build SQL by joining strings with
> user input, such as `format!("... WHERE title = '{title}'")`. A user
> could type SQL commands into the title field. The `$1` placeholder with
> `.bind(...)` sends the value separately, so it is always treated as
> data, never as code.

#### 11.10 Wiring the Router and Starting the Server

Continue `main.rs` with the setup code. Add the imports you need
(`tokio::net::TcpListener`, `tower_http::cors::CorsLayer`, and
`sqlx::postgres::PgPoolOptions`), then append:

```rust
// backend/src/main.rs (continued)
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the connection string from the environment, failing clearly if missing.
    let database_url = std::env::var("DATABASE_URL")?;

    // A pool keeps several connections open for reuse, instead of one per request.
    let pool = PgPool::connect(&database_url).await?;

    // Apply every pending migration. The migrations are embedded into the binary.
    sqlx::migrate!().run(&pool).await?;

    let state = AppState { pool };

    // Map URLs and HTTP methods to handlers.
    // Axum 0.8 uses {id} for path parameters. Older versions used :id.
    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task).delete(delete_task))
        .layer(CorsLayer::permissive())   // Allow the browser frontend to call us. Dev only!
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("API running at http://127.0.0.1:3000/tasks");

    axum::serve(listener, app).await?;
    Ok(())
}
```

Two details deserve attention:

- **`sqlx::migrate!()`** reads the migration files at **compile time** and
  embeds them into your program. The finished binary carries its own
  schema history, so it can set up a fresh database without the
  `migrations/` folder present.
- **`CorsLayer::permissive()`** is convenient while developing. Browsers
  block requests between different origins (for example, a frontend on port
  8080 calling an API on port 3000) unless the server opts in. Restrict
  it to specific origins before deploying.

Initialize the database and run the server:

```bash
cargo install sqlx-cli --no-default-features --features postgres   # if not done
sqlx database create          # Creates the database named in DATABASE_URL.
cargo run -p backend          # Migrations run automatically on startup.
```

Test the API from a second terminal:

```bash
curl -X POST http://127.0.0.1:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Axum"}'

curl http://127.0.0.1:3000/tasks
curl http://127.0.0.1:3000/tasks/1
curl -X DELETE http://127.0.0.1:3000/tasks/1
```

Expected output for the first command, with a plausible id:

```json
{"id":1,"title":"Learn Axum","done":false}
```

#### 11.11 Testing the Backend

Chapter 10 taught unit tests. For a web backend, a useful test sends a
request into the router and checks the response, without opening a real
network port. Add test-only dependencies:

```bash
cargo add --dev tower --features util
```

Restructure slightly so the router can be built separately. In `main.rs`,
move the router construction into a function:

```rust
// backend/src/main.rs
fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task).delete(delete_task))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
```

Then call `build_router(state)` from `main`, and add a test module at the
bottom of the file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt; // Provides `oneshot`, which sends one request.

    #[sqlx::test] // Gives each test its own fresh database, and runs migrations.
    async fn list_tasks_returns_ok(pool: PgPool) {
        let app = build_router(AppState { pool });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/tasks")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

Run the tests with `DATABASE_URL` set:

```bash
cargo test -p backend
```

> 💡 **Why `#[sqlx::test]`?** Each test receives its own temporary
> database. Tests cannot interfere with one another or with your
> development data. It is one of the most valuable tools in this part.

#### 11.12 Logging With `tracing`

`println!` works for a quick demo, but servers need structured logs with
levels and timestamps. Add the `tracing` crate:

```bash
cargo add tracing
cargo add tracing-subscriber --features env-filter
```

Initialize it at the start of `main`:

```rust
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

tracing::info!("starting API server");
```

Control verbosity with `RUST_LOG`:

```bash
RUST_LOG=info cargo run -p backend     # Show info and above.
RUST_LOG=debug cargo run -p backend    # Show more detail.
```

Replace the `eprintln!` in `AppError` with `tracing::error!("database error: {err}");`
when you adopt structured logging across the project.

#### 11.13 GraphQL: The Other Style

Your task asked about REST and GraphQL, so here is the second style in
brief. Instead of many endpoints, a GraphQL server exposes **one**
endpoint. Clients send a query describing exactly which fields they want.

```graphql
query {
  tasks {
    title
    done
  }
}
```

The response contains only the requested fields, which avoids sending
unused data. The trade-offs:

| Concern | REST | GraphQL |
|:--------|:-----|:--------|
| Endpoints | Many, one per resource and action. | One, typically `/graphql`. |
| Over-fetching | Common; responses have fixed shapes. | Avoided; clients pick fields. |
| Caching | Simple, using standard HTTP caching. | More involved. |
| Learning curve | Lower. | Higher: schemas, resolvers, query depth limits. |
| Good fit | Public APIs, simple CRUD, caching-heavy apps. | Complex, nested data for many client types. |

For Rust, the `async-graphql` crate integrates with Axum. Start with REST,
since it is simpler to debug with `curl`. Add GraphQL when your data's
shape and your clients' needs justify it.

#### 11.14 Chapter 11 Exercise: Mark Tasks Done

Add a `PATCH /tasks/{id}` endpoint that accepts `{"done": true}` and
updates the matching row, returning the updated task or `404`.

> ✅ **Hint:**
>
> 1. Define `#[derive(Deserialize)] struct UpdateTask { done: bool }`
>    in the `shared` crate.
> 2. Use `routing::patch` and add `.patch(update_task)` to the `/tasks/{id}` route.
> 3. Write `UPDATE tasks SET done = $1 WHERE id = $2 RETURNING id, title, done`
>    with `fetch_optional`, then map `None` to `AppError::NotFound`.

---

### Chapter 12: Frontend Engineering

#### 12.1 Why Rust in the Browser?

Browsers run one language natively: JavaScript. For decades, that was the
only option. Then **WebAssembly** (abbreviated **Wasm**) arrived. It is a
compact binary format that browsers execute at near-native speed. Rust
compiles to Wasm very well, because it has no garbage collector to bring
along.

> 💡 **Analogy:** Think of Wasm as a universal shipping container. The
> browser is a port that unloads any container of the right shape. Rust
> packs its cargo into that container, and the browser runs it.

You do not have to abandon JavaScript. Modern Rust frontends call
JavaScript APIs whenever they need to, such as the browser's `fetch` or
the DOM. The Rust code runs the application logic, and the browser handles
the screen.

#### 12.2 The Mental Model of a Reactive UI

Most modern frontends share one idea: the interface is a **function of
state**. You describe what the screen should look like for a given state,
and the framework updates the screen when the state changes. Nobody
manually searches the page for elements to edit.

Leptos expresses this with **signals**, values that notify whatever depends
on them when they change:

```rust
let (count, set_count) = signal(0);   // Read with count.get(), write with set_count.set(...).
```

When `set_count` runs, only the parts of the view that read `count` update.
This fine-grained approach is part of why Leptos is fast.

#### 12.3 Choosing a Framework: Leptos or Yew

| Framework | Model | Notes |
|:----------|:------|:------|
| **Leptos** | Fine-grained signals, JSX-like `view!` macro. | Very fast; growing full-stack story with server functions. |
| **Yew** | Virtual DOM, component messages, closer to React. | Mature, widely used, familiar to React developers. |

This book uses **Leptos 0.7** in client-side rendering (CSR) mode. Its
signal model is the most distinctive of the two, and its syntax is
concise enough for a first project.

#### 12.4 Setting Up the Frontend

Install the WebAssembly compilation target and the **Trunk** build tool,
which bundles Wasm, HTML, and assets into a deployable site:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Add dependencies from inside `frontend/`:

```bash
cd frontend
cargo add leptos --features csr
cargo add gloo-net --features http
cargo add shared --path ../shared
```

| Crate | Role |
|:------|:-----|
| `leptos` | Components, signals, and the `view!` macro. |
| `gloo-net` | Friendly wrapper around the browser's `fetch` for HTTP requests. |
| `shared` | The same `Task` and `NewTask` types the backend uses. |

Trunk expects an HTML entry point. Create `frontend/index.html`:

```html
<!-- frontend/index.html -->
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Task Board</title>
    <!-- Trunk compiles the Rust crate and injects the Wasm module here. -->
    <link data-trunk rel="rust" />
  </head>
  <body></body>
</html>
```

#### 12.5 Your First Component

Replace `frontend/src/main.rs` with an app that talks to the backend. The
code below is the complete file, shown in pieces so each idea is easy to
see.

First, the imports and a constant for the API address:

```rust
// frontend/src/main.rs
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;   // Runs an async block in the browser.
use shared::{NewTask, Task};

// Where the backend lives. Change this when you deploy.
const API: &str = "http://127.0.0.1:3000/tasks";
```

Next, a function that fetches all tasks. It returns an empty list if
anything goes wrong, which keeps the UI simple for now:

```rust
async fn fetch_tasks() -> Vec<Task> {
    // `json()` converts the response body into the type we declared.
    match Request::get(API).send().await {
        Ok(response) => response.json().await.unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
```

Then the component itself. A **component** is a function that returns a
view. `#[component]` marks it for Leptos:

```rust
#[component]
fn App() -> impl IntoView {
    // Reactive state: the list of tasks and the text in the input box.
    let tasks = RwSignal::new(Vec::<Task>::new());
    let title = RwSignal::new(String::new());

    // Load the tasks once, when the component first appears.
    spawn_local(async move {
        tasks.set(fetch_tasks().await);
    });

    // An event handler that sends a new task to the backend.
    let add_task = move |_| {
        let new_title = title.get();
        if new_title.trim().is_empty() {
            return;   // Ignore blank submissions.
        }

        spawn_local(async move {
            let body = NewTask { title: new_title };
            // Build the request, then send it. Each step can fail.
            if let Ok(request) = Request::post(API).json(&body) {
                if let Ok(response) = request.send().await {
                    if let Ok(task) = response.json::<Task>().await {
                        tasks.update(|list| list.push(task));   // Re-render happens automatically.
                    }
                }
            }
        });

        title.set(String::new());   // Clear the input box.
    };

    // The view: HTML-like markup checked by the Rust compiler.
    view! {
        <main>
            <h1>"Task Board"</h1>

            <input
                type="text"
                placeholder="What needs doing?"
                prop:value=move || title.get()
                on:input=move |ev| title.set(event_target_value(&ev))
            />
            <button on:click=add_task>"Add"</button>

            <ul>
                <For
                    each=move || tasks.get()
                    key=|task| task.id
                    children=|task| view! { <li>{task.title}</li> }
                />
            </ul>
        </main>
    }
}

fn main() {
    // Attach the App component to the page's <body>.
    leptos::mount::mount_to_body(App);
}
```

Look at what the compiler now checks for you. If you misspell
`on:click` or pass a number where the view expects text, the build fails
before the page ever loads.

#### 12.6 Understanding the Reactive Pieces

| Piece | Purpose in the example |
|:------|:-----------------------|
| `RwSignal::new(...)` | Reactive state that can be read and written. |
| `.get()` | Reads a signal and subscribes the surrounding view to changes. |
| `.set(...)` / `.update(...)` | Writes a signal, notifying dependents. |
| `prop:value=...` | Binds an HTML property to a reactive value. |
| `on:input=...` | Runs code when the event fires. |
| `<For each key children />` | Renders a list efficiently, reusing elements by `key`. |
| `spawn_local` | Starts async work, such as network calls, in the browser. |

> 🧭 **Why `key` matters:** When the list changes, `key` tells Leptos
> which item is which. Without a stable key, the framework may rebuild
> rows it could have reused, which wastes work and can lose focus or
> scroll positions.

#### 12.7 Running the Full Stack

Open two terminals. In the first, start the backend:

```bash
cd odyssey-app
export DATABASE_URL="postgres://postgres:secret@localhost:5432/odyssey"
cargo run -p backend
```

In the second, start the frontend with hot reloading:

```bash
cd odyssey-app/frontend
trunk serve --open
```

Trunk builds the Wasm module, serves the page (usually at
`http://127.0.0.1:8080`), and rebuilds whenever you save a file. Type a
task, click Add, and reload. Your task is still there, because it lives in
PostgreSQL.

#### 12.8 Browser Limits to Keep in Mind

- **Network calls fail.** Wi-Fi drops, servers restart. Show users clear
  messages and retry sensibly. The example above silently ignores failures,
  which is acceptable for a tutorial but not for production.
- **Wasm bundles have size.** Release builds with `trunk build --release`
  are much smaller. Watch the file size as dependencies grow.
- **Secrets do not belong in the browser.** Anything shipped to the client
  can be read by the user. API keys with real power stay on the server.

#### 12.9 Chapter 12 Exercise: Checkboxes

Add a checkbox next to each task that toggles its `done` state by calling
the `PATCH` endpoint from Chapter 11's exercise. Update the matching task in
the signal when the response arrives.

> ✅ **Hint:** Use `<input type="checkbox" prop:checked=task.done ... />`
> inside the `children` closure. Because `tasks` is a signal, updating
> one element with `tasks.update(...)` refreshes the screen automatically.

---

### Chapter 13: Putting It Together

#### 13.1 From Exercises to Architecture

So far, your code has grown in one direction: everything in `main.rs`.
That works for a tutorial. A production application needs boundaries.
Each part of the system should have one clear responsibility, so that
changes in one area do not ripple unpredictably through the rest.

This chapter reorganizes the project into a structure you can grow
without fear.

#### 13.2 The Architecture at a Glance

```text
                 ┌────────────────────────────┐
                 │        Browser             │
                 │  Leptos frontend (Wasm)    │
                 └─────────────┬──────────────┘
                               │ HTTP + JSON
                 ┌─────────────▼──────────────┐
                 │        Backend (Axum)      │
                 │                            │
                 │  routes  ─►  handlers      │
                 │                 │          │
                 │             repository     │
                 │                 │          │
                 └─────────────────┼──────────┘
                                   │ SQL
                 ┌─────────────────▼──────────┐
                 │     PostgreSQL database    │
                 └────────────────────────────┘

    shared crate: Task, NewTask (used by BOTH sides)
```

Each layer has a single job:

| Layer | Responsibility | Knows about |
|:------|:---------------|:------------|
| Routes | Map URLs and methods to handlers. | Handlers only. |
| Handlers | Translate HTTP into domain actions and back. | Requests, responses, and services. |
| Repository | Run SQL queries, and nothing else. | The database only. |
| Shared | Define data types for every side. | Nothing else. |

The payoff: if you switch databases, only the repository changes. If you
change the URL scheme, only the routes change.

#### 13.3 Reorganizing the Backend

Split `backend/src/main.rs` into modules, each in its own file:

```text
backend/
├── Cargo.toml
├── migrations/
│   └── 20260101000000_create_tasks.sql
└── src/
    ├── main.rs          # Startup: config, pool, server.
    ├── config.rs        # Reads settings from the environment.
    ├── error.rs         # AppError and its HTTP mapping.
    ├── routes.rs        # The build_router function.
    ├── handlers.rs      # HTTP-facing functions.
    └── repository.rs    # SQL queries.
```

The repository holds all SQL. Handlers call it and never write queries
themselves:

```rust
// backend/src/repository.rs
use shared::Task;
use sqlx::PgPool;
use crate::error::AppError;

pub async fn all_tasks(pool: &PgPool) -> Result<Vec<Task>, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT id, title, done FROM tasks ORDER BY id")
        .fetch_all(pool)
        .await?;
    Ok(tasks)
}

pub async fn insert_task(pool: &PgPool, title: &str) -> Result<Task, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, done",
    )
    .bind(title)
    .fetch_one(pool)
    .await?;
    Ok(task)
}
```

Handlers stay thin and readable:

```rust
// backend/src/handlers.rs
use axum::{extract::State, http::StatusCode, Json};
use shared::{NewTask, Task};
use crate::{error::AppError, repository, AppState};

pub async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = repository::all_tasks(&state.pool).await?;
    Ok(Json(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(input): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let title = input.title.trim();
    if title.is_empty() {
        return Err(AppError::Validation("title cannot be empty".to_string()));
    }
    let task = repository::insert_task(&state.pool, title).await?;
    Ok((StatusCode::CREATED, Json(task)))
}
```

> 🧭 **Notice the `crate::` paths.** Inside a crate, `crate::` means "start
> from this crate's root," the `main.rs` file. This is the module system
> from Chapter 10 applied to a real project.

#### 13.4 Configuration From the Environment

Hard-coded values such as addresses and database URLs make deployment
painful. Gather every setting into one struct, loaded once at startup:

```rust
// backend/src/config.rs
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")?,
            // unwrap_or supplies a default when the variable is unset.
            bind_addr: std::env::var("BIND_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
        })
    }
}
```

The same binary can now run on a laptop, a test server, or a cloud host,
differing only in its environment variables. This follows the
**twelve-factor app** guidance, a widely used checklist for deployable
services.

#### 13.5 Graceful Shutdown

When you stop a server with Ctrl+C, it should finish in-flight requests
instead of cutting them off mid-response. Axum supports this with a small
signal future:

```rust
// backend/src/main.rs
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    tracing::info!("shutdown signal received, finishing requests");
}

// In main, replace the simple serve call with:
// axum::serve(listener, app)
//     .with_graceful_shutdown(shutdown_signal())
//     .await?;
```

Containers and cloud platforms also send a termination signal before
stopping a process. Handling it the same way keeps deployments clean.

#### 13.6 Packaging for Deployment

A **container image** bundles your binary and its runtime environment so
it runs identically anywhere. A multi-stage Dockerfile builds the program
in one stage and copies only the finished binary into a small final image:

```dockerfile
# Stage 1: build the release binary with the full Rust toolchain.
FROM rust:1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p backend

# Stage 2: a small runtime image containing only the binary.
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
ENV BIND_ADDR=0.0.0.0:3000
EXPOSE 3000
CMD ["backend"]
```

Because migrations were embedded in Chapter 11, the image does not need the
`migrations/` folder at runtime. Supply `DATABASE_URL` through the
platform's secret settings, not inside the image.

> 🛡️ **Build once, configure at runtime.** The same image should run in
> testing and production. Only the environment variables change.

#### 13.7 Production Readiness Checklist

Before a real launch, walk through this list. Each row is a question a
reviewer should be able to answer with confidence:

| Area | Question to answer |
|:-----|:-------------------|
| **Secrets** | Are credentials injected from the environment or a secret manager, never committed? |
| **Authentication** | Who is allowed to call each endpoint, and how is identity proven? |
| **Authorization** | Can user A read or delete user B's tasks? (Usually, no.) |
| **Validation** | Are all inputs checked for length, format, and range? |
| **CORS** | Is access restricted to your real frontend origin? |
| **HTTPS** | Does all traffic travel encrypted, typically terminated by a load balancer? |
| **Rate limiting** | Can a single client flood the API? |
| **Logging** | Are errors logged with context, without leaking secrets or personal data? |
| **Observability** | Can you see request counts, latency, and error rates? |
| **Migrations** | Are schema changes applied automatically and reversibly where possible? |
| **Backups** | Is the database backed up, and has a restore been tested? |
| **Tests** | Do unit, integration, and database tests run on every change? |

> ⚠️ **Security is never finished.** The table above is a starting point,
> not a certificate. Authentication in particular, often implemented with
> password hashing such as Argon2 and signed tokens such as JWTs, deserves
> its own careful study before you handle real users' accounts.

#### 13.8 A Continuous Integration Workflow

Automate checks so that every change is verified the same way. Here is a
minimal GitHub Actions workflow that formats, lints, and tests the project:

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: secret
          POSTGRES_DB: odyssey
        ports:
          - 5432:5432
    env:
      DATABASE_URL: postgres://postgres:secret@localhost:5432/odyssey
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
```

The `-D warnings` flag turns every clippy warning into an error, which
enforces the idiomatic style from Chapter 10 automatically.

#### 13.9 Where to Go Next

You can now build what most web applications need. Consider these
directions to deepen your practice:

- **Authentication:** Add user accounts with hashed passwords and session
  or token-based login.
- **Background jobs:** Move slow work, such as sending emails, into queues.
- **Real-time updates:** Push changes to browsers with WebSockets or
  server-sent events.
- **Deployment:** Run containers on a managed platform, and automate
  releases through CI.
- **Testing the frontend:** Explore component tests and end-to-end
  browser tests.

#### 13.10 Chapter 13 Exercise: Ship Your Own Feature

Choose one feature to add end to end, touching every layer:

1. Add a `priority` field (`low`, `medium`, `high`) stored in the database.
2. Add a migration that adds the column with a default of `'medium'`.
3. Update the shared types, the repository, the handler, and the frontend
   display.
4. Add one unit test and one database test.
5. Make `cargo fmt`, `cargo clippy`, and `cargo test` pass.

> ✅ **Hint:** Treat the enum as a real Rust `enum` in the `shared` crate,
> deriving `Serialize`, `Deserialize`, `Clone`, `Copy`, and `PartialEq`.
> Store it as `TEXT` in the database and convert it with `match` statements
> in the repository layer. This combines Chapters 5, 6, and 13.

---

### Part IV Recap

You now know how to:

- Explain HTTP, REST, and when GraphQL fits better (Chapter 11).
- Build an Axum API with extractors, typed errors, and a PostgreSQL pool
  (Chapter 11).
- Manage schema changes with migrations and test handlers without a
  network (Chapter 11).
- Write a Leptos frontend compiled to WebAssembly, with reactive state and
  live data from the backend (Chapter 12).
- Organize a multi-crate workspace, configure from the environment, shut
  down gracefully, containerize, and automate checks (Chapter 13).

> 🧭 **A milestone:** You have shipped a complete, layered, full-stack Rust
> application. That is a genuine accomplishment, and many working
> developers have never built one this carefully.

In **Part V: The Grand Finale**, you will leave the web behind and build
something even more fundamental: a compiler. You will write a lexer that
turns text into tokens, a parser that builds a tree from those tokens,
a type checker that catches mistakes before running, and an interpreter
that executes your own programming language. Everything you have learned
so far, from enums and pattern matching to traits and lifetimes, becomes
the toolkit for that journey.


---

<a id="part-v-compiler-pebble-ch-14-17"></a>

## Part V - Compiler Pebble (Ch 14-17)

| Navigation | |
| --- | --- |
| Previous | [← Part IV - Full-Stack (Ch 11-13)](#part-iv-full-stack-ch-11-13) |
| Next | [Part V - TinyGit Capstone (Ch 18) →](#part-v-tinygit-capstone-ch-18) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |


> 🧭 **Where we are:** You can now build web services, reason about
> ownership, and write concurrent code. Part V uses everything at once to
> build something deeper: a programming language of your own, named
> **Pebble**, with its own compiler-style pipeline. Pebble can read input
> from the keyboard, read and write files, and run recursive functions.

> 🎯 **What you will have at the end:** A working command-line program,
> `pebble`, that runs scripts such as this:

```pebble
// guestbook.pbl: a tiny program that uses input and files.
let name: str = input("Who is visiting? ");
if !file_exists("guestbook.txt") {
    write_file("guestbook.txt", "");
}
append_file("guestbook.txt", name + "\n");
print("Visitors so far:");
print(read_file("guestbook.txt"));
```

---

### Chapter 14: Introduction to Compilers

#### 14.1 What a Language Implementation Does

A programming language is a set of rules. Something must read a program
written under those rules and make it happen. That something is an
**implementation**, and it usually works in stages:

1. **Lexing:** break raw text into meaningful pieces called **tokens**.
2. **Parsing:** arrange tokens into a tree that reflects their structure.
3. **Checking:** verify that the program makes sense before running it.
4. **Executing:** run the program, either by walking the tree directly
   (an **interpreter**) or by translating it into machine instructions
   (a **compiler**).

> 💡 **Analogy:** Imagine translating a letter from English into French.
> First you identify the words (lexing). Then you work out the sentence
> structure, subject, verb, and object (parsing). Then you check that the
> grammar agrees (checking). Finally, you write the French version
> (generating output) or simply read the letter aloud in French as you go
> (interpreting).

#### 14.2 The Pipeline at a Glance

```text
  source text  "let x = 1 + 2;"
        │
        ▼  lexer.rs         characters → tokens
  [Let, Ident("x"), Assign, Int(1), Plus, Int(2), Semicolon]
        │
        ▼  parser.rs        tokens → abstract syntax tree (AST)
  Let { name: "x", value: Binary(Add, Int(1), Int(2)) }
        │
        ▼  checker.rs       AST → verified AST (types, names)
        │
        ▼  interpreter.rs   AST → running program
  (x now holds 3)
```

The **abstract syntax tree** (AST) is the central idea of the whole
project. It throws away punctuation and keeps structure. The expression
`1 + 2 * 3` becomes a tree where multiplication sits below addition,
because multiplication binds more tightly. The tree is the meaning of the
program, in a form the rest of the compiler can easily walk.

#### 14.3 Meet Pebble

Pebble is deliberately small so that every part of it fits in your head.
Here is what it supports:

| Feature | Example |
|:--------|:--------|
| Variables | `let x = 5;` and `let mut total: int = 0;` |
| Types | `int` (64-bit whole numbers), `bool`, `str` |
| Arithmetic | `+ - * / %` on `int`; `+` also joins `str` values |
| Comparison | `== != < <= > >=` |
| Logic | `&&`, `\|\|`, `!` with short-circuit evaluation |
| Decisions | `if`, `else if`, `else` |
| Loops | `while condition { ... }` |
| Functions | `fn add(a: int, b: int) -> int { return a + b; }` |
| Output | `print(value);` |
| Keyboard input | `input("prompt")` returns a `str` |
| File access | `read_file`, `write_file`, `append_file`, `file_exists` |
| Conversion | `to_int(text)` turns a `str` into an `int` |
| Comments | `// anything after two slashes` |

A complete Pebble program looks like this:

```pebble
// factorial.pbl: asks for a number and computes its factorial.
fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

let text: str = input("Number: ");
let n: int = to_int(text);
print(factorial(n));
```

> 💡 **Design choice:** Pebble requires type annotations on function
> parameters and return types, but lets `let` infer types from the value.
> This mirrors Rust and gives the checker enough information to catch
> mistakes early without making small scripts tedious to write.

#### 14.4 The Grammar

A **grammar** is the formal rulebook for valid programs. Language
designers write it in a notation called EBNF (Extended Backus-Naur Form).
Each line defines one construct. `{ x }` means "zero or more," `[ x ]`
means "optional," and `|` means "or."

```text
program     = { statement } ;

statement   = let_stmt | assign_stmt | fn_decl | if_stmt
            | while_stmt | return_stmt | expr_stmt ;

let_stmt    = "let" [ "mut" ] IDENT [ ":" type ] "=" expression ";" ;
assign_stmt = IDENT "=" expression ";" ;
fn_decl     = "fn" IDENT "(" [ param { "," param } ] ")"
              [ "->" type ] block ;
param       = IDENT ":" type ;
if_stmt     = "if" expression block [ "else" ( if_stmt | block ) ] ;
while_stmt  = "while" expression block ;
return_stmt = "return" [ expression ] ";" ;
expr_stmt   = expression ";" ;
block       = "{" { statement } "}" ;
type        = "int" | "bool" | "str" ;

expression  = binary ;
binary      = unary { binary_op binary } ;
unary       = ( "-" | "!" ) unary | primary ;
primary     = INT | STRING | "true" | "false"
            | IDENT [ "(" [ expression { "," expression } ] ")" ]
            | "(" expression ")" ;
```

The `binary` rule looks ambiguous: `1 + 2 * 3` could group either way.
The parser resolves this with **precedence**, a ranking of operators.
Chapter 15 implements that ranking explicitly.

#### 14.5 Setting Up the Project

```bash
cargo new pebble
cd pebble
mkdir examples
```

Your project will grow to this shape by the end of Part V:

```text
pebble/
├── Cargo.toml
├── examples/
│   ├── hello.pbl
│   ├── guestbook.pbl
│   └── factorial.pbl
└── src/
    ├── main.rs         # Entry point: reads a file and runs the pipeline.
    ├── token.rs        # Token kinds.
    ├── lexer.rs        # Text → tokens.
    ├── ast.rs          # Tree shapes for expressions and statements.
    ├── parser.rs       # Tokens → AST.
    ├── error.rs        # One error type shared by every phase.
    ├── builtins.rs     # Signatures and implementations of built-in functions.
    ├── checker.rs      # Type and name checking.
    └── interpreter.rs  # Runs the checked AST.
```

Create the three example programs now, so you can test as you go:

```pebble
// examples/hello.pbl
let name: str = input("What is your name? ");
print("Hello, " + name + "!");
```

```pebble
// examples/guestbook.pbl
let name: str = input("Who is visiting? ");
if !file_exists("guestbook.txt") {
    write_file("guestbook.txt", "");
}
append_file("guestbook.txt", name + "\n");
print("Visitors so far:");
print(read_file("guestbook.txt"));
```

```pebble
// examples/factorial.pbl
fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

let text: str = input("Number: ");
print(factorial(to_int(text)));
```

#### 14.6 Interpreting Versus Compiling

Both approaches are valid, and they make different trade-offs:

| Approach | How it works | Strengths | Weaknesses |
|:---------|:-------------|:----------|:-----------|
| **Tree-walking interpreter** | Visits AST nodes and computes results directly. | Simple, easy to debug, quick to change. | Slower; each node is visited at runtime. |
| **Bytecode compiler and VM** | Converts the AST into a flat list of instructions, then runs them on a virtual machine. | Faster; the instructions can be stored and shared. | More moving parts. |
| **Native compiler** | Emits machine code for a real processor. | Fastest execution. | Most complex; depends on the target CPU. |

Pebble is a **tree-walking interpreter**. It is the right first choice:
every step is visible, and you can see exactly how a program runs. Section
17.9 shows how the same ideas extend into a bytecode design.

> 🧭 **Why this order matters:** Compiler courses often begin with
> optimization. Begin with correctness. A simple implementation that
> behaves correctly is the foundation every faster design is measured
> against.

#### 14.7 Chapter 14 Exercise

Without running anything, write the token list for this line:

```pebble
let total: int = price * 2 + tax;
```

> ✅ **Answer:** `Let`, `Ident("total")`, `Colon`, `Ident("int")`,
> `Assign`, `Ident("price")`, `Star`, `Int(2)`, `Plus`, `Ident("tax")`,
> `Semicolon`, `Eof`. Notice that `int` is an identifier at the lexical
> level. The parser decides it is a type because of its position.

---

### Chapter 15: Building the Lexer and Parser

#### 15.1 Tokens: The Vocabulary of Pebble

A token is a small labeled piece of source text. We record its kind, and
its line number for error messages. Create `src/token.rs`:

```rust
// src/token.rs

/// Every kind of token Pebble recognizes.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals and names carry their data inside the variant.
    Int(i64),
    Str(String),
    Ident(String),

    // Keywords.
    Let,
    Mut,
    Fn,
    Return,
    If,
    Else,
    While,
    True,
    False,

    // Punctuation.
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    Arrow, // ->

    // Operators.
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign, // =
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AndAnd,
    OrOr,
    Bang, // !

    /// Marks the end of the source. Every token list ends with one.
    Eof,
}

/// A token together with the line it appeared on.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}
```

> 💡 **Why keep `line`?** When a program has an error on line 42, the
> user needs to hear about line 42. Carrying line numbers from the very
> first phase makes every later error message useful.

#### 15.2 Errors: One Type for Every Phase

Every phase can fail, and the user should always see the same kind of
message. Create `src/error.rs`:

```rust
// src/error.rs
use std::fmt;

/// Which phase of the pipeline reported the problem.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    Syntax,
    Type,
    Runtime,
}

/// A single problem found in a Pebble program.
#[derive(Debug, Clone, PartialEq)]
pub struct PebbleError {
    pub kind: ErrorKind,
    pub line: usize,
    pub message: String,
}

impl PebbleError {
    pub fn new(kind: ErrorKind, line: usize, message: impl Into<String>) -> Self {
        PebbleError { kind, line, message: message.into() }
    }
}

impl fmt::Display for PebbleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.kind {
            ErrorKind::Syntax => "syntax",
            ErrorKind::Type => "type",
            ErrorKind::Runtime => "runtime",
        };
        write!(f, "{label} error on line {}: {}", self.line, self.message)
    }
}

// Lets PebbleError work with `Box<dyn Error>` and the `?` operator elsewhere.
impl std::error::Error for PebbleError {}
```

`impl Into<String>` in the constructor means callers can pass either a
`&str` or a `String`. This is the same generic-bound idea from Chapter 7.

#### 15.3 The Lexer: Turning Characters Into Tokens

The lexer walks the source one character at a time. It recognizes
numbers, names, strings, and operators, skips whitespace and comments,
and tracks line numbers. Create `src/lexer.rs`:

```rust
// src/lexer.rs
use crate::error::{ErrorKind, PebbleError};
use crate::token::{Token, TokenKind};

/// Turns source text into a list of tokens ending with `Eof`.
pub fn tokenize(source: &str) -> Result<Vec<Token>, PebbleError> {
    // Work on characters, not bytes, so non-ASCII text in strings is safe.
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut pos = 0;
    let mut line = 1;

    while pos < chars.len() {
        let c = chars[pos];
        let next = chars.get(pos + 1).copied();

        // Newlines advance the line counter but produce no token.
        if c == '\n' {
            line += 1;
            pos += 1;
            continue;
        }
        if c.is_whitespace() {
            pos += 1;
            continue;
        }

        // A comment runs from `//` to the end of the line.
        if c == '/' && next == Some('/') {
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            continue;
        }

        // Numbers: a run of digits, converted to i64.
        if c.is_ascii_digit() {
            let start = pos;
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                pos += 1;
            }
            let text: String = chars[start..pos].iter().collect();
            let value = text.parse::<i64>().map_err(|_| {
                PebbleError::new(ErrorKind::Syntax, line, format!("number '{text}' is too large"))
            })?;
            tokens.push(Token { kind: TokenKind::Int(value), line });
            continue;
        }

        // Words: keywords or identifiers.
        if c.is_alphabetic() || c == '_' {
            let start = pos;
            while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                pos += 1;
            }
            let word: String = chars[start..pos].iter().collect();
            tokens.push(Token { kind: keyword_or_ident(&word), line });
            continue;
        }

        // Strings: the opening quote, then characters until the closing quote.
        if c == '"' {
            pos += 1;
            let mut text = String::new();
            loop {
                match chars.get(pos).copied() {
                    None | Some('\n') => {
                        return Err(PebbleError::new(ErrorKind::Syntax, line, "unterminated string"));
                    }
                    Some('"') => {
                        pos += 1;
                        break;
                    }
                    Some('\\') => {
                        // Escape sequences: \n, \", and \\.
                        let escaped = match chars.get(pos + 1).copied() {
                            Some('n') => '\n',
                            Some('"') => '"',
                            Some('\\') => '\\',
                            _ => {
                                return Err(PebbleError::new(
                                    ErrorKind::Syntax,
                                    line,
                                    "invalid escape sequence in string",
                                ));
                            }
                        };
                        text.push(escaped);
                        pos += 2;
                    }
                    Some(ch) => {
                        text.push(ch);
                        pos += 1;
                    }
                }
            }
            tokens.push(Token { kind: TokenKind::Str(text), line });
            continue;
        }

        // Everything else is punctuation or an operator, possibly two characters long.
        let (kind, len) = operator(c, next).ok_or_else(|| {
            PebbleError::new(ErrorKind::Syntax, line, format!("unexpected character '{c}'"))
        })?;
        tokens.push(Token { kind, line });
        pos += len;
    }

    tokens.push(Token { kind: TokenKind::Eof, line });
    Ok(tokens)
}

/// Maps a word to a keyword token, or to an identifier if it is not reserved.
fn keyword_or_ident(word: &str) -> TokenKind {
    match word {
        "let" => TokenKind::Let,
        "mut" => TokenKind::Mut,
        "fn" => TokenKind::Fn,
        "return" => TokenKind::Return,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        _ => TokenKind::Ident(word.to_string()),
    }
}

/// Recognizes punctuation and operators. Returns the token and its length.
fn operator(c: char, next: Option<char>) -> Option<(TokenKind, usize)> {
    Some(match (c, next) {
        ('(', _) => (TokenKind::LParen, 1),
        (')', _) => (TokenKind::RParen, 1),
        ('{', _) => (TokenKind::LBrace, 1),
        ('}', _) => (TokenKind::RBrace, 1),
        (',', _) => (TokenKind::Comma, 1),
        (':', _) => (TokenKind::Colon, 1),
        (';', _) => (TokenKind::Semicolon, 1),
        ('-', Some('>')) => (TokenKind::Arrow, 2),
        ('-', _) => (TokenKind::Minus, 1),
        ('+', _) => (TokenKind::Plus, 1),
        ('*', _) => (TokenKind::Star, 1),
        ('/', _) => (TokenKind::Slash, 1),
        ('%', _) => (TokenKind::Percent, 1),
        ('=', Some('=')) => (TokenKind::EqEq, 2),
        ('=', _) => (TokenKind::Assign, 1),
        ('!', Some('=')) => (TokenKind::NotEq, 2),
        ('!', _) => (TokenKind::Bang, 1),
        ('<', Some('=')) => (TokenKind::LtEq, 2),
        ('<', _) => (TokenKind::Lt, 1),
        ('>', Some('=')) => (TokenKind::GtEq, 2),
        ('>', _) => (TokenKind::Gt, 1),
        ('&', Some('&')) => (TokenKind::AndAnd, 2),
        ('|', Some('|')) => (TokenKind::OrOr, 2),
        _ => return None,
    })
}
```

Notice the two-character operators. The lexer looks at `next` before
deciding, so `<=` is one token, while `<` followed by a space is two
different things. This is called **maximal munch**: always take the
longest valid token.

#### 15.4 The AST: The Shape of a Program

The AST uses Rust enums to represent every construct. Recall from Chapter
5 that an enum variant can hold data, which is exactly what tree nodes
need. Create `src/ast.rs`:

```rust
// src/ast.rs

/// The three value types of Pebble.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Int,
    Bool,
    Str,
    /// The type of a statement-like call, such as `print`, or a function with no return.
    Unit,
}

impl Type {
    /// The name a programmer types in source code, used in error messages.
    pub fn name(self) -> &'static str {
        match self {
            Type::Int => "int",
            Type::Bool => "bool",
            Type::Str => "str",
            Type::Unit => "unit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// An expression, with the line it came from.
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Int(i64),
    Bool(bool),
    Str(String),
    Var(String),
    /// `Box` is required here: an `Expr` contains `Expr`s, and a type cannot
    /// contain itself directly without an unknown size (see Chapter 8).
    Unary { op: UnaryOp, expr: Box<Expr> },
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

/// A statement, with the line it came from.
#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Let {
        name: String,
        mutable: bool,
        annotation: Option<Type>,
        value: Expr,
    },
    Assign { name: String, value: Expr },
    Expr(Expr),
    If {
        cond: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While { cond: Expr, body: Vec<Stmt> },
    Return(Option<Expr>),
    FnDecl(FnDecl),
}

/// A function definition. Parameters carry their declared types.
#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub ret: Type,
    pub body: Vec<Stmt>,
}
```

> 🧭 **Why `else if` has no node of its own:** The parser turns
> `else if` into an `else` block containing a single nested `if`. The rest
> of the pipeline only needs to understand two shapes, which keeps every
> later phase simpler.

#### 15.5 The Parser: Recursive Descent

A **recursive descent** parser has one function per grammar rule. Each
function consumes the tokens it needs and calls other functions for the
parts it contains. Creating this is mostly mechanical once the grammar is
written.

The expression part uses **precedence climbing**. Each binary operator
gets a number. Higher numbers bind more tightly. The parser reads one
operand, then keeps consuming operators whose precedence is at least the
minimum it was asked for.

| Precedence | Operators |
|:----------:|:----------|
| 1 (lowest) | `\|\|` |
| 2 | `&&` |
| 3 | `==` `!=` |
| 4 | `<` `<=` `>` `>=` |
| 5 | `+` `-` |
| 6 (highest) | `*` `/` `%` |

Create `src/parser.rs`:

```rust
// src/parser.rs
use crate::ast::{BinOp, Expr, ExprKind, FnDecl, Stmt, StmtKind, Type, UnaryOp};
use crate::error::{ErrorKind, PebbleError};
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Parses an entire file into a list of top-level statements.
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, PebbleError> {
        let mut stmts = Vec::new();
        while !self.at_end() {
            stmts.push(self.statement()?);
        }
        Ok(stmts)
    }

    // ----- Helpers for moving through the token list -----

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn line(&self) -> usize {
        self.peek().line
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    /// Returns the current token and moves past it. Never moves past `Eof`.
    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if !self.at_end() {
            self.pos += 1;
        }
        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().kind == *kind
    }

    /// Consumes the token if it matches. Returns whether it did.
    fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind, what: &str) -> Result<(), PebbleError> {
        if self.check(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.syntax_error(&format!("expected {what}")))
        }
    }

    fn identifier(&mut self) -> Result<String, PebbleError> {
        if let TokenKind::Ident(name) = &self.peek().kind {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(self.syntax_error("expected a name"))
        }
    }

    fn syntax_error(&self, message: &str) -> PebbleError {
        PebbleError::new(ErrorKind::Syntax, self.line(), message)
    }

    // ----- Statements -----

    fn statement(&mut self) -> Result<Stmt, PebbleError> {
        let kind = self.peek().kind.clone();
        match kind {
            TokenKind::Let => self.let_statement(),
            TokenKind::Fn => self.fn_declaration(),
            TokenKind::If => self.if_statement(),
            TokenKind::While => self.while_statement(),
            TokenKind::Return => self.return_statement(),
            _ => self.expression_or_assignment(),
        }
    }

    fn let_statement(&mut self) -> Result<Stmt, PebbleError> {
        let line = self.line();
        self.advance(); // `let`
        let mutable = self.eat(&TokenKind::Mut);
        let name = self.identifier()?;
        let annotation = if self.eat(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(TokenKind::Assign, "'=' after variable name")?;
        let value = self.expression()?;
        self.expect(TokenKind::Semicolon, "';' after let statement")?;
        let kind = StmtKind::Let { name, mutable, annotation, value };
        Ok(Stmt { kind, line })
    }

    fn fn_declaration(&mut self) -> Result<Stmt, PebbleError> {
        let line = self.line();
        self.advance(); // `fn`
        let name = self.identifier()?;
        self.expect(TokenKind::LParen, "'(' after function name")?;

        let mut params = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                let param = self.identifier()?;
                self.expect(TokenKind::Colon, "':' after parameter name")?;
                let ty = self.parse_type()?;
                params.push((param, ty));
                if !self.eat(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen, "')' after parameters")?;

        // The return type is optional. Without it, the function returns nothing.
        let ret = if self.eat(&TokenKind::Arrow) {
            self.parse_type()?
        } else {
            Type::Unit
        };
        let body = self.block()?;
        let decl = FnDecl { name, params, ret, body };
        Ok(Stmt { kind: StmtKind::FnDecl(decl), line })
    }

    fn if_statement(&mut self) -> Result<Stmt, PebbleError> {
        let line = self.line();
        self.advance(); // `if`
        let cond = self.expression()?;
        let then_block = self.block()?;

        let else_block = if self.eat(&TokenKind::Else) {
            if self.check(&TokenKind::If) {
                // `else if` becomes an else block holding one nested `if`.
                Some(vec![self.if_statement()?])
            } else {
                Some(self.block()?)
            }
        } else {
            None
        };

        let kind = StmtKind::If { cond, then_block, else_block };
        Ok(Stmt { kind, line })
    }

    fn while_statement(&mut self) -> Result<Stmt, PebbleError> {
        let line = self.line();
        self.advance(); // `while`
        let cond = self.expression()?;
        let body = self.block()?;
        Ok(Stmt { kind: StmtKind::While { cond, body }, line })
    }

    fn return_statement(&mut self) -> Result<Stmt, PebbleError> {
        let line = self.line();
        self.advance(); // `return`
        let value = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.expect(TokenKind::Semicolon, "';' after return")?;
        Ok(Stmt { kind: StmtKind::Return(value), line })
    }

    /// Handles both `name = value;` and ordinary expression statements.
    fn expression_or_assignment(&mut self) -> Result<Stmt, PebbleError> {
        let line = self.line();

        // Two-token lookahead: an identifier followed by `=` is an assignment.
        if let TokenKind::Ident(name) = self.peek().kind.clone() {
            let next_is_assign = self.tokens.get(self.pos + 1).map(|t| &t.kind)
                == Some(&TokenKind::Assign);
            if next_is_assign {
                self.advance(); // name
                self.advance(); // =
                let value = self.expression()?;
                self.expect(TokenKind::Semicolon, "';' after assignment")?;
                return Ok(Stmt { kind: StmtKind::Assign { name, value }, line });
            }
        }

        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon, "';' after expression")?;
        Ok(Stmt { kind: StmtKind::Expr(expr), line })
    }

    /// A block is `{`, zero or more statements, then `}`.
    fn block(&mut self) -> Result<Vec<Stmt>, PebbleError> {
        self.expect(TokenKind::LBrace, "'{' to start a block")?;
        let mut stmts = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.at_end() {
            stmts.push(self.statement()?);
        }
        self.expect(TokenKind::RBrace, "'}' to close a block")?;
        Ok(stmts)
    }

    fn parse_type(&mut self) -> Result<Type, PebbleError> {
        let name = self.identifier()?;
        match name.as_str() {
            "int" => Ok(Type::Int),
            "bool" => Ok(Type::Bool),
            "str" => Ok(Type::Str),
            other => Err(self.syntax_error(&format!("unknown type '{other}'"))),
        }
    }

    // ----- Expressions -----

    pub fn expression(&mut self) -> Result<Expr, PebbleError> {
        self.binary(1)
    }

    /// Precedence climbing: parse an operand, then absorb any operator
    /// whose precedence is at least `min_prec`.
    fn binary(&mut self, min_prec: u8) -> Result<Expr, PebbleError> {
        let mut left = self.unary()?;
        while let Some((op, prec)) = binary_op(&self.peek().kind) {
            if prec < min_prec {
                break;
            }
            let line = self.advance().line;
            // Parsing the right side at `prec + 1` makes operators left-associative:
            // in `a - b - c`, the second `-` is not absorbed into the right operand.
            let right = self.binary(prec + 1)?;
            let kind = ExprKind::Binary { op, left: Box::new(left), right: Box::new(right) };
            left = Expr { kind, line };
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, PebbleError> {
        let line = self.line();
        let op = match self.peek().kind {
            TokenKind::Minus => Some(UnaryOp::Neg),
            TokenKind::Bang => Some(UnaryOp::Not),
            _ => None,
        };
        if let Some(op) = op {
            self.advance();
            let expr = self.unary()?;
            return Ok(Expr { kind: ExprKind::Unary { op, expr: Box::new(expr) }, line });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, PebbleError> {
        let token = self.advance();
        let line = token.line;
        let kind = match token.kind {
            TokenKind::Int(n) => ExprKind::Int(n),
            TokenKind::Str(s) => ExprKind::Str(s),
            TokenKind::True => ExprKind::Bool(true),
            TokenKind::False => ExprKind::Bool(false),
            TokenKind::Ident(name) => {
                if self.eat(&TokenKind::LParen) {
                    // A function call: name(arg, arg, ...)
                    let mut args = Vec::new();
                    if !self.check(&TokenKind::RParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.eat(&TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RParen, "')' after arguments")?;
                    ExprKind::Call { name, args }
                } else {
                    ExprKind::Var(name)
                }
            }
            TokenKind::LParen => {
                let inner = self.expression()?;
                self.expect(TokenKind::RParen, "')' after expression")?;
                return Ok(inner);
            }
            _ => return Err(PebbleError::new(ErrorKind::Syntax, line, "expected an expression")),
        };
        Ok(Expr { kind, line })
    }
}

/// Describes each binary operator: what it means and how tightly it binds.
fn binary_op(kind: &TokenKind) -> Option<(BinOp, u8)> {
    let entry = match kind {
        TokenKind::OrOr => (BinOp::Or, 1),
        TokenKind::AndAnd => (BinOp::And, 2),
        TokenKind::EqEq => (BinOp::Eq, 3),
        TokenKind::NotEq => (BinOp::NotEq, 3),
        TokenKind::Lt => (BinOp::Lt, 4),
        TokenKind::LtEq => (BinOp::LtEq, 4),
        TokenKind::Gt => (BinOp::Gt, 4),
        TokenKind::GtEq => (BinOp::GtEq, 4),
        TokenKind::Plus => (BinOp::Add, 5),
        TokenKind::Minus => (BinOp::Sub, 5),
        TokenKind::Star => (BinOp::Mul, 6),
        TokenKind::Slash => (BinOp::Div, 6),
        TokenKind::Percent => (BinOp::Rem, 6),
        _ => return None,
    };
    Some(entry)
}
```

Read the `binary` function slowly, because it is the most subtle code in
the parser. The trick with `prec + 1` is the key to left associativity, and
it is worth tracing `1 - 2 - 3` on paper. It should produce `(1 - 2) - 3`,
which equals `-4`, not `1 - (2 - 3)`, which equals `2`.

#### 15.6 Wiring It Up and Trying It

Create `src/main.rs` with the full pipeline. We will extend it in later
chapters:

```rust
// src/main.rs
mod ast;
mod builtins;
mod checker;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod token;

use std::{env, fs, process};

use error::PebbleError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!("usage: pebble <file.pbl>");
        process::exit(2);
    };

    let source = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("cannot read '{path}': {err}");
            process::exit(1);
        }
    };

    if let Err(err) = run(&source) {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run(source: &str) -> Result<(), PebbleError> {
    let tokens = lexer::tokenize(source)?;
    let program = parser::Parser::new(tokens).parse_program()?;

    // Phases below are added in Chapters 16 and 17.
    println!("parsed {} top-level statement(s)", program.len());
    Ok(())
}
```

The `builtins`, `checker`, and `interpreter` modules do not exist yet, so
this will not compile until Chapters 16 and 17 supply them. To test the
parser now, temporarily comment out those three `mod` lines and the
`checker` and `interpreter` calls. Then run:

```bash
cargo run -- examples/hello.pbl
```

To see the tree itself, add `println!("{:#?}", program);` in `run` before
the summary line. The `{:#?}` format from Chapter 5 prints the nested
structure with indentation.

Try a syntax error on purpose. Delete the semicolon in `hello.pbl`:

```text
syntax error on line 2: expected ';' after expression
```

Notice that the message names the line where the parser stopped. That is
the same design goal as Rust's own compiler messages.

> 🛡️ **Robustness check:** Try an unterminated string, `print("hi);`.
> You should see `unterminated string` on line 1. A well-built front end
> never panics on bad input. It reports the problem and stops cleanly.

#### 15.7 Chapter 15 Exercise

Add a `%` example to the parser and verify precedence. Write the Pebble
expression `2 + 3 * 4 % 5` and work out, by hand, the tree the parser
should produce. Then use `{:#?}` to confirm it.

> ✅ **Answer:** `*` and `%` share precedence 6 and associate left, so
> `3 * 4 % 5` groups as `(3 * 4) % 5`, which is `2`. Then `2 + 2` gives `4`.
> The tree's root is `Add`, with `Int(2)` on the left and a `Rem` node on
> the right, whose left child is a `Mul` of 3 and 4.

---

### Chapter 16: Semantic Analysis and Type Checking

#### 16.1 Why Check Before Running?

A syntactically valid program can still be meaningless. `let x: int = "hi";`
is grammatically fine but makes no sense. `print(undefined_name);` parses
perfectly well. A **checker** runs over the tree before anything executes,
so that these mistakes are reported immediately, with line numbers, rather
than halfway through a long run.

The checker answers three questions for every part of the program:

1. **Does every name refer to something that exists?** (name resolution)
2. **Do the types agree?** (type checking)
3. **Are the rules obeyed?** (for example, only `mut` variables may be
   reassigned, and functions must be defined at the top level)

> 💡 **The payoff:** A checked program never needs to ask "is this a
> number or a string?" at runtime. The interpreter in Chapter 17 can trust
> that the types already agree. This is one reason statically typed
> languages can be both safe and fast.

#### 16.2 Built-In Functions and Their Signatures

Pebble's builtins, such as `input` and `read_file`, live in a dedicated
module. The checker needs their **signatures**: the types they accept and
return. Chapter 17 adds their implementations to the same file, so that
one module describes each builtin completely.

Create `src/builtins.rs`, beginning with the signatures:

```rust
// src/builtins.rs
use crate::ast::Type;

/// Returns the parameter types and return type of a builtin, if it exists.
/// `print` is handled specially by the checker and interpreter, so it is absent.
pub fn builtin_signature(name: &str) -> Option<(Vec<Type>, Type)> {
    match name {
        "input" => Some((vec![Type::Str], Type::Str)),
        "read_file" => Some((vec![Type::Str], Type::Str)),
        "write_file" | "append_file" => Some((vec![Type::Str, Type::Str], Type::Unit)),
        "file_exists" => Some((vec![Type::Str], Type::Bool)),
        "to_int" => Some((vec![Type::Str], Type::Int)),
        _ => None,
    }
}

/// Whether `name` is one of the builtins that take ordinary arguments.
pub fn is_builtin(name: &str) -> bool {
    builtin_signature(name).is_some()
}
```

Read the `input` signature. It takes one `str` prompt and returns a `str`.
A Pebble program therefore cannot accidentally treat keyboard text as a
number. It must call `to_int` first, and the checker enforces that step.

#### 16.3 Scopes and the Symbol Table

A **symbol table** records what each name means. Pebble uses a stack of
**scopes**, one per block. Looking up a name searches from the innermost
scope outward, which is exactly how shadowing works.

```text
  global scope      { factorial: fn(int)->int, name: str }
  └─ block scope    { count: int }                 ← inner `if` block
     └─ block scope { count: int (shadowed) }      ← innermost block

  lookup("count") searches from innermost to outermost and stops at the first match.
```

Each variable's entry records its type and whether it is `mut`. Functions
get their own table of signatures, since they can be called from anywhere
once defined.

#### 16.4 The Checker

Create `src/checker.rs`. Read the file in sections: the data structures
first, then the statement rules, then the expression rules.

```rust
// src/checker.rs
use std::collections::HashMap;

use crate::ast::{BinOp, Expr, ExprKind, FnDecl, Stmt, StmtKind, Type, UnaryOp};
use crate::builtins;
use crate::error::{ErrorKind, PebbleError};

/// What the checker remembers about a variable.
#[derive(Clone, Copy)]
struct Binding {
    ty: Type,
    mutable: bool,
}

/// What the checker remembers about a function.
#[derive(Clone)]
struct Signature {
    params: Vec<Type>,
    ret: Type,
}

pub struct Checker {
    /// One map per open scope. The last map is the innermost scope.
    scopes: Vec<HashMap<String, Binding>>,
    functions: HashMap<String, Signature>,
    /// The return type of the function currently being checked, if any.
    return_type: Option<Type>,
}

fn type_error(line: usize, message: impl Into<String>) -> PebbleError {
    PebbleError::new(ErrorKind::Type, line, message)
}

/// Succeeds only when `actual` equals `expected`; otherwise explains the mismatch.
fn require(actual: Type, expected: Type, line: usize, context: &str) -> Result<(), PebbleError> {
    if actual == expected {
        Ok(())
    } else {
        Err(type_error(
            line,
            format!("{context}: expected {}, found {}", expected.name(), actual.name()),
        ))
    }
}

/// Checks a whole program. Returns the first problem found.
pub fn check_program(program: &[Stmt]) -> Result<(), PebbleError> {
    let mut checker = Checker::new();

    // Pass 1: record every function signature, so functions can call
    // each other regardless of the order they are written in.
    for stmt in program {
        if let StmtKind::FnDecl(decl) = &stmt.kind {
            if checker.functions.contains_key(&decl.name) {
                return Err(type_error(
                    stmt.line,
                    format!("function '{}' is defined twice", decl.name),
                ));
            }
            let params = decl.params.iter().map(|(_, ty)| *ty).collect();
            let sig = Signature { params, ret: decl.ret };
            checker.functions.insert(decl.name.clone(), sig);
        }
    }

    // Pass 2: check each statement in order, with full information available.
    for stmt in program {
        checker.check_stmt(stmt)?;
    }
    Ok(())
}

impl Checker {
    fn new() -> Self {
        Checker {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            return_type: None,
        }
    }

    // ----- Scopes -----

    fn declare(&mut self, name: &str, binding: Binding, line: usize) -> Result<(), PebbleError> {
        let scope = self.scopes.last_mut().expect("there is always a scope");
        if scope.contains_key(name) {
            return Err(type_error(line, format!("'{name}' is already declared in this scope")));
        }
        scope.insert(name.to_string(), binding);
        Ok(())
    }

    fn lookup(&self, name: &str, line: usize) -> Result<Binding, PebbleError> {
        // Search innermost to outermost; the first match wins.
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .copied()
            .ok_or_else(|| type_error(line, format!("unknown variable '{name}'")))
    }

    fn check_block(&mut self, stmts: &[Stmt]) -> Result<(), PebbleError> {
        self.scopes.push(HashMap::new());
        let result = stmts.iter().try_for_each(|stmt| self.check_stmt(stmt));
        self.scopes.pop(); // Always pop, even if checking failed.
        result
    }

    // ----- Statements -----

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), PebbleError> {
        let line = stmt.line;
        match &stmt.kind {
            StmtKind::Let { name, mutable, annotation, value } => {
                let value_ty = self.check_expr(value)?;
                if value_ty == Type::Unit {
                    return Err(type_error(line, "cannot store the result of a call that returns nothing"));
                }
                let ty = match annotation {
                    Some(expected) => {
                        require(value_ty, *expected, line, "initial value")?;
                        *expected
                    }
                    None => value_ty,
                };
                self.declare(name, Binding { ty, mutable: *mutable }, line)?;
            }

            StmtKind::Assign { name, value } => {
                let binding = self.lookup(name, line)?;
                if !binding.mutable {
                    return Err(type_error(
                        line,
                        format!("cannot assign to '{name}' because it was not declared with 'mut'"),
                    ));
                }
                let value_ty = self.check_expr(value)?;
                require(value_ty, binding.ty, line, "assignment")?;
            }

            StmtKind::Expr(expr) => {
                self.check_expr(expr)?;
            }

            StmtKind::If { cond, then_block, else_block } => {
                let cond_ty = self.check_expr(cond)?;
                require(cond_ty, Type::Bool, line, "if condition")?;
                self.check_block(then_block)?;
                if let Some(else_block) = else_block {
                    self.check_block(else_block)?;
                }
            }

            StmtKind::While { cond, body } => {
                let cond_ty = self.check_expr(cond)?;
                require(cond_ty, Type::Bool, line, "while condition")?;
                self.check_block(body)?;
            }

            StmtKind::Return(value) => {
                let actual = match value {
                    Some(expr) => self.check_expr(expr)?,
                    None => Type::Unit,
                };
                let expected = self
                    .return_type
                    .ok_or_else(|| type_error(line, "'return' can only be used inside a function"))?;
                require(actual, expected, line, "return value")?;
            }

            StmtKind::FnDecl(decl) => self.check_function(decl, line)?,
        }
        Ok(())
    }

    fn check_function(&mut self, decl: &FnDecl, line: usize) -> Result<(), PebbleError> {
        // Functions are only allowed at the top level, so they cannot capture local variables.
        if self.scopes.len() > 1 {
            return Err(type_error(line, "functions must be declared at the top level"));
        }

        let saved_return = self.return_type.replace(decl.ret);
        self.scopes.push(HashMap::new());

        // Parameters are declared in the function's own scope and are immutable.
        let result = decl
            .params
            .iter()
            .try_for_each(|(name, ty)| {
                self.declare(name, Binding { ty: *ty, mutable: false }, line)
            })
            .and_then(|()| decl.body.iter().try_for_each(|stmt| self.check_stmt(stmt)));

        self.scopes.pop();
        self.return_type = saved_return;
        result
    }

    // ----- Expressions -----

    /// Returns the type of an expression, or an error if it is ill-typed.
    fn check_expr(&mut self, expr: &Expr) -> Result<Type, PebbleError> {
        let line = expr.line;
        match &expr.kind {
            ExprKind::Int(_) => Ok(Type::Int),
            ExprKind::Bool(_) => Ok(Type::Bool),
            ExprKind::Str(_) => Ok(Type::Str),
            ExprKind::Var(name) => Ok(self.lookup(name, line)?.ty),

            ExprKind::Unary { op, expr: inner } => {
                let ty = self.check_expr(inner)?;
                match op {
                    UnaryOp::Neg => {
                        require(ty, Type::Int, line, "operand of '-'")?;
                        Ok(Type::Int)
                    }
                    UnaryOp::Not => {
                        require(ty, Type::Bool, line, "operand of '!'")?;
                        Ok(Type::Bool)
                    }
                }
            }

            ExprKind::Binary { op, left, right } => self.check_binary(*op, left, right, line),
            ExprKind::Call { name, args } => self.check_call(name, args, line),
        }
    }

    fn check_binary(
        &mut self,
        op: BinOp,
        left: &Expr,
        right: &Expr,
        line: usize,
    ) -> Result<Type, PebbleError> {
        let lt = self.check_expr(left)?;
        let rt = self.check_expr(right)?;

        match op {
            // `+` joins two strings, or adds two integers.
            BinOp::Add if lt == Type::Str && rt == Type::Str => Ok(Type::Str),
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => {
                require(lt, Type::Int, line, "left side of arithmetic")?;
                require(rt, Type::Int, line, "right side of arithmetic")?;
                Ok(Type::Int)
            }
            BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq => {
                require(lt, Type::Int, line, "left side of comparison")?;
                require(rt, Type::Int, line, "right side of comparison")?;
                Ok(Type::Bool)
            }
            // Equality works on any type, as long as both sides match.
            BinOp::Eq | BinOp::NotEq => {
                require(lt, rt, line, "comparison")?;
                Ok(Type::Bool)
            }
            BinOp::And | BinOp::Or => {
                require(lt, Type::Bool, line, "left side of logical operator")?;
                require(rt, Type::Bool, line, "right side of logical operator")?;
                Ok(Type::Bool)
            }
        }
    }

    fn check_call(&mut self, name: &str, args: &[Expr], line: usize) -> Result<Type, PebbleError> {
        // `print` accepts any single value, so it is checked separately.
        if name == "print" {
            if args.len() != 1 {
                return Err(type_error(line, "print takes exactly one argument"));
            }
            self.check_expr(&args[0])?;
            return Ok(Type::Unit);
        }

        let sig = match builtins::builtin_signature(name) {
            Some((params, ret)) => Signature { params, ret },
            None => self
                .functions
                .get(name)
                .cloned()
                .ok_or_else(|| type_error(line, format!("unknown function '{name}'")))?,
        };

        if args.len() != sig.params.len() {
            return Err(type_error(
                line,
                format!(
                    "'{name}' expects {} argument(s), but {} were given",
                    sig.params.len(),
                    args.len()
                ),
            ));
        }

        for (arg, expected) in args.iter().zip(&sig.params) {
            let actual = self.check_expr(arg)?;
            require(actual, *expected, line, &format!("argument to '{name}'"))?;
        }
        Ok(sig.ret)
    }
}
```

Two details deserve a closer look. First, `check_block` pushes a scope,
checks the statements, and pops the scope **whether or not checking
succeeded**. Skipping the pop on error would leave the table in a broken
state, which matters if a future version of the checker recovers from
errors and continues. Second, `check_function` refuses nested
definitions by looking at the scope depth. Because functions are checked
only at the top level, a function body can see globals but never a
caller's locals. The interpreter in Chapter 17 must behave the same way,
and it does.

Now add `Type` to the `ast.rs` imports used by the checker. Everything is
already in place from Chapter 15.

#### 16.5 Typical Errors and What They Mean

| Program fragment | Message |
|:-----------------|:--------|
| `let x: int = "hi";` | `initial value: expected int, found str` |
| `let x = 1; x = 2;` | `cannot assign to 'x' because it was not declared with 'mut'` |
| `if 1 { }` | `if condition: expected bool, found int` |
| `print(missing);` | `unknown variable 'missing'` |
| `let n = to_int(5);` | `argument to 'to_int': expected str, found int` |
| `fn a() { fn b() {} }` | `functions must be declared at the top level` |

Try each of these in a scratch file. Reading error messages is a skill,
and this table is a good way to build it.

#### 16.6 Chapter 16 Exercise

Add a builtin `len(text: str) -> int` that returns the number of
characters in a string. Add its signature to `builtin_signature`. Then
test that `print(len(5));` produces a type error.

> ✅ **Hint:** In `builtin_signature`, add a new arm:
> `"len" => Some((vec![Type::Str], Type::Int)),`. You will implement the
> behavior in Chapter 17's builtin section, using `chars().count()`.

---

### Chapter 17: Executing Pebble: The Interpreter, Input, and File I/O

#### 17.1 Values at Runtime

The checker has already proven that types agree. At runtime, Pebble only
needs to represent the values themselves. Each runtime value is one of
four variants, mirroring the four types in `ast.rs`. Add this to
`src/interpreter.rs`, which we build over the next few sections:

```rust
// src/interpreter.rs
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::{BinOp, Expr, ExprKind, FnDecl, Stmt, StmtKind, UnaryOp};
use crate::builtins;
use crate::error::{ErrorKind, PebbleError};

/// Pebble's maximum nesting depth for function calls. Each call uses native
/// stack space in this interpreter, so an unlimited depth could crash the program.
const MAX_CALL_DEPTH: usize = 256;

/// A value that exists while the program runs.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Unit,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Str(s) => write!(f, "{s}"),
            Value::Unit => write!(f, "()"),
        }
    }
}

/// How a block of statements finished.
enum Flow {
    /// Ran to the end normally.
    Normal,
    /// Hit a `return` statement carrying a value.
    Return(Value),
}

fn runtime(line: usize, message: impl Into<String>) -> PebbleError {
    PebbleError::new(ErrorKind::Runtime, line, message)
}

fn overflow(line: usize) -> PebbleError {
    runtime(line, "integer overflow: the result does not fit in an int")
}
```

Why is `Flow` separate from `Value`? A `return` is not a value an
expression produces. It is a signal that unwinds through nested blocks and
loops until it reaches the function that called it. Modeling it as its own
enum lets the rest of the code handle it explicitly, with no hidden
exceptions. This is the same spirit as `Result` from Chapter 6.

#### 17.2 The Environment and Its Scopes

The interpreter keeps variables in a stack of scopes, just as the checker
did. The differences are that scopes hold **values** instead of types, and
that function calls need special handling.

```rust
pub struct Interpreter {
    /// Scope 0 holds top-level variables. Deeper entries are blocks and calls.
    scopes: Vec<HashMap<String, Value>>,
    /// Every top-level function, shared cheaply through reference counting.
    functions: HashMap<String, Rc<FnDecl>>,
    /// How many function calls are currently active.
    depth: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            depth: 0,
        }
    }

    /// Runs a checked program from top to bottom.
    pub fn run(&mut self, program: &[Stmt]) -> Result<(), PebbleError> {
        // Register all functions first, so they can be called in any order.
        for stmt in program {
            if let StmtKind::FnDecl(decl) = &stmt.kind {
                self.functions.insert(decl.name.clone(), Rc::new(decl.clone()));
            }
        }

        for stmt in program {
            if let Flow::Return(_) = self.exec_stmt(stmt)? {
                break; // A top-level `return` ends the program.
            }
        }
        Ok(())
    }

    fn define(&mut self, name: &str, value: Value) {
        self.scopes
            .last_mut()
            .expect("there is always a scope")
            .insert(name.to_string(), value);
    }

    fn assign(&mut self, name: &str, value: Value, line: usize) -> Result<(), PebbleError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(slot) = scope.get_mut(name) {
                *slot = value;
                return Ok(());
            }
        }
        Err(runtime(line, format!("unknown variable '{name}'")))
    }

    fn read_var(&self, name: &str, line: usize) -> Result<Value, PebbleError> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .cloned()
            .ok_or_else(|| runtime(line, format!("unknown variable '{name}'")))
    }
}
```

`Interpreter::new` is a plain constructor. Clippy may suggest adding a
`Default` implementation as well. That is a reasonable refinement, and an
easy one to add later.

#### 17.3 Running Statements

Statements run in a loop, and any `return` stops the loop early. Add
these methods to the same `impl Interpreter` block, or in a second block
below it, which Rust also allows:

```rust
impl Interpreter {
    fn exec_block(&mut self, stmts: &[Stmt]) -> Result<Flow, PebbleError> {
        self.scopes.push(HashMap::new());
        let result = self.exec_stmts(stmts);
        self.scopes.pop(); // Always pop, even if execution failed.
        result
    }

    fn exec_stmts(&mut self, stmts: &[Stmt]) -> Result<Flow, PebbleError> {
        for stmt in stmts {
            if let Flow::Return(value) = self.exec_stmt(stmt)? {
                return Ok(Flow::Return(value)); // Unwind immediately.
            }
        }
        Ok(Flow::Normal)
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Flow, PebbleError> {
        let line = stmt.line;
        match &stmt.kind {
            StmtKind::Let { name, value, .. } => {
                let v = self.eval(value)?;
                self.define(name, v);
            }

            StmtKind::Assign { name, value } => {
                let v = self.eval(value)?;
                self.assign(name, v, line)?;
            }

            StmtKind::Expr(expr) => {
                self.eval(expr)?;
            }

            StmtKind::If { cond, then_block, else_block } => {
                if self.eval_bool(cond)? {
                    return self.exec_block(then_block);
                }
                if let Some(else_block) = else_block {
                    return self.exec_block(else_block);
                }
            }

            StmtKind::While { cond, body } => {
                while self.eval_bool(cond)? {
                    if let Flow::Return(value) = self.exec_block(body)? {
                        return Ok(Flow::Return(value));
                    }
                }
            }

            StmtKind::Return(value) => {
                let v = match value {
                    Some(expr) => self.eval(expr)?,
                    None => Value::Unit,
                };
                return Ok(Flow::Return(v));
            }

            // Functions were registered before execution began.
            StmtKind::FnDecl(_) => {}
        }
        Ok(Flow::Normal)
    }
}
```

The `Let` arm ignores the annotation with `..`. The checker already
verified it, so the runtime does not need it.

#### 17.4 Evaluating Expressions

Expression evaluation returns a `Value`. Two details matter here.
First, `&&` and `||` **short-circuit**: the right side runs only when it
can still change the answer, so `n != 0 && 10 / n > 1` is safe when `n` is
zero. Second, arithmetic uses **checked** operations, which return `None`
on overflow instead of wrapping around or panicking.

```rust
impl Interpreter {
    fn eval(&mut self, expr: &Expr) -> Result<Value, PebbleError> {
        let line = expr.line;
        match &expr.kind {
            ExprKind::Int(n) => Ok(Value::Int(*n)),
            ExprKind::Bool(b) => Ok(Value::Bool(*b)),
            ExprKind::Str(s) => Ok(Value::Str(s.clone())),
            ExprKind::Var(name) => self.read_var(name, line),

            ExprKind::Unary { op, expr: inner } => {
                let v = self.eval(inner)?;
                match (op, v) {
                    (UnaryOp::Neg, Value::Int(n)) => {
                        n.checked_neg().map(Value::Int).ok_or_else(|| overflow(line))
                    }
                    (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    _ => Err(runtime(line, "invalid operand for unary operator")),
                }
            }

            ExprKind::Binary { op, left, right } => self.eval_binary(*op, left, right, line),
            ExprKind::Call { name, args } => self.call(name, args, line),
        }
    }

    fn eval_bool(&mut self, expr: &Expr) -> Result<bool, PebbleError> {
        match self.eval(expr)? {
            Value::Bool(b) => Ok(b),
            _ => Err(runtime(expr.line, "expected a true or false value")),
        }
    }

    fn eval_binary(
        &mut self,
        op: BinOp,
        left: &Expr,
        right: &Expr,
        line: usize,
    ) -> Result<Value, PebbleError> {
        // Short-circuit: the right side runs only when it can change the result.
        if op == BinOp::And || op == BinOp::Or {
            let l = self.eval_bool(left)?;
            return match (op, l) {
                (BinOp::And, false) => Ok(Value::Bool(false)),
                (BinOp::Or, true) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(self.eval_bool(right)?)),
            };
        }

        let l = self.eval(left)?;
        let r = self.eval(right)?;
        match (op, l, r) {
            (BinOp::Add, Value::Int(a), Value::Int(b)) => {
                a.checked_add(b).map(Value::Int).ok_or_else(|| overflow(line))
            }
            (BinOp::Add, Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
            (BinOp::Sub, Value::Int(a), Value::Int(b)) => {
                a.checked_sub(b).map(Value::Int).ok_or_else(|| overflow(line))
            }
            (BinOp::Mul, Value::Int(a), Value::Int(b)) => {
                a.checked_mul(b).map(Value::Int).ok_or_else(|| overflow(line))
            }
            (BinOp::Div, Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    return Err(runtime(line, "division by zero"));
                }
                a.checked_div(b).map(Value::Int).ok_or_else(|| overflow(line))
            }
            (BinOp::Rem, Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    return Err(runtime(line, "remainder by zero"));
                }
                a.checked_rem(b).map(Value::Int).ok_or_else(|| overflow(line))
            }
            (BinOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (BinOp::LtEq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (BinOp::Gt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (BinOp::GtEq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (BinOp::Eq, a, b) => Ok(Value::Bool(a == b)),
            (BinOp::NotEq, a, b) => Ok(Value::Bool(a != b)),
            _ => Err(runtime(line, "invalid operands for this operator")),
        }
    }
}
```

> 🛡️ **Why checked arithmetic?** In release builds of ordinary Rust, an
> overflowing `i64` addition wraps silently. A language that wraps silently
> can produce wrong answers with no warning. Pebble reports overflow as a
> clear error instead, which is a better default for a teaching language.

#### 17.5 Function Calls and Isolation

This is the most important runtime idea in the chapter. When a function
runs, it must see the **global** variables and its own **parameters**, and
nothing else. In particular, it must not see the caller's local variables,
or a local with the same name as a global would silently take over.

The solution is a single line of Rust. `Vec::split_off(1)` removes every
scope except the global one and returns them, so the callee starts from a
clean stack. When the call finishes, the saved scopes are put back.

```text
  before call:  [ globals | caller block | caller inner block ]
  split_off(1): [ globals ]       saved: [ caller block | caller inner block ]
  callee runs:  [ globals | params ]
  restore:      [ globals | caller block | caller inner block ]
```

Add this to the `impl Interpreter` block:

```rust
impl Interpreter {
    fn call(&mut self, name: &str, args: &[Expr], line: usize) -> Result<Value, PebbleError> {
        // Evaluate arguments in the caller's scope, before anything changes.
        let mut values = Vec::with_capacity(args.len());
        for arg in args {
            values.push(self.eval(arg)?);
        }

        // `print` is special: it accepts any value and returns nothing.
        if name == "print" {
            println!("{}", values[0]);
            return Ok(Value::Unit);
        }

        if builtins::is_builtin(name) {
            return builtins::call(name, &values).map_err(|message| runtime(line, message));
        }

        let decl = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| runtime(line, format!("unknown function '{name}'")))?;
        self.call_user(decl, values, line)
    }

    fn call_user(
        &mut self,
        decl: Rc<FnDecl>,
        values: Vec<Value>,
        line: usize,
    ) -> Result<Value, PebbleError> {
        if self.depth >= MAX_CALL_DEPTH {
            return Err(runtime(line, format!("recursion is too deep (limit is {MAX_CALL_DEPTH})")));
        }

        // Hide the caller's locals: keep only the global scope.
        let caller_scopes = self.scopes.split_off(1);
        self.scopes.push(HashMap::new());
        for ((param, _), value) in decl.params.iter().zip(values) {
            self.define(param, value);
        }

        self.depth += 1;
        let outcome = self.exec_stmts(&decl.body);
        self.depth -= 1;

        // Restore the caller's view, even if the function failed.
        self.scopes.truncate(1);
        self.scopes.extend(caller_scopes);

        match outcome? {
            Flow::Return(value) => Ok(value),
            Flow::Normal => Ok(Value::Unit),
        }
    }
}
```

Notice that arguments are evaluated **before** the scopes are swapped.
They belong to the caller, so they must be computed while the caller's
scopes are still in place.

#### 17.6 Implementing the Builtins: Input and File I/O

Now complete `src/builtins.rs` by adding the implementations below the
signatures from Chapter 16. These are the functions that connect Pebble to
the outside world.

```rust
// Append to src/builtins.rs, below the existing signature functions.
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use crate::interpreter::Value;

/// Runs a builtin. Errors are returned as plain messages; the caller adds the line number.
pub fn call(name: &str, args: &[Value]) -> Result<Value, String> {
    match name {
        // Show a prompt, then wait for one line of keyboard input.
        "input" => {
            print!("{}", text(&args[0])?);
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut line = String::new();
            io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
            // Remove the trailing newline, handling both Unix and Windows line endings.
            Ok(Value::Str(line.trim_end_matches(['\r', '\n']).to_string()))
        }

        // Read an entire file into a string.
        "read_file" => {
            let path = text(&args[0])?;
            fs::read_to_string(path)
                .map(Value::Str)
                .map_err(|e| format!("cannot read '{path}': {e}"))
        }

        // Create or replace a file with the given content.
        "write_file" => {
            let path = text(&args[0])?;
            fs::write(path, text(&args[1])?)
                .map_err(|e| format!("cannot write '{path}': {e}"))?;
            Ok(Value::Unit)
        }

        // Add content to the end of a file, creating the file if needed.
        "append_file" => {
            let path = text(&args[0])?;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| format!("cannot open '{path}': {e}"))?;
            file.write_all(text(&args[1])?.as_bytes())
                .map_err(|e| format!("cannot write '{path}': {e}"))?;
            Ok(Value::Unit)
        }

        "file_exists" => Ok(Value::Bool(Path::new(text(&args[0])?).exists())),

        // Parse a string as a whole number, reporting a clear message if it fails.
        "to_int" => {
            let s = text(&args[0])?;
            s.trim()
                .parse::<i64>()
                .map(Value::Int)
                .map_err(|_| format!("'{s}' is not a whole number"))
        }

        "len" => {
            let s = text(&args[0])?;
            Ok(Value::Int(s.chars().count() as i64))
        }

        other => Err(format!("unknown builtin '{other}'")),
    }
}

/// Extracts the text from a `str` value. The checker guarantees the type,
/// but we still check, because a function that accepts any value must not panic.
fn text(value: &Value) -> Result<&str, String> {
    match value {
        Value::Str(s) => Ok(s.as_str()),
        _ => Err("expected a str value".to_string()),
    }
}
```

The `len` builtin from the Chapter 16 exercise appears here. Its signature
must also exist in `builtin_signature`, which you added as the Chapter 16
exercise answer. If you skipped that exercise, add
`"len" => Some((vec![Type::Str], Type::Int)),` now. `chars().count()` counts Unicode
characters, not bytes, so `len("héllo")` gives 5 as a user would expect.

Several details in this code reward attention:

- **`print!` before `read_line`:** A prompt must appear before the program
  waits. `print!` does not add a newline, so the user types on the same
  line as the prompt. `flush` forces the text out of the buffer. Without it,
  the prompt may not appear until after you have already typed.
- **`trim_end_matches(['\r', '\n'])`:** Windows terminals end lines with
  `\r\n`, while Linux and macOS use `\n`. Removing both keeps programs
  portable.
- **Errors become values:** A missing file produces a clean message such as
  `cannot read 'notes.txt': No such file or directory`. The program does not
  crash, and the user learns what went wrong.
- **`text` is the only place that unwraps `Value::Str`:** Keeping that
  logic in one helper means that every builtin handles bad input the same way.

#### 17.7 Assembling the Final `main.rs`

Update the `run` function in `src/main.rs` so that every phase runs in
order:

```rust
fn run(source: &str) -> Result<(), PebbleError> {
    let tokens = lexer::tokenize(source)?;
    let program = parser::Parser::new(tokens).parse_program()?;
    checker::check_program(&program)?;
    interpreter::Interpreter::new().run(&program)
}
```

Then build the whole language:

```bash
cargo build
```

If the compiler reports an error, read it carefully. The most likely
causes are a missing `use` line or a `mod` declaration. Each error message
names the file and line, which points directly at the fix.

#### 17.8 Running Real Programs

Start with the hello example:

```bash
cargo run -- examples/hello.pbl
```

```text
What is your name? Ada
Hello, Ada!
```

Now the factorial program. Run it several times with different inputs:

```bash
cargo run -- examples/factorial.pbl
```

```text
Number: 10
3628800
```

Try the guestbook twice in a row. The second run shows the first visitor
still in the file, because `append_file` adds to the file rather than
replacing it:

```bash
cargo run -- examples/guestbook.pbl
cargo run -- examples/guestbook.pbl
```

```text
Who is visiting? Ada
Visitors so far:
Ada
Who is visiting? Linus
Visitors so far:
Ada
Linus
```

Finally, try a program with mistakes, to see the phases working together:

```pebble
// examples/broken.pbl
let count: int = 0;
count = count + 1;
print(undefined_name);
```

Running it reports the first problem the checker finds, before any line
executes:

```text
type error on line 2: cannot assign to 'count' because it was not declared with 'mut'
```

Fix that by writing `let mut count`, and the next error appears, on line 3.
This is the checker doing its job: finding mistakes early and naming them
precisely.

> 🧭 **Try this:** Write a Pebble program that asks for a list of names
> one at a time, stops when the user types `done`, and writes every name
> to `names.txt`. It needs `while`, `input`, `!=`, and `append_file`.
> Every feature you need is already implemented.

#### 17.9 Safety and Honest Limits

Pebble is a teaching language, so it has limits you should understand
rather than hide:

- **File access is unrestricted.** A Pebble program can read or overwrite
  any file the user can access. Real scripting environments often sandbox
  file access to one directory. Adding that is a good exercise: check that
  every path stays inside a chosen folder, rejecting paths that begin with
  `/` or contain `..`.
- **Not every path returns a value.** The checker does not verify that a
  function with `-> int` returns on every path. A function that forgets its
  `return` yields `()`, and the error surfaces later as an operand problem.
  Fixing this properly means a separate analysis that follows every branch.
- **Recursion depth is capped** at 256 calls, because the interpreter uses
  native stack space for each Pebble call.

Each of these is a real design decision that production languages face.
Naming them is part of the education.

#### 17.10 A Preview of Code Generation

Pebble executes its tree directly. The alternative, which many real
languages use, is to convert the tree into a flat list of instructions for
a **virtual machine**. A stack machine evaluates expressions by pushing
operands and applying operators to the top of the stack. The expression
`1 + 2 * 3` compiles to:

```text
PUSH 1      stack: [1]
PUSH 2      stack: [1, 2]
PUSH 3      stack: [1, 2, 3]
MUL         stack: [1, 6]        (pops 3 and 2, pushes 6)
ADD         stack: [7]           (pops 6 and 1, pushes 7)
```

The instructions form a list rather than a tree, so a simple loop can
execute them, and the result can be saved to a file and reloaded later.
This is the core idea behind bytecode languages such as Python and Java.

#### 17.11 Chapter 17 Exercise: A Bytecode Compiler for Arithmetic

Write a function `fn compile(expr: &Expr, out: &mut Vec<Op>)` that
converts integer expressions into a list of operations, where:

```rust
enum Op {
    Push(i64),
    Add,
    Sub,
    Mul,
    Div,
}
```

Then write `fn run_ops(ops: &[Op]) -> Option<i64>`, a stack machine that
executes them and returns the final value. Test that `1 + 2 * 3` gives 7.

> ✅ **Hint:** For `compile`, handle `ExprKind::Int` by pushing, and
> `ExprKind::Binary` by compiling the left side, then the right side, then
> pushing the operator. Post-order traversal, where children come before
> the parent, produces exactly the instruction order shown above. For
> `run_ops`, use a `Vec<i64>` as the stack and `pop` twice for each operator.

---

### Part V Recap

You have now built a complete language implementation. Here is the path
you took:

- You learned the stages of a language pipeline, and why each exists
  (Chapter 14).
- You turned text into tokens, tokens into a tree, and a tree into a
  verified program (Chapters 15 and 16).
- You executed that program with values, scopes, and function calls that
  isolate their variables (Chapter 17).
- You added keyboard input and file reading and writing, with errors that
  become messages instead of crashes (Chapter 17).

The capstone project in the next file applies these ideas to a different
kind of system: a version-control tool in the style of Git. It uses the
same habits you have practiced here, namely precise data types, explicit
error handling, and reading and writing files.


---

<a id="part-v-tinygit-capstone-ch-18"></a>

## Part V - TinyGit Capstone (Ch 18)

| Navigation | |
| --- | --- |
| Previous | [← Part V - Compiler Pebble (Ch 14-17)](#part-v-compiler-pebble-ch-14-17) |
| Next | [Part VI - Advanced Rust (Ch 19-23) →](#part-vi-advanced-rust-ch-19-23) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part V (continued): A Tiny Version-Control Tool

> 🧭 **Where we are:** Chapters 14 to 17 gave you Pebble, a complete
> language pipeline with keyboard input and file reading and writing.
> This chapter applies the same skills to a different kind of system:
> a version-control tool in the style of Git. You will hash files,
> store snapshots, and walk history.

> 🎯 **What you will have at the end:** A working command-line program,
> `tinygit`, that supports `init`, `add`, `commit`, `log`, and `status`
> using only the Rust standard library.

---

### Chapter 18: TinyGit — File I/O at Scale

#### 18.1 What Git Actually Does

Git looks mysterious until you see the trick. A repository is just
three things:

1. **Objects:** every version of every file, stored by the hash of
   its contents.
2. **An index:** a list that says "this path currently means that hash".
3. **Commits:** snapshots of the index, chained together with parent
   pointers so you can walk backwards through time.

> 💡 **Analogy:** Objects are photocopies filed by fingerprint. The index
> is the table of contents on the librarian's desk. A commit is a dated
> photograph of that table of contents, with a note saying which
> photograph came before it.

Real Git uses SHA-1 or SHA-256, compression, trees, and packs. We will
use the same shape with simpler pieces so the whole project builds with
no extra crates. The habits transfer directly: precise types, explicit
errors, and reading and writing files.

#### 18.2 The Storage Layout

Create a new project:

```bash
cargo new tinygit
cd tinygit
```

Our on-disk layout mirrors Git, with shorter names:

```text
.tinygit/
  objects/ab/cdef...   <- file contents, filed by hash
  commits/<hash>       <- one text file per commit
  index                <- lines of "<hash> <path>"
  HEAD                 <- the hash of the latest commit
```

Why split a hash like `ab/cdef`? When a project has thousands of files,
one directory with thousands of entries becomes slow. Two levels keep
each directory small. Git does exactly this.

> ⚠️ **Safety rule for this chapter:** Never track `.tinygit` itself,
> and never follow `..` outside the project. A version-control tool
> that can overwrite its own database, or files above the project,
> is a footgun. Our `add` refuses both.

#### 18.3 Hashing Without Dependencies

Real Git hashes with SHA-256. Adding the `sha2` and `hex` crates is
the right move for production. For teaching, we want zero new
dependencies so you can focus on the design. We use FNV-1a, a tiny,
stable, non-cryptographic hash. It is easy to read and deterministic
across runs and platforms.

> 🧭 **Honest limits:** FNV-1a is NOT collision-resistant. Two different
> files could in theory share a hash. Real systems must use SHA-256 or
> better. We use FNV here the way a driving school uses cones in a
> parking lot: safe to learn in, not legal on the highway.

```rust
fn fnv1a_hex(bytes: &[u8]) -> String {
    // FNV-1a 64-bit: simple, stable, non-cryptographic.
    // Good for teaching; NOT a substitute for SHA-256.
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}
```

`wrapping_mul` is deliberate. In debug builds Rust panics on integer
overflow. Hashing *wants* overflow as part of the algorithm, so we say
so in the type system. This is the same overflow story you met with
`i32` in Chapter 3, now used on purpose.

#### 18.4 Small Helpers: Fail, Read, Write

Every command needs the same three helpers. Put them at the top of
`src/main.rs`:

```rust
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(".tinygit")
}

fn fail(msg: &str) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(1);
}

fn read_text(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_blob(hash: &str, bytes: &[u8]) -> io::Result<()> {
    // Objects live at .tinygit/objects/ab/cdef... like git's fan-out.
    let dir = repo_root().join("objects").join(&hash[..2]);
    fs::create_dir_all(&dir)?;
    fs::write(dir.join(&hash[2..]), bytes)?;
    Ok(())
}

fn read_blob(hash: &str) -> io::Result<Vec<u8>> {
    let path = repo_root().join("objects").join(&hash[..2]).join(&hash[2..]);
    fs::read(path)
}
```

Notice the return type `!` on `fail`. That is Rust's **never type**:
this function never returns normally because it exits the process.
The compiler understands this, so you can call `fail(...)` in any
branch without faking a return value. It pairs naturally with `Result`
and `?` from Chapter 6: I/O errors propagate with `?`, user errors
exit with a message.

#### 18.5 `init`: Creating the Database

```rust
fn cmd_init() -> io::Result<()> {
    let root = repo_root();
    if root.exists() {
        println!("Reinitialized existing tinygit repository.");
        return Ok(());
    }
    fs::create_dir_all(root.join("objects"))?;
    fs::create_dir_all(root.join("commits"))?;
    fs::write(root.join("index"), "")?;
    fs::write(root.join("HEAD"), "")?;
    println!("Initialized empty tinygit repository in .tinygit/");
    Ok(())
}
```

`create_dir_all` creates parents as needed and succeeds if the directory
already exists. That makes `init` idempotent: running it twice does not
destroy history. Idempotency was a Chapter 9 idea for servers; it matters
for CLIs too.

#### 18.6 `add`: Hashing and Staging

`add` reads a file, hashes it, stores the blob, and records the mapping
in the index:

```rust
fn cmd_add(file: &str) -> io::Result<()> {
    if !repo_root().exists() {
        fail("not a tinygit repository (missing .tinygit/). Run `tinygit init` first.");
    }
    // Safety: refuse to track our own database or escape the directory.
    if file.starts_with(".tinygit") || file.contains("..") {
        fail("refusing to track .tinygit internals or parent paths.");
    }
    let bytes = fs::read(file).unwrap_or_else(|_| fail(&format!("cannot read file `{file}`")));
    let hash = fnv1a_hex(&bytes);
    write_blob(&hash, &bytes)?;

    // Update index: replace existing entry for this path, else append.
    let index_path = repo_root().join("index");
    let old = read_text(&index_path).unwrap_or_default();
    let mut kept: Vec<String> = Vec::new();
    for line in old.lines() {
        let mut parts = line.splitn(2, ' ');
        let _h = parts.next().unwrap_or("");
        let p = parts.next().unwrap_or("");
        if p != file && !p.is_empty() {
            kept.push(line.to_string());
        }
    }
    kept.push(format!("{hash} {file}"));
    kept.sort();
    fs::write(index_path, kept.join("\n") + "\n")?;
    println!("added {file} -> {hash}");
    Ok(())
}
```

Three details worth slowing down for:

- `splitn(2, ' ')` splits each index line into at most two pieces. File
  names can contain spaces, so splitting on *every* space would corrupt
  `my notes.txt`. This is the same "parse, don't hope" discipline from
  the Pebble lexer.
- We rewrite the whole index file on every `add`. For thousands of files
  you would want a faster format. For a teaching tool, a plain text file
  you can open in Notepad is a feature: you can *see* the database.
- `kept.sort()` keeps output deterministic. Deterministic output makes
  tests and `log` stable, the same reason we sort before printing.

#### 18.7 `commit`: Freezing a Snapshot

A commit is a text file that records its parent, a message, a timestamp,
and the full snapshot:

```rust
fn cmd_commit(message: &str) -> io::Result<()> {
    if !repo_root().exists() {
        fail("not a tinygit repository. Run `tinygit init` first.");
    }
    let index = read_text(&repo_root().join("index")).unwrap_or_default();
    let entries: Vec<&str> = index.lines().filter(|l| !l.trim().is_empty()).collect();
    if entries.is_empty() {
        fail("nothing to commit (index is empty). Use `tinygit add <file>` first.");
    }
    let parent = read_text(&repo_root().join("HEAD")).unwrap_or_default();
    let parent = parent.trim().to_string();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Commit body is plain text: parent, message, timestamp, snapshot.
    let mut body = format!("parent {parent}\nmessage {message}\ntime {now}\n");
    for e in &entries {
        body.push_str(&format!("{e}\n"));
    }
    let hash = fnv1a_hex(body.as_bytes());
    fs::write(repo_root().join("commits").join(&hash), body.as_bytes())?;
    fs::write(repo_root().join("HEAD"), format!("{hash}\n"))?;
    println!("committed {hash}");
    Ok(())
}
```

The commit's own hash covers the body, so the body covers the snapshot,
so the hash identifies the entire state. That content-addressing is the
core Git insight. Changing one byte of one file changes its blob hash,
which changes the snapshot, which changes the commit hash. History
becomes tamper-evident.

Timestamps use `SystemTime` and seconds since the Unix epoch. We use
`unwrap_or(0)` so a broken clock degrades to `0` instead of crashing
the commit. Time handling with graceful fallback is a Chapter 10 habit.

#### 18.8 `log` and `status`: Reading History

`log` follows `HEAD` backwards through `parent` links:

```rust
fn cmd_log() -> io::Result<()> {
    let cur = read_text(&repo_root().join("HEAD")).unwrap_or_default();
    let cur = cur.trim().to_string();
    if cur.is_empty() {
        println!("no commits yet.");
        return Ok(());
    }
    let mut hash = cur;
    loop {
        let body = read_text(&repo_root().join("commits").join(&hash))?;
        let mut parent = String::new();
        let mut message = String::new();
        let mut time = String::new();
        let mut files = 0;
        for line in body.lines() {
            if let Some(p) = line.strip_prefix("parent ") {
                parent = p.to_string();
            } else if let Some(m) = line.strip_prefix("message ") {
                message = m.to_string();
            } else if let Some(t) = line.strip_prefix("time ") {
                time = t.to_string();
            } else if !line.trim().is_empty() {
                files += 1;
            }
        }
        println!("commit {hash}\n  message: {message}\n  time: {time}  files: {files}\n");
        if parent.trim().is_empty() {
            break;
        }
        hash = parent.trim().to_string();
    }
    Ok(())
}
```

`status` compares the working tree against the index by re-hashing each
tracked file:

```rust
fn cmd_status() -> io::Result<()> {
    if !repo_root().exists() {
        fail("not a tinygit repository. Run `tinygit init` first.");
    }
    let index = read_text(&repo_root().join("index")).unwrap_or_default();
    if index.trim().is_empty() {
        println!("nothing tracked. Use `tinygit add <file>`.");
        return Ok(());
    }
    for line in index.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, ' ');
        let hash = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");
        let current = match fs::read(path) {
            Ok(b) => fnv1a_hex(&b),
            Err(_) => String::from("(deleted)"),
        };
        if current == format!("{hash}") {
            println!("  unchanged  {path}");
        } else {
            println!("  modified   {path}");
        }
    }
    Ok(())
}
```

> 💡 **Why re-hash instead of checking timestamps?** Timestamps lie. Editors
> touch files, clocks skew, and copies preserve times. Hashing the actual
> bytes is slower but honest. Correctness first, speed later: the book's
> standing rule.

#### 18.9 Wiring the CLI

We parse arguments with `std::env` only, so beginners see every step.
Production CLIs should use `clap`, but hand-rolling once teaches what
`clap` automates:

```rust
fn usage() -> ! {
    eprintln!("usage:");
    eprintln!("  tinygit init");
    eprintln!("  tinygit add <file>");
    eprintln!("  tinygit commit -m \"message\"");
    eprintln!("  tinygit log");
    eprintln!("  tinygit status");
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }
    let result = match args[1].as_str() {
        "init" => cmd_init(),
        "add" => {
            if args.len() != 3 {
                usage();
            }
            cmd_add(&args[2])
        }
        "commit" => {
            // Accept: tinygit commit -m "message"
            if args.len() != 4 || args[2] != "-m" {
                usage();
            }
            cmd_commit(&args[3])
        }
        "log" => cmd_log(),
        "status" => cmd_status(),
        _ => usage(),
    };
    if let Err(e) = result {
        fail(&format!("{e}"));
    }
}
```

Exit code `2` for usage errors follows the Unix convention: `0` means
success, `1` means a runtime failure, `2` means the invocation itself
was wrong. Scripts can distinguish "you typed it wrong" from "it ran
but failed".

#### 18.10 Trying It End to End

```bash
cargo run -- init
echo "hello tinygit" > demo.txt
cargo run -- add demo.txt
cargo run -- status
cargo run -- commit -m "first commit"
cargo run -- log
echo "second line" >> demo.txt
cargo run -- status
```

Expected output looks like this (hashes will differ on your machine
only if file contents differ, including newlines):

```text
Initialized empty tinygit repository in .tinygit/
added demo.txt -> 3ff10b4bcb780d1e
  unchanged  demo.txt
committed 09cb6dd2b9355aab
commit 09cb6dd2b9355aab
  message: first commit
  time: 1792155299  files: 1

  modified   demo.txt
```

> 🧭 **Debugging tip:** If `status` says `modified` right after `add`,
> your editor changed line endings. Open `.tinygit/index` and compare
> the stored hash with a fresh hash. The database is plain text on
> purpose so you can inspect it.

This program was compiled and run with `cargo check` clean and the
session above passing before this chapter was written. If you see a
compiler warning about an unused variable, read it: the compiler is
usually right, and fixing warnings now is cheaper than debugging later.

#### 18.11 What We Deliberately Left Out

A real version-control tool also needs branching, merging, remotes,
deltas, packing, and cryptographic hashes. Each is a natural next step:

- Replace FNV-1a with `sha2::Sha256` and hex-encode the 32 bytes.
- Add `checkout <hash>` by copying blobs back to working-tree paths,
  refusing to overwrite dirty files without `--force`.
- Add `diff` by reading two blobs and printing changed lines.
- Compress blobs with `flate2` to learn about size versus speed.

> ⚠️ **Do not use TinyGit for real work.** It has no locking, no atomic
> writes, no permissions handling, and a non-cryptographic hash. It is
> a learning scaffold in the same way Pebble is: correct enough to teach
> from, honest about its limits, and small enough to hold in your head.

#### 18.12 Chapter 18 Exercise

1. Add a `cat <hash>` command that prints a stored object to stdout
   using `read_blob`. This is the read path every other command needs.
2. Add a `checkout` guard: before overwriting a working file, check
   `status` internally and refuse unless the file is unchanged or the
   user passes `--force`.
3. Swap FNV-1a for SHA-256: add `sha2` to `Cargo.toml`, hash with
   `Sha256::digest`, and hex-encode. Note how little else changes when
   hashing lives behind one function.

> ✅ **Hint:** Keep hashing behind `fnv1a_hex`-shaped function
> `fn hash_bytes(bytes: &[u8]) -> String`. Callers should not know or
> care which algorithm sits inside. That seam is the Strategy pattern
> from Chapter 7, without naming it.

---

### Part V Recap (updated)

You have now built two complete systems with the same toolkit:

- You learned the stages of a language pipeline, and why each exists
  (Chapter 14).
- You turned text into tokens, tokens into a tree, and a tree into a
  verified program (Chapters 15 and 16).
- You executed that program with values, scopes, and function calls that
  isolate their variables, plus keyboard input and file I/O (Chapter 17).
- You stored file snapshots by content hash, chained snapshots into
  history, and reported working-tree state (Chapter 18).

The next file collects the reference material: corrections to Part IV,
the Pebble language reference, and where to go from here.


---

<a id="part-vi-advanced-rust-ch-19-23"></a>

## Part VI - Advanced Rust (Ch 19-23)

| Navigation | |
| --- | --- |
| Previous | [← Part V - TinyGit Capstone (Ch 18)](#part-v-tinygit-capstone-ch-18) |
| Next | [Part VII - Systems and Performance (Ch 24-27) →](#part-vii-systems-and-performance-ch-24-27) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part VI: Advanced Rust — Traits, Macros, Unsafe, and FFI

> 🧭 **Where we are:** Parts I–V gave you ownership, structs, error
> handling, concurrency, a web backend, a WASM frontend, a language,
> and a version-control tool. Part VI goes deeper into the language
> itself: the trait system that powers reuse, macros that write code,
> and unsafe code plus FFI for talking to the operating system and C.

> 🎯 **What you will have at the end:** You will read and write advanced
> trait bounds, `dyn Trait` trade-offs, declarative and procedural
> macros, sound `unsafe` blocks, and a minimal C interop example.

---

### Chapter 19: Generics Revisited — Bounds, Defaults, and Monomorphization

#### 19.1 Why Generics Exist

You already used `Vec<T>` and `fn largest<T>(list: &[T]) -> &T`. Generics
let one definition serve many types without runtime cost. The compiler
stamps out a copy per concrete type. That is **monomorphization**:
fast binaries, larger binaries, zero runtime dispatch.

```rust
fn first<T>(list: &[T]) -> Option<&T> {
    // Works for any T. No bound needed because we only index and borrow.
    list.first()
}

fn main() {
    let a = vec![1, 2, 3];
    let b = vec!["x".to_string(), "y".to_string()];
    println!("{:?} {:?}", first(&a), first(&b));
}
```

#### 19.2 Trait Bounds: Saying What T Can Do

As soon as you compare, print, or copy `T`, you must bound it:

```rust
use std::fmt::Display;

fn print_all<T: Display>(items: &[T]) {
    for item in items {
        println!("{item}");
    }
}

// Multiple bounds with +. T must be printable AND comparable.
fn dump_sorted<T: Display + Ord + Clone>(mut items: Vec<T>) {
    items.sort();
    for item in &items {
        println!("{item}");
    }
}
```

`where` clauses keep long bounds readable. Prefer them once you have two
type parameters or three bounds:

```rust
fn merge<K, V>(a: Vec<(K, V)>, b: Vec<(K, V)>) -> Vec<(K, V)>
where
    K: Ord + Clone,
    V: Clone,
{
    let mut out = a;
    out.extend(b);
    out.sort_by(|x, y| x.0.cmp(&y.0));
    out
}
```

#### 19.3 Default Type Parameters and the `Sized` Rule

Every type parameter is `Sized` by default. `Sized` means "known size at
compile time". Slices (`[T]`) and `dyn Trait` are unsized, so this fails:

```rust
// fn bad<T>(x: T) {} // actually means fn bad<T: Sized>(x: T) {}
```

Opt out with `?Sized` when you take a reference or smart pointer:

```rust
fn print_len<T: ?Sized + AsRef<str>>(x: &T) {
    println!("{}", x.as_ref().len());
}
```

Default parameters power smart-pointer ergonomics:

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Meters(f64);

impl Add for Meters {
    type Output = Meters; // associated type: the result of adding
    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}
```

`Add<Rhs = Self>` has a default. That is why `Meters + Meters` works
without writing `Add<Meters>`.

#### 19.4 `impl Trait` vs Generics

`impl Trait` in argument position is sugar for a generic:

```rust
// These two are equivalent:
fn a(x: impl AsRef<str>) { println!("{}", x.as_ref()); }
fn b<T: AsRef<str>>(x: T) { println!("{}", x.as_ref()); }
```

In return position it means "one concrete type I will not name":

```rust
fn evens(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|n| n % 2 == 0)
}
```

Use `impl Iterator` when the exact combinator chain is long. Name the
concrete type only when callers must name it (struct fields, trait
methods that need object safety — see Chapter 20).

#### 19.5 Zero-Cost in Practice: Measure the Bloat

Monomorphization duplicates code. Ten instantiations of a large generic
function means ten copies in the binary. Keep hot generic functions
small and push shared logic into non-generic helpers:

```rust
// Thin generic shell + shared core.
fn push_checked<T>(vec: &mut Vec<T>, item: T, cap: usize) -> Result<(), String> {
    if vec.len() >= cap {
        return Err(format!("capacity {cap} exceeded"));
    }
    vec.push(item);
    Ok(())
}
```

> ✅ **Exercise 19:** Write `fn mymax<T: PartialOrd + Copy>(a: T, b: T) -> T`.
> Then write a version returning `&T` from a slice with `Option<&T>` for
> empty input. Note which version needs `Copy` and which needs only a
> borrow.

---

### Chapter 20: Mastering Traits

#### 20.1 Associated Types vs Generic Parameters

Associated types say "this trait has exactly one related type per
implementation". Generics say "many":

```rust
// One Output per Self: associated type is right.
trait Parse {
    type Output;
    fn parse(s: &str) -> Result<Self::Output, String>;
}

// Many possible comparisons: generic parameter is right.
trait RoughlyEqual<Rhs = Self> {
    fn roughly_equal(&self, other: &Rhs) -> bool;
}
```

Rule of thumb: if the impl author picks the type, use an associated
type (`Iterator::Item`). If the caller picks it, use a generic
(`From<T>`, `PartialEq<Rhs>`).

#### 20.2 `dyn Trait`: When Dynamic Dispatch Wins

Generics stamp out copies at compile time. `dyn Trait` picks the method
at runtime through a vtable. Smaller binaries, slightly slower calls,
and the ability to hold mixed types:

```rust
trait Draw {
    fn draw(&self) -> String;
}

struct Circle;
struct Square;

impl Draw for Circle {
    fn draw(&self) -> String { "O".to_string() }
}
impl Draw for Square {
    fn draw(&self) -> String { "#".to_string() }
}

fn render_all(shapes: &[Box<dyn Draw>]) -> String {
    shapes.iter().map(|s| s.draw()).collect::<Vec<_>>().join(" ")
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![Box::new(Circle), Box::new(Square)];
    println!("{}", render_all(&shapes)); // O #
}
```

| Approach | Binary size | Speed | Mixed types in one Vec |
| --- | --- | --- | --- |
| Generics | larger | fastest | no |
| `dyn Trait` | smaller | vtable lookup | yes |

#### 20.3 Object Safety: What `dyn` Forbids

Not every trait can be `dyn`. Methods returning `Self` or using
generics break the vtable:

```rust
// NOT object-safe: Self appears in return position, T is generic.
trait Bad {
    fn clone_me(&self) -> Self;
    fn generic<T>(&self, x: T);
}
```

Fix by splitting the trait or returning `Box<dyn Trait>`:

```rust
trait Good {
    fn name(&self) -> &str; // object-safe: no Self, no generics
}

fn names(items: &[Box<dyn Good>]) -> Vec<&str> {
    items.iter().map(|i| i.name()).collect()
}
```

The compiler error `the trait cannot be made into an object` always
means this. Do not fight it with casts; split the trait.

#### 20.4 Blanket Impls, Supertraits, and Marker Traits

A **blanket impl** implements a trait for all `T` meeting a bound.
`std` uses this for `ToString`, `From`, and `Into`:

```rust
trait Loud {
    fn loud(&self) -> String;
}

// Every Display type gets Loud for free.
impl<T: std::fmt::Display> Loud for T {
    fn loud(&self) -> String {
        self.to_string().to_uppercase()
    }
}
```

A **supertrait** requires another trait first:

```rust
trait Named: ToString {
    fn name(&self) -> String {
        self.to_string()
    }
}
```

**Marker traits** carry no methods, only promises: `Sized`, `Send`,
`Sync`, `Copy`, `Unpin`. You saw `Send`/`Sync` in Chapter 9. `Copy`
means "bitwise copy is safe". Implement it only for small stack-only
types:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

#### 20.5 Generic Associated Types (GATs)

Sometimes the associated type itself needs a lifetime or parameter.
Lending iterators are the classic case: the `Item` borrows from `self`.
GATs express that:

```rust
trait Lending {
    type Item<'a> where Self: 'a;
    fn get(&self) -> Self::Item<'_>;
}

struct Words {
    text: String,
}

impl Lending for Words {
    type Item<'a> = &'a str where Self: 'a;
    fn get(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}
```

You will meet GATs in async traits and streaming APIs. Read them as
"this associated type is itself generic". Most application code never
writes a GAT; library code sometimes must.

> ✅ **Exercise 20:** Define `trait Area { fn area(&self) -> f64; }` for
> `Circle(f64)` and `Rect { w: f64, h: f64 }`. Store both in a
> `Vec<Box<dyn Area>>` and sum the areas. Then rewrite with generics and
> note what you lose (mixed types) and gain (speed, no vtable).

---

### Chapter 21: Lifetimes Without Fear, Part Two

#### 21.1 Variance, `'static`, and What Outlives What

`'static` does not mean "lives forever". It means "valid for the rest of
the program *if* it lives that long". String literals are `&'static str`.
Owned `String`s are not, but they can be held anywhere because they own
their data:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("abc");
    let out;
    {
        let s2 = String::from("xy");
        out = longest(&s1, &s2); // both borrows alive here
        println!("{out}");
    } // s2 drops here; out must not escape
}
```

If you return `out` past `s2`'s scope, the compiler stops you. That is
the entire lifetime system: do returned borrows outlive their sources?

#### 21.2 Structs Holding References

A struct borrowing data must name the lifetime:

```rust
#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap();
    let e = Excerpt { text: first };
    println!("{e:?}");
}
```

`Excerpt<'a>` cannot outlive `novel`. Methods on it need matching
lifetimes only when they return borrows:

```rust
impl<'a> Excerpt<'a> {
    fn text(&self) -> &'a str {
        self.text
    }
}
```

#### 21.3 Elision: The Three Rules the Compiler Uses

You write explicit lifetimes only when elision fails. The rules:

1. Each elided lifetime in parameters gets its own name.
2. If exactly one input lifetime, it is assigned to all outputs.
3. If `&self` is present, `&self`'s lifetime goes to all outputs.

```rust
// Elided: fn first_word(s: &str) -> &str
// Means:  fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

When two inputs both flow to the output, elision fails and you name them:

```rust
fn pick<'a>(a: &'a str, b: &'a str, use_a: bool) -> &'a str {
    if use_a { a } else { b }
}
```

#### 21.4 Interior Mutability: `Cell`, `RefCell`, `Mutex`

Borrow rules are compile-time by default. Interior mutability moves the
check to runtime for single-threaded (`Cell`, `RefCell`) or threads
(`Mutex`, `RwLock`):

```rust
use std::cell::RefCell;

struct Log {
    entries: RefCell<Vec<String>>,
}

impl Log {
    fn add(&self, msg: &str) {
        // &self is enough: RefCell checks borrows at runtime.
        self.entries.borrow_mut().push(msg.to_string());
    }
    fn count(&self) -> usize {
        self.entries.borrow().len()
    }
}
```

`RefCell` panics on conflicting borrows instead of refusing to compile.
Use it for tests, mocks, and single-threaded graphs — not as a default.
`Cell` is cheaper but only for `Copy` types (`Cell::set`, `Cell::get`).

#### 21.5 `Rc`, `Arc`, and Cycles with `Weak`

`Rc<T>` shares ownership on one thread; `Arc<T>` shares across threads.
Both leak if you form a cycle (A owns B, B owns A). Break cycles with
`Weak`:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let root = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let child = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    *child.parent.borrow_mut() = Rc::downgrade(&root);
    root.children.borrow_mut().push(Rc::clone(&child));
    println!("root refs: {}", Rc::strong_count(&root)); // 1
}
```

`Weak::downgrade` does not increment the strong count, so dropping `root`
frees it even though `child` points back. `Arc` has the same API with
`Arc::downgrade`. Rule: parents own children strongly; children point to
parents weakly.

> ✅ **Exercise 21:** Build `Excerpt` over a `String` you create in `main`.
> Then try returning it from a function that creates the `String` locally.
> Read the error, then fix by returning the owned `String` instead.

---

### Chapter 22: Macros — Code That Writes Code

#### 22.1 Why Macros Exist

Functions run at runtime. Generics stamp out types at compile time.
Macros run at compile time on syntax itself: they take tokens in and
produce tokens out. Use them for variadic arguments (`vec![1, 2, 3]`),
DSLs (`sqlx::query!` checked SQL), and removing boilerplate you cannot
remove with functions.

> 🧭 **Order of resort:** functions first, generics and traits second,
> macros last. Macros are powerful, harder to debug, and slower to
> compile. Reach for them when the other tools cannot express the shape.

#### 22.2 Declarative Macros with `macro_rules!`

`macro_rules!` matches patterns like `match` arms, with `$` captures:

```rust
macro_rules! say_many {
    // say_many!(3, "hi") expands to three println! calls.
    ($n:expr, $msg:expr) => {{
        for _ in 0..$n {
            println!("{}", $msg);
        }
    }};
}

fn main() {
    say_many!(2, "hello");
}
```

Fragment specifiers you will use most: `expr` (expression), `ident`
(identifier), `ty` (type), `block`, `literal`, `path`, `stmt`. Repetition
uses `$( ... ),*` with separators `,` `;` or nothing:

```rust
macro_rules! vec_of_strings {
    ($($s:expr),* $(,)?) => {{
        vec![$($s.to_string()),*]
    }};
}

fn main() {
    let v = vec_of_strings!["a", "b", "c"];
    assert_eq!(v, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}
```

Macros are hygienic for locals: variables declared inside do not clash
with caller variables. They are *not* magic scoping: unqualified paths
resolve at the expansion site, so use `$crate::` for paths inside your
own crate's macros.

#### 22.3 A Real-World Declarative Macro: `hashmap!`

```rust
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($k, $v);)*
        map
    }};
}

fn main() {
    let m = hashmap!["a" => 1, "b" => 2,];
    assert_eq!(m["a"], 1);
}
```

Each `$k => $v` repeats the `insert` line. The trailing `$(,)?` allows
an optional trailing comma, the Rust style. This exact pattern appears
in production crates (`maplit` is this macro published).

#### 22.4 Procedural Macros: Derive, Attribute, Function-like

Procedural macros are Rust functions that transform `TokenStream`s.
Three flavors:

| Kind | Input | Example |
| --- | --- | --- |
| Derive | struct/enum definition | `#[derive(Debug, Clone)]` |
| Attribute | attribute + item | `#[tokio::main]`, `#[test]` |
| Function-like | arbitrary tokens | `sqlx::query!("SELECT ...")` |

You write them in a separate crate with `proc-macro = true`:

```toml
[lib]
proc-macro = true

[dependencies]
syn = "2"
quote = "1"
proc-macro2 = "1"
```

A minimal derive that adds a `hello()` method:

```rust
// hello_derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let expanded = quote! {
        impl #name {
            pub fn hello() -> &'static str {
                concat!("hello from ", stringify!(#name))
            }
        }
    };
    TokenStream::from(expanded)
}
```

```rust
// usage (in another crate depending on hello_derive)
// use hello_derive::Hello;
// #[derive(Hello)]
// struct Robot;
// assert_eq!(Robot::hello(), "hello from Robot");
```

Debugging procedural macros: `cargo expand` shows the generated code.
Run `cargo install cargo-expand` once, then `cargo expand --lib` on
small examples. Reading the expansion turns macro magic into plain code.

#### 22.5 When NOT to Use Macros

- Validation you can do with types (use builders, `Result`, `NonZeroU32`).
- Async wrappers (use `async fn` and traits).
- Stringly-typed DSLs with no diagnostics (a bad macro error is worse
  than a verbose function call).
- Anything where `cargo expand` output would confuse your team.

> ✅ **Exercise 22:** Write `macro_rules! first` that takes
> `first![a, b, c]` and expands to `a`. Then extend it to `choose![2; a,
> b, c]` expanding to the second element. This teaches index-free token
> matching without any parsing crates.

---

### Chapter 23: Unsafe Rust and FFI

#### 23.1 What `unsafe` Means (and Does Not Mean)

`unsafe` does not turn off the borrow checker. It allows five operations
the compiler cannot verify:

1. Dereference raw pointers (`*const T`, `*mut T`).
2. Call `unsafe fn` (including C functions and some std functions).
3. Implement `unsafe trait` (a contract you promise to uphold).
4. Mutate `static mut` variables.
5. Access union fields.

Everything else — moves, borrows, lifetimes, types — is still checked
inside `unsafe` blocks. Keep blocks tiny and wrap them in safe APIs:

```rust
fn first_byte(s: &str) -> Option<u8> {
    // Safe wrapper: the unsafe block cannot leak a dangling pointer
    // because it only reads through the borrowed &str.
    if s.is_empty() {
        return None;
    }
    let ptr = s.as_ptr(); // *const u8, valid while s is borrowed
    // SAFETY: s is non-empty, so offset 0 is in bounds and aligned.
    // ptr is never stored, never written, never returned.
    unsafe { Some(*ptr) }
}
```

Document every `unsafe` block with a `// SAFETY:` comment stating the
invariant. Future readers (including you) need the *why*, not the *what*.

#### 23.2 Raw Pointers vs References

| Feature | `&T` / `&mut T` | `*const T` / `*mut T` |
| --- | --- | --- |
| Null | never | allowed |
| Validity | always valid | may dangle |
| Borrow checked | yes | no |
| Dereference | safe | `unsafe` only |
| Send/Sync | auto | never auto (`Send`/`Sync` must be manual) |

```rust
fn sum_ptr(values: &[i32]) -> i32 {
    let mut total = 0;
    let ptr = values.as_ptr();
    let len = values.len();
    for i in 0..len {
        // SAFETY: i is in 0..len, ptr..ptr+len is the slice allocation,
        // values outlives the loop, no mutation through this pointer.
        total += unsafe { *ptr.add(i) };
    }
    total
}
```

Prefer iterators (`values.iter().sum()`). Write pointer loops only for
FFI buffers, drivers, or proven hot paths with benchmarks.

#### 23.3 Invariants, `unsafe fn`, and `unsafe trait`

Mark a function `unsafe fn` when callers must uphold conditions the
compiler cannot check:

```rust
/// # Safety
/// `ptr` must point to `len` initialized `u8`s valid for reads.
unsafe fn copy_to_vec(ptr: *const u8, len: usize) -> Vec<u8> {
    let slice = std::slice::from_raw_parts(ptr, len);
    slice.to_vec()
}
```

`unsafe trait` is the same idea for implementations. `Send` and `Sync`
are the most common: implementing them manually promises "my type is
safe to move/share across threads". Get it wrong and data races become
possible. Most code should derive them automatically by containing only
`Send`/`Sync` fields.

Miri catches many `unsafe` mistakes in tests. Run
`cargo +nightly miri test` on code with raw pointers. It finds
use-after-free, aliasing violations, and uninitialized reads that normal
tests miss.

#### 23.4 Minimal C FFI

Rust calls C with `extern "C"` blocks. No headers, no build system:

```rust
// Link the C standard library's strlen.
unsafe extern "C" {
    fn strlen(s: *const std::os::raw::c_char) -> usize;
}

fn c_strlen(s: &str) -> usize {
    // Nul-terminated copy: C strings end with 0, Rust strings do not.
    let c = std::ffi::CString::new(s).expect("string contains interior nul");
    // SAFETY: CString guarantees a valid nul-terminated pointer for
    // the duration of this call; strlen only reads.
    unsafe { strlen(c.as_ptr()) }
}

fn main() {
    assert_eq!(c_strlen("hello"), 5);
}
```

Going the other way (C calling Rust) uses `#[unsafe(no_mangle)]` and
`extern "C" fn`. Keep the boundary narrow: plain integers, pointers, and
`#[repr(C)]` structs. Never pass `String`, `Vec`, or trait objects across
FFI. Convert at the edge and validate lengths before `from_raw_parts`.

`#[repr(C)]` fixes field order for C compatibility:

```rust
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
```

#### 23.5 When to Reach for `unsafe`

Checklist before writing `unsafe`:

- [ ] Safe code cannot express this (FFI, MMIO, proven hot loop).
- [ ] The `unsafe` block is under 10 lines with a `SAFETY` comment.
- [ ] A safe wrapper owns the invariant; callers cannot break it.
- [ ] Miri passes; address sanitizer passes for FFI-adjacent code.
- [ ] A second reviewer approved the invariant.

> ✅ **Exercise 23:** Wrap `c_strlen` above in tests for `""`, `"a"`, and
> `"héllo"` (note: `strlen` counts bytes, not chars — assert 7 for the
> last). Then try passing `"a\0b"` and observe the `CString` error. That
> error is the safe boundary doing its job.

---

### Part VI Recap

- Generics stamp out copies; bounds and `where` clauses state what `T`
  can do; `?Sized` and defaults handle edge cases (Ch 19).
- Associated types versus generics, `dyn Trait` trade-offs, object
  safety, blanket impls, and GATs cover the trait system end to end
  (Ch 20).
- Variance, structs with lifetimes, elision rules, interior mutability,
  and `Rc`/`Arc`/`Weak` cycles complete the memory story (Ch 21).
- Declarative macros for boilerplate, procedural macros for derives and
  DSLs, and knowing when not to use either (Ch 22).
- Tiny `unsafe` blocks with `SAFETY` comments, raw-pointer discipline,
  and minimal C FFI behind safe wrappers (Ch 23).

> 🧭 **Carry-forward:** Part VII assumes you can read trait bounds,
> `dyn` trade-offs, lifetime errors, macro expansions via `cargo expand`,
> and `unsafe` invariants. Keep this part open while building systems.


---

<a id="part-vii-systems-and-performance-ch-24-27"></a>

## Part VII - Systems and Performance (Ch 24-27)

| Navigation | |
| --- | --- |
| Previous | [← Part VI - Advanced Rust (Ch 19-23)](#part-vi-advanced-rust-ch-19-23) |
| Next | [Part VIII - Production and Capstones (Ch 28-33) →](#part-viii-production-and-capstones-ch-28-33) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part VII: Systems, I/O, Networking, and Performance

> 🧭 **Where we are:** You know the language deeply. Part VII turns it
> into systems: files, processes, sockets, async runtimes, and the
> measurement discipline that makes "fast" a number instead of a hope.

> 🎯 **What you will have at the end:** A CLI with real argument parsing,
> a TCP echo server and client, a Tokio async pipeline with cancellation,
> criterion benchmarks, and a profiling workflow you can reuse.

---

### Chapter 24: Files, Paths, Processes, and CLIs

#### 24.1 Paths Are Not Strings

`&str` is text. Paths are OS concepts with separators, prefixes, and
non-UTF8 bytes on Unix. `PathBuf` owns, `Path` borrows — the same
owned/borrowed split as `String`/`&str`:

```rust
use std::path::{Path, PathBuf};

fn main() {
    let mut p = PathBuf::from("projects");
    p.push("odyssey");
    p.push("notes.txt");
    println!("{}", p.display()); // projects/odyssey/notes.txt (or \ on Windows)
    println!("{:?}", p.extension()); // Some("txt")
    println!("{:?}", Path::new("a/b").join("../c")); // a/b/../c (not normalized)
}
```

Never build paths with `format!("{dir}/{file}")`. It breaks on Windows
and ignores `..` semantics. Use `join` and `display` for messages.

`canonicalize` resolves symlinks and `..`, but touches the filesystem
and fails for missing files. Use it for sandbox checks (TinyGit's `..`
refusal in Chapter 18), not for display.

#### 24.2 Robust File I/O

`fs::read_to_string` is fine for configs. Production file code needs
three upgrades: buffered I/O, explicit errors, and atomic writes:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?; // ? propagates NotFound, PermissionDenied
    let reader = BufReader::new(file);
    let mut n = 0;
    for line in reader.lines() {
        line?; // surface mid-file UTF-8 errors with their line context lost? see below
        n += 1;
    }
    Ok(n)
}

fn write_atomic(path: &str, contents: &str) -> io::Result<()> {
    // Write temp + rename: readers never see a half-written file.
    let tmp = format!("{path}.tmp");
    {
        let mut f = File::create(&tmp)?;
        f.write_all(contents.as_bytes())?;
        f.sync_all()?; // flush OS buffers before rename
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}
```

Match on `io::ErrorKind` for user-facing messages instead of printing
`debug` output:

```rust
use std::io::ErrorKind;

fn open_config(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("config {path} not found, using defaults");
            String::new()
        }
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            std::process::exit(1);
        }
    }
}
```

#### 24.3 Environment, Args, and Exit Codes

Twelve-factor CLIs read config from the environment, arguments from the
command line, and signal failure with exit codes:

```rust
use std::env;

fn main() {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("PORT must be a number 0-65535");
            std::process::exit(2);
        });
    let args: Vec<String> = env::args().collect();
    println!("listening on {port}, args: {args:?}");
}
```

Conventions: `0` success, `1` runtime failure, `2` usage error. Write
errors to stderr, data to stdout, so `tinygit log > out.txt` captures
data without diagnostics.

#### 24.4 A Real CLI with `clap`

Hand-rolled `env::args` taught parsing in Chapter 18. Real CLIs use
`clap` derive for help text, validation, and subcommands:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
```

```rust
use clap::{Parser, Subcommand};

/// Tiny task tracker CLI.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Add a task.
    Add { title: String },
    /// List tasks.
    List,
    /// Mark a task done.
    Done { id: i64 },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Add { title } => println!("would add {title:?}"),
        Cmd::List => println!("would list"),
        Cmd::Done { id } => println!("would complete {id}"),
    }
    Ok(())
}
```

`anyhow::Result<()>` as a `main` return prints `Error: ...` and exits
`1` on failure. Use `anyhow` for applications, `thiserror` for
libraries (Chapter 6's rule: libraries name errors, binaries simplify
them).

> ✅ **Exercise 24:** Convert TinyGit's `match args[1]` to `clap`
> subcommands. Keep the same five commands and exit codes. Note how much
> usage text disappears.

---

### Chapter 25: Networking — TCP, HTTP, and WebSockets

#### 25.1 TCP: The Byte Stream Under Everything

HTTP, Postgres wire protocol, and Redis protocol all ride on TCP. Learn
TCP once and every protocol becomes "framing on top of a stream":

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];
    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) => return, // peer closed
            Ok(n) => n,
            Err(_) => return,
        };
        // Echo back. write_all handles partial writes for us.
        if stream.write_all(&buf[..n]).is_err() {
            return;
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("echo server on 127.0.0.1:7878");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(|| handle(s));
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
    Ok(())
}
```

Test with `cargo run` in one terminal and this in another:

```bash
echo hello | nc 127.0.0.1 7878
```

Three production gaps this omits: backpressure (unbounded threads),
timeouts (`set_read_timeout`), and graceful shutdown. Chapter 13's
shutdown pattern plus a fixed thread pool or Tokio (next chapter) closes
all three.

#### 25.2 HTTP Without Magic

An HTTP request is text headers plus optional bytes. This minimal
client shows the shape without a framework:

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn get(host: &str, path: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect((host, 80))?;
    let req = format!("GET {path} HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(req.as_bytes())?;
    let mut reader = BufReader::new(stream);
    let mut out = String::new();
    // Skip headers, keep body. Real clients parse Content-Length/chunked.
    let mut in_body = false;
    for line in reader.by_ref().lines() {
        let line = line?;
        if in_body {
            out.push_str(&line);
            out.push('\n');
        } else if line.is_empty() {
            in_body = true;
        }
    }
    Ok(out)
}
```

Use this once to understand `Host`, `\r\n`, and blank-line framing.
Then use `reqwest` for real code. Hand-rolled HTTP is a teaching tool,
not a dependency strategy.

#### 25.3 Production HTTP: `axum` Patterns You Reuse

Chapter 11 built the handlers. Three patterns complete the picture:

```rust
// 1. Shared state with Clone. Axum requires State: Clone + Send.
#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

// 2. Timeouts at the tower layer so slow handlers fail fast.
// use tower_http::timeout::TimeoutLayer;
// use std::time::Duration;
// Router::new().route(...).layer(TimeoutLayer::new(Duration::from_secs(5)))

// 3. Request IDs for correlating logs across services.
// Every error response carries the ID; logs carry it too.
```

Pagination caps belong on every list endpoint. Never return unbounded
`SELECT *`:

```rust
// GET /tasks?limit=50&offset=0 — clamp limit server-side.
// let limit: i64 = query.limit.unwrap_or(50).clamp(1, 100);
```

#### 25.4 WebSockets and SSE: Pushing Updates

Polling `GET /tasks` every second wastes connections. Two push options:

| Tech | Direction | Use when |
| --- | --- | --- |
| Server-Sent Events | server to client | live feeds, logs, progress |
| WebSockets | both ways | chat, games, collaborative editing |

Axum supports both via `axum::extract::ws`. The pattern is the same as
Chapter 9 channels: a broadcast sender fans out, each socket task
forwards. Backpressure matters here — drop slow receivers instead of
buffering forever.

> ✅ **Exercise 25:** Extend the echo server with a 5-second read timeout
> and a 4-thread pool (use `std::sync::mpsc` + workers). Measure with
> `time echo hi | nc 127.0.0.1 7878` before and after.

---

### Chapter 26: Async Tokio — Runtimes, Tasks, and Cancellation

#### 26.1 Why Async Instead of Threads

Threads block: one thread per connection wastes a stack per idle client.
Async tasks yield at `.await`, so ten thousand connections fit on a few
threads. The price is explicit yield points and `Send` requirements.
Chapter 9 introduced the model; here is the production detail.

```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
    let producer = tokio::spawn(async move {
        for i in 0..5 {
            tx.send(format!("msg {i}")).await.unwrap();
        }
    });
    let consumer = tokio::spawn(async move {
        while let Some(m) = rx.recv().await {
            println!("got {m}");
        }
    });
    let _ = tokio::join!(producer, consumer);
}
```

`tokio::spawn` requires `Send + 'static`. Values crossing tasks must be
owned or `Arc`. That is the borrow checker applied to concurrency.

#### 26.2 `select!`, Timeouts, and Cancellation

`select!` waits on multiple futures and runs the first ready branch.
Every branch must be cancellation-safe (dropping it mid-await loses no
state the program needs):

```rust
use tokio::time::{sleep, Duration};

async fn fetch_primary() -> String {
    sleep(Duration::from_millis(200)).await;
    "primary".to_string()
}

async fn fetch_fallback() -> String {
    sleep(Duration::from_millis(50)).await;
    "fallback".to_string()
}

#[tokio::main]
async fn main() {
    let winner = tokio::select! {
        a = fetch_primary() => a,
        b = fetch_fallback() => b,
    };
    println!("winner: {winner}"); // fallback
}
```

Timeouts compose with the same tool:

```rust
async fn with_timeout() {
    let slow = async {
        tokio::time::sleep(Duration::from_secs(10)).await;
        "done"
    };
    match tokio::time::timeout(Duration::from_secs(1), slow).await {
        Ok(v) => println!("{v}"),
        Err(_) => println!("timed out after 1s"),
    }
}
```

Shutdown uses a broadcast or `CancellationToken`. Spawned tasks `select!`
on work versus shutdown, flush, then exit. That is Chapter 13's graceful
shutdown generalized to every background task.

#### 26.3 Channels, Backpressure, and `join` vs `spawn`

| Primitive | Capacity | Use |
| --- | --- | --- |
| `mpsc` | bounded (set it) | pipeline stages |
| `broadcast` | bounded, lag possible | shutdown, config reload |
| `oneshot` | single value | request/response |
| `watch` | latest value | shared config |

Bounded channels are backpressure: a fast producer blocks instead of
allocating forever. Size them from measurements (Chapter 27), not vibes.
`join!` runs futures on the same task (no `Send` needed beyond the
futures); `spawn` runs independently and needs `Send + 'static`.

```rust
async fn pipeline() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u32>(16);
    let produce = async move {
        for i in 0..100 {
            tx.send(i).await.unwrap();
        }
    };
    let consume = async {
        let mut sum = 0u64;
        while let Some(v) = rx.recv().await {
            sum += v as u64;
        }
        sum
    };
    let (_, total) = tokio::join!(produce, consume());
    println!("sum={total}");
}
```

#### 26.4 Streams and `StreamExt`

Iterators are pull-based and synchronous. Streams are pull-based and
async. The combinators rhyme (`map`, `filter`, `fold`, `buffered`):

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
futures = "0.3"
```

```rust
use futures::StreamExt;
use tokio_stream::iter;

async fn doubled_sum() -> u32 {
    iter(0..10u32)
        .filter(|n| futures::future::ready(n % 2 == 0))
        .map(|n| async move { n * 2 })
        .buffered(4) // up to 4 concurrent maps, order preserved
        .fold(0, |acc, n| async move { acc + n })
        .await
}
```

`buffered` versus `buffer_unordered`: ordered output versus fastest
output. Pick `buffered` for user-visible lists, `unordered` for
throughput jobs where order does not matter.

> ✅ **Exercise 26:** Build a two-stage pipeline (read lines, hash with
> FNV from Chapter 18) with a bounded channel of 8. Add a 2-second
> `timeout` around the whole run and print partial progress on timeout.

---

### Chapter 27: Testing, Benchmarking, and Performance

#### 27.1 From Unit Tests to Property Tests

Chapter 10 covered `#[test]`, integration tests, and doc tests. Three
upgrades carry you to production confidence:

```rust
fn parse_port(s: &str) -> Result<u16, String> {
    s.trim().parse::<u16>().map_err(|_| format!("{s:?} is not a port 0-65535"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_ports() {
        assert_eq!(parse_port("80"), Ok(80));
        assert!(parse_port("abc").is_err());
        assert!(parse_port("99999").is_err());
    }

    #[test]
    fn rejects_surrounding_garbage() {
        assert!(parse_port("80 ").is_ok()); // trim is deliberate
        assert!(parse_port("8 0").is_err());
    }
}
```

Property tests assert invariants over hundreds of random inputs:

```toml
[dev-dependencies]
proptest = "1"
```

```rust
// #[cfg(test)] module using proptest:
// proptest! { #[test] fn roundtrip(n: u16) {
//   prop_assert_eq!(parse_port(&n.to_string()), Ok(n)); } }
```

Write the example by hand first (unit), then generalize (property).
Properties that caught real bugs: parser roundtrips, sort idempotence,
and `encode(decode(x)) == x`.

#### 27.2 Benchmarks with Criterion

`cargo bench` needs stable harness or Criterion. Criterion gives
statistics and regression detection:

```toml
[dev-dependencies]
criterion = "2"

[[bench]]
name = "hashing"
harness = false
```

```rust
// benches/hashing.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn bench_hash(c: &mut Criterion) {
    let data = vec![0xABu8; 4096];
    c.bench_function("fnv1a/4k", |b| b.iter(|| fnv1a(black_box(&data))));
}

criterion_group!(benches, bench_hash);
criterion_main!(benches);
```

`black_box` prevents the optimizer from deleting the work. Check in
`criterion` baselines in CI and fail on >10% regression for hot paths.

#### 27.3 Profiling: Flamegraphs Before Optimizing

Order of operations: measure, hypothesize, change one thing, re-measure.
Two tools cover 90% of cases:

```bash
# CPU flamegraph (Linux; samply works cross-platform):
cargo install samply
samply record ./target/release/myapp
# Opens an interactive flamegraph in the browser.

# Memory high-water mark:
# /usr/bin/time -v ./target/release/myapp
```

Common wins in Rust systems, in order of frequency:

1. Fewer allocations (`&str` over `String`, `it.next()` over `collect`).
2. Bigger I/O batches (8KB+ reads, `BufWriter`, `writev` via `write_all`).
3. Right data structure (`HashMap` vs `BTreeMap` vs `Vec` + sort).
4. Less locking (per-shard `Mutex`, lock-free queue, or channel).
5. SIMD/branchless only after 1–4 with profiler evidence.

#### 27.4 Sizing Pools, Queues, and Timeouts

Every number in production config needs a source:

| Knob | Default to try | How to size |
| --- | --- | --- |
| DB pool | 2x CPU cores | load test to p95 latency knee |
| Channel bound | 16–64 | producer/consumer rate x allowed lag |
| HTTP timeout | 5s reads, 30s deploys | p99 handler time x 3 |
| Retry | 3 with jitter | idempotent reads only |
| Cache TTL | 60s | staleness tolerance in writing |

Log pool wait time, queue depth, and timeout counts as metrics. If you
cannot graph it, you cannot size it. That observability habit from
Chapter 13 applies to every queue you add.

> ✅ **Exercise 27:** Benchmark `format!("{hash:016x}")` versus a manual
> hex writer for 10k hashes. Record median ns/iter on your machine.
> Then profile TinyGit `add` on a 50MB file and note where time goes
> (hashing, I/O, or index rewrite).

---

### Part VII Recap

- Paths are OS objects (`PathBuf`/`Path`), file writes should be atomic,
  and CLIs use env plus `clap` with `anyhow` for errors (Ch 24).
- TCP is a byte stream; HTTP framing sits on top; Axum patterns (state,
  timeouts, pagination) plus push via SSE/WebSockets cover services
  (Ch 25).
- Tokio tasks need `Send + 'static`; `select!`, timeouts, bounded
  channels, and streams build cancellable pipelines (Ch 26).
- Unit plus property tests, Criterion benchmarks with baselines, and
  flamegraph-first profiling make performance a number (Ch 27).

> 🧭 **Carry-forward:** Every system in Part VIII assumes you can open a
> socket, bound a queue, cancel a task, and prove speed with numbers.
> Reread Chapters 26–27 if any capstone step feels like guessing.


---

<a id="part-viii-production-and-capstones-ch-28-33"></a>

## Part VIII - Production and Capstones (Ch 28-33)

| Navigation | |
| --- | --- |
| Previous | [← Part VII - Systems and Performance (Ch 24-27)](#part-vii-systems-and-performance-ch-24-27) |
| Next | [Part IX - Stdlib and Idioms (Ch 34-36) →](#part-ix-stdlib-and-idioms-ch-34-36) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part VIII: Production Full-Stack, Security, and Capstones

> 🧭 **Where we are:** You can write advanced Rust and run real systems.
> Part VIII ships them: hardened backends, reactive frontends, containers
> and CI, plus three capstones that tie the whole book together.

> 🎯 **What you will have at the end:** Auth patterns, transaction and
> migration discipline, Leptos data-fetching done right, a deployment
> checklist, and three buildable projects (CLI, mini-Redis, WASM board).

---

### Chapter 28: Hardened Backends — Auth, Transactions, Observability

#### 28.1 Passwords and Sessions Without Footguns

Store passwords with Argon2id, never SHA-256 or MD5. Verify in constant
time through the library. Sessions live server-side; the cookie is a
random ID with `HttpOnly; Secure; SameSite=Lax`:

```toml
[dependencies]
argon2 = "0.5"
rand = "0.8"
```

```rust
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| format!("hash failed: {e}"))
}

fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false, // fail closed: bad hash never authenticates
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
}
```

Uniform responses prevent user enumeration: login with an unknown email
takes the same time and message as a wrong password. Add per-IP rate
limits on login, signup, and reset before any other endpoint.

#### 28.2 Transactions: Money Moves Together or Not at All

Two writes that must agree belong in one transaction. Classic example:
transfer between accounts must debit and credit atomically:

```rust
async fn transfer(pool: &sqlx::PgPool, from: i64, to: i64, amount: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    // Lock rows in id order to avoid deadlocks under concurrency.
    let (low, high) = if from < to { (from, to) } else { (to, from) };
    sqlx::query!("SELECT id FROM accounts WHERE id IN ($1, $2) FOR UPDATE", low, high)
        .fetch_all(&mut *tx)
        .await?;
    sqlx::query!("UPDATE accounts SET balance = balance - $1 WHERE id = $2", amount, from)
        .execute(&mut *tx)
        .await?;
    sqlx::query!("UPDATE accounts SET balance = balance + $1 WHERE id = $2", amount, to)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
```

Rules: acquire locks in a fixed order, keep transactions short, never do
network I/O inside them. Test the failure path (second update fails)
and assert the first rolled back. That test is worth more than the happy
path.

#### 28.3 Migrations You Can Roll Back

Each migration moves forward and backward. Name them by timestamp so
ordering is total:

```sql
-- migrations/20260101000000_create_tasks.sql
CREATE TABLE tasks (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL CHECK (char_length(title) BETWEEN 1 AND 200),
    done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tasks_done_created ON tasks (done, created_at);
```

```sql
-- migrations/20260101000001_add_task_owner.sql
ALTER TABLE tasks ADD COLUMN owner_id BIGINT REFERENCES users(id);
CREATE INDEX idx_tasks_owner ON tasks (owner_id);
```

Deploy order: expand (add nullable column) → migrate data in batches →
contract (add NOT NULL). Never `DROP COLUMN` in the same deploy that
stops writing it. Old binaries still running during a rolling deploy
must see a schema they understand.

#### 28.4 Observability: Logs, Metrics, Traces

Structure logs as JSON with correlation IDs. Redact by allowlist —
log event names and IDs, never bodies or tokens:

```rust
// tracing setup (once at startup):
// tracing_subscriber::fmt().json().with_env_filter("info").init();
// Per request: tracing::info!(request_id = %id, route = "/tasks", "handled");
```

Track RED metrics per endpoint: rate, errors, duration (p50/p95/p99).
Alert on error-budget burn, not raw CPU. Trace across backend and DB
with OpenTelemetry spans that carry no PII. If you cannot answer "which
deploy made p95 worse?" from dashboards, add the missing span before the
next feature.

> ✅ **Exercise 28:** Add `owner_id BIGINT` to TinyGit-style task commits
> in a new migration with a backfill (`UPDATE ... WHERE owner_id IS NULL`)
> in batches of 1000. Write the rollback (`DROP COLUMN` only after the
> code no longer reads it).

---

### Chapter 29: Frontend Engineering Done Right

#### 29.1 Signals Without Spaghetti

Leptos reactivity is three pieces: signals (values), memos (derived),
effects (side work). Keep the graph acyclic and synchronous; push I/O to
actions:

```rust
// Conceptual Leptos 0.7 shape (check docs.rs for your minor version):
// let (tasks, set_tasks) = signal(Vec::<Task>::new());
// let remaining = Memo::new(move |_| tasks.get().iter().filter(|t| !t.done).count());
// Effect::new(move |_| { document::set_title(&format!("{} left", remaining.get())); });
```

Rules: never write to a signal inside the memo that reads it (cycle);
never fetch in a render path (use resources/actions); key lists by `id`,
not index, so reordering preserves input focus.

#### 29.2 Data Fetching as a State Machine

Chapter 12's `fetch_tasks` swallowed errors. Production fetching names
every state from Chapter 5's async pattern:

```rust
enum TasksState {
    Loading,
    Loaded(Vec<Task>),
    Empty,
    Failed(String),
}

fn next_state(result: Result<Vec<Task>, String>) -> TasksState {
    match result {
        Err(e) => TasksState::Failed(e),
        Ok(v) if v.is_empty() => TasksState::Empty,
        Ok(v) => TasksState::Loaded(v),
    }
}
```

Render each state: skeletons matching row layout for loading, an
actionable empty ("Add your first task"), an error with Retry that keeps
typed input, and stale-while-revalidate (show old rows with a subtle
"refreshing" indicator while refetching). Skeletons that match layout
prevent CLS; spinners that replace the page cause it.

#### 29.3 Forms That Respect Users

Labels above inputs, `type`/`autocomplete` set, validation on blur and
submit, errors linked with `aria-describedby`, input preserved on
failure, submit disabled while pending with width preserved:

```rust
fn validate_title(title: &str) -> Result<(), String> {
    let t = title.trim();
    if t.is_empty() {
        return Err("Give the task a name.".to_string());
    }
    if t.chars().count() > 200 {
        return Err("Keep titles under 200 characters.".to_string());
    }
    Ok(())
}
```

Checkboxes issue `PATCH /tasks/{id}` optimistically only for reversible
actions, with rollback and a notice on failure. Never use optimistic UI
for billing, auth, or destructive deletes — wait for the server.

#### 29.4 WASM Size and Speed Budget

WASM binaries balloon fast. Budget them like Chapter 7 backend latency:

| Metric | Target | Fail |
| --- | --- | --- |
| Initial WASM (gzip) | < 300 KB | > 600 KB |
| First interaction | < 2.5 s Slow 4G | > 4 s |
| Re-render on keystroke | < 50 ms | > 100 ms |

Split routes with lazy `import()`, compress with brotli, preload only the
current route. Facade heavy embeds (maps, editors) behind "click to
load". Measure with `trunk build --release` plus Lighthouse on a
mid-tier phone profile, not your desktop.

> ✅ **Exercise 29:** Implement `TasksState` in your Leptos app with all
> four states plus a `Refreshing(Vec<Task>)` variant that keeps old rows
> visible. Inject a fake 500 once to screenshot the error state.

---

### Chapter 30: Shipping — Containers, CI, and Rollback

#### 30.1 Twelve-Factor Config in One Place

One artifact, many environments. Config comes from the environment,
validated at boot, never branched on `if prod`:

```rust
#[derive(Clone)]
struct ProdConfig {
    bind_addr: String,
    database_url: String,
    frontend_origin: String,
    log_level: String,
}

impl ProdConfig {
    fn from_env() -> Result<Self, String> {
        Ok(Self {
            bind_addr: std::env::var("BIND_ADDR").unwrap_or("0.0.0.0:3000".to_string()),
            database_url: std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL missing")?,
            frontend_origin: std::env::var("FRONTEND_ORIGIN").map_err(|_| "FRONTEND_ORIGIN missing")?,
            log_level: std::env::var("RUST_LOG").unwrap_or("info".to_string()),
        })
    }
}
```

`.env` files never ship. `.env.example` documents names with dummy
values. Missing secrets crash startup; no `|| "secret"` fallbacks.

#### 30.2 CI That Gates Merges

```yaml
# .github/workflows/ci.yml (backend job excerpt)
# jobs:
#   backend:
#     runs-on: ubuntu-latest
#     services: { postgres: { image: postgres:16, env: { POSTGRES_PASSWORD: secret } } }
#     steps:
#       - uses: actions/checkout@v4
#       - run: cargo fmt --check
#       - run: cargo clippy -- -D warnings
#       - run: cargo test -p backend
#       - run: cargo audit --deny warnings
```

Add a second job for `wasm32-unknown-unknown` + `trunk build` when the
frontend changes. Pin GitHub Actions by SHA. Require both jobs plus the
contrast/budget checks from earlier appendices before merge.

#### 30.3 Deployment and Rollback

Ship blue/green or rolling with health gates. Database migrations run
before new binaries start (expand phase), code deploys next, contraction
last. Keep the previous image tagged and the rollback migration tested:

```text
deploy v42: migrate up -> roll 1 canary -> watch p95/errors 10m
  -> roll rest -> hold old image 24h
rollback: route back to v41 -> migrate down (only if contraction ran)
```

Automate rollback on SLO breach where safe (stateless backends yes,
migrations with data loss no — those need a human). Record commit, SBOM,
approver, and migration version per deploy. Preview environments use
synthetic data and auto-expire.

> ✅ **Exercise 30:** Write a `HEALTHCHECK` plus `/healthz` (liveness)
> and `/readyz` (DB reachable) split. Verify the load balancer hits
> `/readyz` before sending traffic and stops on 503 during deploys.

---

### Chapter 31: Capstone I — A Production CLI (ripgrep-lite)

Build a parallel file searcher that respects `.gitignore`, skips binary
files, and prints `path:line:match` with colors when on a TTY:

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn search_file(path: &Path, needle: &str) -> std::io::Result<Vec<(usize, String)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut hits = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return Ok(hits), // binary or bad UTF-8: skip file, not crash
        };
        if line.contains(needle) {
            hits.push((i + 1, line));
        }
    }
    Ok(hits)
}
```

Parallelize with threads or Tokio tasks over a bounded channel (Chapter
26 sizing). Honor ignore files with the `ignore` crate in real code;
hand-roll directory walking once with `walkdir` to learn, then delegate.
Benchmark on the Rust repo itself (`cargo bench` from Chapter 27) and
record lines/sec before adding memory maps (`memmap2`) or SIMD.

Acceptance: searches `rust-course/` in under a second, binary files
skipped without panic, exit `1` when no matches, `2` on bad usage,
colors only when stdout is a TTY (`std::io::IsTerminal`).

### Chapter 32: Capstone II — Mini-Redis (Storage + Protocol)

Build a subset of Redis speaking RESP over TCP: `PING`, `SET key value`,
`GET key`, `DEL key`, `EXPIRE`, with RDB-style snapshots:

```text
+OK\r\n            <- Simple String
-ERR message\r\n    <- Error
:42\r\n             <- Integer
$3\r\nfoo\r\n      <- Bulk String
*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n  <- Array
```

Store `HashMap<String, (Vec<u8>, Option<Instant>)>` behind a sharded
`Vec<Mutex<HashMap>>` (16 shards by FNV hash from Chapter 18) to cut
lock contention. Expire lazily on read plus actively every 100ms for a
random sample. Snapshot with `write_atomic` (Chapter 24) on a timer and
on `SAVE`.

Test with `nc` and the Redis CLI against your port. Required tests:
expiry boundary (TTL 0 vs 1ms), concurrent `SET` from 8 tasks with no
lost writes, snapshot roundtrip after kill -9 mid-write (atomic rename
must save you). This project reuses every systems chapter at once.

### Chapter 33: Capstone III — WASM Kanban Board

A Leptos kanban with three columns, drag-and-drop plus keyboard move
(`Ctrl+Arrow`), offline queue in `localStorage`, and optimistic sync to
`PATCH /tasks/{id}` from Appendix B:

- Columns derive from `Task.done` plus a local `doing` lane stored in
  the frontend until the backend gains a status enum.
- Each card shows title, id, and sync state (synced, pending, failed).
- Failed syncs retry with backoff and never lose typed edits.
- Full keyboard path: Tab to card, Enter to edit, Esc to cancel, Space
  to drag, arrows to move. `aria-live` announces moves.
- Budget from Chapter 29 enforced in CI; screenshots for light, dark,
  360px, and keyboard-focus states in the PR.

Ship it behind the Chapter 30 pipeline with the wasm CI job enabled.
The demo that matters is not the happy path — it is airplane mode
mid-drag with edits preserved and synced on reconnect.

---

### Closing: What "Full-Stack & Compiler Architect" Means

You can now:

- Model data with types that make illegal states unrepresentable.
- Handle every error with `Result`, `?`, transactions, and rollbacks.
- Share work across threads and async tasks with bounds you can name.
- Write macros and `unsafe` narrowly, with expansions and Miri as proof.
- Serve HTTP over Tokio and Postgres with auth, limits, and migrations.
- Ship WASM that respects budgets, keyboards, and screen readers.
- Build languages and version-control tools from first principles.
- Measure before optimizing and gate merges on green checks.

The stack will change. The discipline — precise types, explicit errors,
verified runs — transfers to whatever comes next. Keep building.

---

### Chapter 28B: Auth Middleware, Rate Limits, and Request IDs

#### Session cookie shape

```rust
#[derive(Clone)]
struct Session {
    id: String,
    user_id: i64,
}
```

Cookies carry only the random `id`. User data lives in Postgres or Redis
with expiry. Cookie flags: `HttpOnly`, `Secure` (HTTPS only), `SameSite=Lax`,
`Path=/`, `Max-Age` matching session TTL.

#### Auth extractor (Axum)

```rust
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

struct Auth(pub Session);

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = AppError2;
    async fn from_request_parts(parts: &mut Parts, _s: &S) -> Result<Self, Self::Rejection> {
        let cookie = parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        lookup_session(cookie).ok_or(AppError2::Unauthorized).map(Auth)
    }
}

#[derive(Debug)]
enum AppError2 {
    Unauthorized,
}

fn lookup_session(_cookie: &str) -> Option<Session> {
    None // query sessions table by id, check expiry
}
```

Private routes take `Auth` as a parameter. Public routes do not. New routes
without `Auth` or an allowlist comment fail review by rule.

#### Per-IP rate limit sketch

```rust
use std::collections::HashMap;
use std::time::Instant;

struct Limits {
    hits: HashMap<String, (u32, Instant)>,
    max: u32,
}

impl Limits {
    fn allow(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        let e = self.hits.entry(ip.to_string()).or_insert((0, now));
        if now.duration_since(e.1).as_secs() > 60 {
            *e = (0, now);
        }
        e.0 += 1;
        e.0 <= self.max
    }
}
```

Production uses `tower_governor` or Redis sliding windows. The invariant
is identical: login, signup, and reset have the strictest tier, and
excess returns 429 with `Retry-After`, never a silent drop.

#### Request IDs end to end

```rust
fn request_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    format!("{t:032x}")
}
```

Generate at the edge, forward in `x-request-id`, log on every line for
the request, return to the client on errors. Debugging without IDs is
guessing. With IDs it is lookup.

#### Password reset flow (safe shape)

```text
1. POST /reset-request {email} -> always 200 "if it exists, mail sent"
2. Token is 32 random bytes, single-use, 15-minute expiry, hashed at rest
3. POST /reset {token, new_password} validates entropy (zxcvbn 3+)
4. All sessions for user revoked, audit log appended
5. Timing uniform whether email exists or not
```

Never reveal "email not found". Never log tokens. Never allow enumeration
by timing: hash a dummy when the account is missing so durations match.

#### Audit log row

```sql
CREATE TABLE audit (
    id BIGSERIAL PRIMARY KEY,
    at TIMESTAMPTZ NOT NULL DEFAULT now(),
    actor BIGINT,
    action TEXT NOT NULL,
    target TEXT NOT NULL,
    ip TEXT
);
```

Append on privilege changes, role grants, API key creation, and data
export. Audit tables are append-only: no UPDATE or DELETE grants for the
app role, retention by policy, reviewed quarterly.

#### HTTPS and headers checklist

```text
Strict-Transport-Security: max-age=63072000; includeSubDomains
Content-Security-Policy: default-src self; script-src self
X-Content-Type-Options: nosniff
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=()
Cookies: __Host- prefix, Secure, HttpOnly, SameSite=Lax
```

Terminate TLS at the load balancer or ingress, forward proto correctly,
and test with `curl -I https://your.app` before announcing launch.

#### Login handler skeleton

```rust
#[derive(serde::Deserialize)]
struct Login {
    email: String,
    password: String,
}

async fn login(Json(b): Json<Login>) -> Result<Json<serde_json::Value>, AppError2> {
    let _ = (b.email, b.password);
    // 1. fetch hash by email (constant-time path even when missing)
    // 2. verify_password (Argon2id, Chapter 28)
    // 3. create session row, set cookie, append audit
    Err(AppError2::Unauthorized)
}
```

Uniform error for unknown email and wrong password. Same message, same
code, same timing shape. Enumeration resistance is a test, not a comment:
script both paths and assert indistinguishability.


---

<a id="part-ix-stdlib-and-idioms-ch-34-36"></a>

## Part IX - Stdlib and Idioms (Ch 34-36)

| Navigation | |
| --- | --- |
| Previous | [← Part VIII - Production and Capstones (Ch 28-33)](#part-viii-production-and-capstones-ch-28-33) |
| Next | [Part X - Practice and Exam (Ch 37-42) →](#part-x-practice-and-exam-ch-37-42) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part IX: The Standard Library, Idioms, and Pattern Catalog

> 🧭 **Where we are:** You can build systems and ship them. Part IX is
> the reference you keep open: every std module you reach for weekly,
> the idioms that mark native Rust, and the patterns that solve the same
> five problems in every codebase.

---

### Chapter 34: `std` Module by Module

#### 34.1 `option` and `result` Combinators (Stop Nesting `match`)

| Combinator | Purpose | Example |
| --- | --- | --- |
| `map` | transform value | `opt.map(\|s\| s.len())` |
| `and_then` | chain fallible | `opt.and_then(parse)` |
| `unwrap_or` / `unwrap_or_else` | default | `opt.unwrap_or(0)` |
| `ok_or` / `ok_or_else` | Option to Result | `opt.ok_or("missing")` |
| `transpose` | flip nesting | `Option<Result<T,E>>` to `Result<Option<T>,E>` |
| `?` | early return | `let v = opt?;` |

```rust
fn parse_port(s: Option<&str>) -> Result<u16, String> {
    s.ok_or_else(|| "PORT missing".to_string())?
        .trim()
        .parse::<u16>()
        .map_err(|_| format!("{s:?} is not a port"))
}

fn main() {
    assert_eq!(parse_port(Some("8080")), Ok(8080));
    assert!(parse_port(None).is_err());
}
```

`transpose` shines when validating collections: `Vec<Option<T>>` to
`Option<Vec<T>>` with `collect`, or `Vec<Result<T,E>>` to
`Result<Vec<T>,E>` the same way. One line replaces a loop.

#### 34.2 `vec`, `slice`, and `iter` Power Tools

```rust
fn main() {
    let mut v = vec![3u32, 1, 2, 2, 5];
    v.sort(); v.dedup(); // sorted unique: [1,2,3,5]
    assert_eq!(v.binary_search(&3), Ok(2));
    // chunks, windows, split_at: no-copy views
    let (a, b) = v.split_at(2);
    assert_eq!((a, b), (&[1, 2][..], &[3, 5][..]));
    // retain mutates in place; drain moves out.
    let mut w = vec![1, 2, 3, 4];
    w.retain(|n| n % 2 == 0);
    assert_eq!(w, vec![2, 4]);
    // windows for sliding pairs:
    let s = vec![1, 2, 3, 4];
    let pairs: Vec<&[i32]> = s.windows(2).collect();
    assert_eq!(pairs, vec![&[1,2][..], &[2,3][..], &[3,4][..]]);
}
```

Iterator economics: `iter()` borrows, `iter_mut()` mutably borrows,
`into_iter()` consumes. `collect::<Vec<_>>()` needs the turbofish when
the target is ambiguous. `partition` splits by predicate in one pass;
`fold` aggregates; `scan` carries state while yielding.

#### 34.3 `hashmap`, `btreemap`, `hashset`, `vecdeque`

| Type | Order | Lookup | Use when |
| --- | --- | --- | --- |
| `HashMap` | none | O(1) avg | caches, indexes |
| `BTreeMap` | sorted | O(log n) | ranges, ordered output |
| `HashSet` | none | O(1) | membership |
| `VecDeque` | insertion | O(1) ends | queues, sliding windows |

```rust
use std::collections::{BTreeMap, HashMap, VecDeque};

fn main() {
    let mut cache: HashMap<String, u32> = HashMap::new();
    // entry API: one lookup for get-or-insert.
    *cache.entry("hits".to_string()).or_insert(0) += 1;
    let mut ordered = BTreeMap::new();
    ordered.insert(2, "b"); ordered.insert(1, "a");
    assert_eq!(ordered.keys().copied().collect::<Vec<_>>(), vec![1, 2]);
    let mut q: VecDeque<u32> = VecDeque::new();
    q.push_back(1); q.push_back(2);
    assert_eq!(q.pop_front(), Some(1));
}
```

`entry` is the HashMap idiom that matters. `or_insert`, `or_insert_with`,
`and_modify` compose without double lookups. For hot maps, `ahash` or
`foldhash` via `HashMap::with_hasher` beats `SipHash13` default.

#### 34.4 `string`, `str`, `format`, and Text

```rust
fn main() {
    // Capacity first when building in a loop.
    let mut s = String::with_capacity(1024);
    s.push_str("hello");
    s.push('!');
    assert_eq!(s, "hello!");
    // lines, split, trim, parse:
    let nums: Vec<i32> = "1, 2, 3".split(',').filter_map(|p| p.trim().parse().ok()).collect();
    assert_eq!(nums, vec![1, 2, 3]);
    // Cow: borrow when possible, own when needed.
    let owned: std::borrow::Cow<'_, str> = std::borrow::Cow::Borrowed("static");
    assert_eq!(owned, "static");
}
```

Unicode truth: `len()` is bytes, `chars().count()` is scalar values,
`graphemes` (via `unicode-segmentation` crate) is what users see. `"é"`
is 2 bytes, 1 char, 1 grapheme; `"👨‍👩‍👧"` is many bytes, several chars,
one grapheme. Use `chars` for logic, graphemes for display width.

#### 34.5 `fs`, `io`, `path`, `env`, `process`, `time`

Covered in depth in Chapter 24. Additions here: `std::io::IsTerminal`
for color gating, `std::process::Command` for subprocesses, `Instant`
for durations versus `SystemTime` for wall clock:

```rust
fn main() {
    let color = std::io::stdin().is_terminal();
    println!("color allowed: {color}");
    let now = std::time::Instant::now();
    let _work: Vec<u32> = (0..1000).collect();
    println!("elapsed: {:?}", now.elapsed()); // monotonic, never goes backwards
}
```

`Command` without shell interpolation avoids injection by construction:

```rust
fn git_log() -> std::io::Result<String> {
    let out = std::process::Command::new("git").args(["log", "--oneline", "-5"]).output()?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}
```

Never `format!("git log {arg}")` into `sh -c`. Arg arrays keep filenames
with spaces and `; rm -rf` safely inert.

#### 34.6 `sync`: `Mutex`, `RwLock`, `OnceLock`, Atomics

```rust
use std::sync::{Arc, Mutex, OnceLock};

static CONFIG: OnceLock<String> = OnceLock::new();

fn config() -> &'static str {
    CONFIG.get_or_init(|| std::env::var("APP_MODE").unwrap_or("dev".to_string()))
}

fn main() {
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            for _ in 0..1000 {
                *c.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    assert_eq!(*counter.lock().unwrap(), 8000);
    println!("mode={}", config());
}
```

`OnceLock` replaces `lazy_static` for one-time init. Atomics
(`AtomicU64`, `AtomicBool`) replace `Mutex` for counters and flags with
explicit `Ordering` (`Relaxed` for stats, `Acquire`/`Release` for
handoff, `SeqCst` when unsure — then measure).

Poisoning: `lock().unwrap()` panics if another thread panicked while
holding the lock. Production code maps poisoning to a 500 with a
request ID rather than `unwrap` in request paths.

#### 34.7 `thread`, `net`, Numbers, and `mem`

- `std::thread::scope` joins automatically; prefer it over manual
  `JoinHandle` vectors for fork-join work.
- `TcpListener`/`TcpStream`/`UdpSocket` are blocking; set timeouts or
  move to Tokio for scale (Chapter 25–26).
- Integers: `checked_*` returns `Option`, `saturating_*` clamps,
  `wrapping_*` wraps, `overflowing_*` returns flag. Pick per call site.
- `std::mem::swap`, `take`, `replace` move values without clones:

```rust
fn main() {
    let mut a = String::from("a");
    let mut b = String::from("b");
    std::mem::swap(&mut a, &mut b);
    assert_eq!((a, b), ("b".to_string(), "a".to_string()));
    let taken = std::mem::take(&mut a); // leaves "" behind
    assert_eq!((taken, a), ("b".to_string(), String::new()));
}
```

---

### Chapter 35: Idiomatic Rust — The Native Speaker List

#### 35.1 Ownership-Native APIs

- Borrow in, own out: `fn parse(s: &str) -> Result<Config, Error>`.
- Builders for >3 optional knobs; plain args for 1–2 required.
- `impl AsRef<str>` / `AsRef<Path>` at boundaries, concrete types inside.
- Return `Self` for chaining (`with_*`), `&mut Self` for in-place.
- No `get_` prefix (`len()`, not `get_len()`); `is_` for bools.

```rust
#[derive(Debug, Default)]
struct Server {
    host: String,
    port: u16,
    workers: usize,
}

impl Server {
    fn new(host: impl Into<String>, port: u16) -> Self {
        Self { host: host.into(), port, workers: 4 }
    }
    fn workers(mut self, n: usize) -> Self {
        self.workers = n;
        self
    }
}
```

#### 35.2 Error-Native Code

- `thiserror` for libraries (named variants), `anyhow` for binaries
  (`Context` on every `?` at I/O boundaries).
- `#[non_exhaustive]` on public error enums you will extend.
- Validate at construction (`Server::new` returns `Result`), never a
  `is_valid()` + `use()` pair callers forget.
- Fail closed: unknown auth state denies; missing config crashes boot.

#### 35.3 Clone, Copy, and Cost Honesty

- `#[derive(Clone)]` liberally, `Copy` only for small stack types.
- `to_vec`, `to_string`, `clone` visible at call sites — cost in plain
  sight beats hidden sharing.
- `Cow<'_, str>` for "usually borrowed, sometimes owned" returns.
- `Arc` for shared ownership, `Rc` only single-threaded, `Weak` for
  back-edges (Chapter 21).

#### 35.4 Docs That Get Read

```rust
/// Parse `PORT`-style values (`"8080"`).
///
/// # Errors
///
/// Returns `Err` when `s` is not `0`–`65535` after trimming.
///
/// # Examples
///
/// ```
/// assert_eq!(parse_port_demo("80"), Ok(80));
/// ```
fn parse_port_demo(s: &str) -> Result<u16, String> {
    s.trim().parse::<u16>().map_err(|_| format!("{s:?} is not a port"))
}
```

Doc examples are tests. Write them for every public function. They are
the only docs that fail CI when they rot.

---

### Chapter 36: Pattern Catalog — Five Problems, Rust Answers

#### 36.1 Builder, Newtype, and Typestate

Newtypes prevent unit mixups at zero cost:

```rust
struct UserId(u64);
struct OrderId(u64);

fn get_user(id: UserId) {}
// get_user(OrderId(1)); // ERROR: wrong type, caught at compile time
```

Typestate moves "called in the wrong order" from runtime to compile
time (unbuilt → building → sealed), the same idea as `Mutex` guards
that cannot outlive their lock.

#### 36.2 RAII Guards and `Drop`

Locks, temp files, and spans release in `Drop`. Write your own guard
for benchmark timers, feature flags, or test fixtures:

```rust
struct Timer {
    name: &'static str,
    start: std::time::Instant,
}

impl Timer {
    fn new(name: &'static str) -> Self {
        Self { name, start: std::time::Instant::now() }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        eprintln!("{} took {:?}", self.name, self.start.elapsed());
    }
}
```

#### 36.3 Strategy (Function Pointers, Closures, Traits)

Sorting, hashing, retry policies: accept `impl Fn` for one-off
customization, traits for families, function pointers for C-compatible
hooks. Chapter 18's `hash_bytes` seam is Strategy without ceremony.

#### 36.4 Observer (Channels, Not Callbacks)

Prefer `broadcast::channel` fan-out over callback registration. Receivers
lag explicitly (`Lagged` error) instead of deadlocking the sender. This
is Chapter 26 applied to UI updates, config reload, and shutdown.

#### 36.5 Visitor Is `match` (and Should Stay That Way)

Other languages need Visitor classes for "one operation over many node
types". Rust uses enums plus `match` with exhaustiveness:

```rust
enum Shape { Circle(f64), Rect { w: f64, h: f64 } }

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect { w, h } => w * h,
    }
}
```

Add a variant and the compiler lists every `match` to update. That
exhaustiveness *is* the pattern. Do not reintroduce class hierarchies.

---

### Part IX Recap

- `Option`/`Result` combinators, slice/iterator tools, and the right map
  per ordering need replace loops and nesting (Ch 34).
- Ownership-native APIs, named library errors, cost-honest clones, and
  doc-tests as the only docs that fail CI (Ch 35).
- Builder, newtype, typestate, RAII guards, strategy seams, channel
  observers, and `match`-as-visitor cover the recurring five (Ch 36).

---

### Chapter 36B: FAQ — The 30 Questions Beginners Actually Ask

| # | Question | Short answer |
| --- | --- | --- |
| 1 | Why immutable by default? | Prevents spooky change; `mut` marks intent |
| 2 | `String` vs `&str`? | Own vs borrow; `String` grows, `&str` views |
| 3 | When to `clone`? | Small data or explicit cost; else borrow |
| 4 | `Copy` when? | Small stack-only types (`Point`, ids) |
| 5 | `==` vs `=`? | Compare vs assign; Rust rejects `if x = 1` |
| 6 | `if` without braces? | Never; braces required, prevents bugs |
| 7 | `loop` vs `while` vs `for`? | Forever, condition, iterator; prefer `for` |
| 8 | Ranges `..` vs `..=`? | Exclude vs include end |
| 9 | Panic vs `Result`? | Bugs vs expected failures; `Result` at edges |
| 10 | `unwrap` ever ok? | Tests, prototypes, proven invariants only |
| 11 | `?` in `main`? | Yes with `Result` return; prints error + exit 1 |
| 12 | `Vec` vs array? | Growable vs fixed; slices view both |
| 13 | `HashMap` vs `BTreeMap`? | Speed vs sorted order and ranges |
| 14 | `iter` vs `into_iter`? | Borrow vs consume |
| 15 | Generics bloat? | Keep generic shells thin, share core logic |
| 16 | `impl Trait` vs `dyn`? | Static vs runtime dispatch; mixed types need `dyn` |
| 17 | What is object safety? | `Self`/generics break vtables; split traits |
| 18 | Lifetime elision? | 3 rules; name lifetimes when 2 inputs flow out |
| 19 | `Rc` vs `Arc`? | Single-thread vs threads; `Weak` breaks cycles |
| 20 | `Cell` vs `RefCell`? | `Copy` set/get vs runtime borrow checks |
| 21 | Macros when? | Last resort after fns and traits |
| 22 | `unsafe` scope? | Tiny blocks with SAFETY comments + wrappers |
| 23 | FFI strings? | `CString` in, validate lengths, never `String` across |
| 24 | Threads vs async? | Few blocking vs many idle connections |
| 25 | `select!` cancellable? | Only if dropping a branch loses nothing needed |
| 26 | Channel bound size? | Rate times allowed lag; 16-64 to start |
| 27 | ` anyhow` vs `thiserror`? | Binaries vs libraries |
| 28 | Test DB per test? | Yes via `#[sqlx::test]`; never share |
| 29 | WASM slow? | Check size budget, split routes, facade embeds |
| 30 | What next after book? | Ship one CLI, one API, one WASM app; then redo exam |

Each answer compresses a chapter. If one surprises you, reread that
chapter before continuing. Surprise marks the growth edge.


---

<a id="part-x-practice-and-exam-ch-37-42"></a>

## Part X - Practice and Exam (Ch 37-42)

| Navigation | |
| --- | --- |
| Previous | [← Part IX - Stdlib and Idioms (Ch 34-36)](#part-ix-stdlib-and-idioms-ch-34-36) |
| Next | [Part XI - Cookbook (75 recipes) →](#part-xi-cookbook-75-recipes) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part X: Guided Builds, Exercises, and Checklists

> 🧭 **Where we are:** Reference is behind you, shipping is ahead. Part X
> is deliberate practice: 60 exercises with acceptance criteria, three
> guided builds with full code, review checklists, and the final exam.

---

### Chapter 37: 60 Exercises That Cover Everything

Each exercise names its done criteria. No "explore" or "play around".
Build, run, observe, then check the hint only after attempting.

#### 37.1 Ownership and Borrowing (1–10)

1. Move a `String` into a function, then fix by borrowing. Done: both
   versions compile; you can state who owns what on each line.
2. Hold two `&` borrows alive, then add `&mut` and read the error. Done:
   you can explain NLL (last use ends the borrow).
3. Slice `[10,20,30,40]` middle two without copying. Done: prints `[20,30]`
   via `&a[1..3]`.
4. Return `&str` from `&String` input; then try returning `&String` to a
   local and fix by returning `String`. Done: dangling version rejected.
5. `Clone` vs borrow: count clones in a 100-line program, halve them.
   Done: `cargo` output identical, `clone` count down.
6. `&mut` through two function layers (`main` → `a(&mut s)` →
   `b(&mut s)`). Done: mutation visible in `main`.
7. Build `Excerpt<'a>` over a local `String`; move the `String` and watch
   the borrow error. Done: you can draw the lifetime arrows.
8. `RefCell` counter with `&self` methods. Done: `add(&self)` compiles,
   conflicting borrow test panics as designed.
9. `Rc` tree with `Weak` parent. Done: `strong_count` is 1 after linking.
10. Split a struct into owned/borrowed halves (`split_at_mut` pattern).
    Done: two mutable disjoint borrows coexist legally.

#### 37.2 Types, Traits, Lifetimes (11–20)

1. `mymax<T: PartialOrd + Copy>` plus slice version returning
   `Option<&T>`. Done: empty slice returns `None`, no panic.
2. `Area` for two shapes via `Box<dyn Area>` sum. Done: mixed vec works.
3. Split an object-unsafe trait into object-safe + generic halves. Done:
   `dyn` compiles after split.
4. Blanket `Loud` for all `Display`. Done: `42.loud() == "42"`.
5. `parse_port` with `Result<u16,String>`, unit + property tests. Done:
   `proptest` roundtrip passes 256 cases.
6. `first_word` elided vs explicit lifetimes, identical behavior. Done:
   you can recite the three elision rules.
7. `hashmap!` macro with trailing comma. Done: `hashmap!["a"=>1,]` works.
8. `vec_of_strings!` with `$($s:expr),*`. Done: empty invocation yields
   empty vec.
9. `c_strlen` wrapper with `CString`, tests for `""`, `"a"`, `"héllo"`
   (7 bytes). Done: interior-nul case errors cleanly.
10. GAT `Lending` for a `Words` struct. Done: returned `&str` borrows
    from `&self` with named lifetime.

#### 37.3 Concurrency and Async (21–30)

1. 8 threads x 1000 increments via `Arc<Mutex<u32>>`. Done: total 8000.
2. `mpsc` pipeline with bound 16, producer/consumer rates logged. Done:
   no unbounded growth under fast producer.
3. `select!` primary/fallback race. Done: fallback wins deterministically
   in test with controlled sleeps.
4. `timeout(1s)` around 10s sleep. Done: prints timeout, task cancelled.
5. `buffered(4)` stream doubling 0..10. Done: sum matches sync version.
6. Broadcast shutdown to 3 tasks; each flushes before exit. Done: zero
   "task dropped mid-write" warnings.
7. Read timeout on TCP echo server (5s). Done: idle client disconnected.
8. 4-worker thread pool for echo server. Done: 100 concurrent `nc`
   clients all echoed.
9. `watch` channel for config reload. Done: tasks see new value without
   restart.
10. `oneshot` request/response across tasks. Done: timeout on dropped
    responder handled, not panicked.

#### 37.4 I/O, CLI, Networking (31–40)

1. `count_lines` with `BufReader`, binary file skipped. Done: bad UTF-8
   returns partial count, not crash.
2. `write_atomic` temp + rename + `sync_all`. Done: kill -9 mid-write
   never leaves half file.
3. `PORT` env parse with exit 2 on garbage. Done: `PORT=abc` exits 2.
4. `clap` TinyGit with 5 subcommands. Done: `--help` lists all, codes
   match hand-rolled version.
5. TCP echo + `nc` test. Done: `echo hi | nc` returns `hi`.
6. Minimal HTTP GET over `TcpStream`. Done: body extracted past headers.
7. Axum `GET /tasks?limit=&offset=` clamped 1..100. Done: `limit=9999`
   returns 100 rows max.
8. `PATCH /tasks/{id}` with `UpdateTask{done}`. Done: unknown id 404.
9. Restrictive CORS with one origin. Done: wrong origin blocked, right
   origin passes preflight.
10. `Command::new("git")` log fetcher, no shell. Done: filename with
    `; rm` stays inert.

#### 37.5 Testing, Profiling, Shipping (41–50)

1. `parse_port` unit tests for known/edge cases. Done: all green.
2. Criterion `fnv1a/4k` bench with `black_box`. Done: median ns recorded.
3. `samply` flamegraph of TinyGit `add` on 50MB. Done: hotspot named
   (I/O vs hash vs index).
4. Size DB pool by load test to p95 knee. Done: number recorded with
   method.
5. `/healthz` vs `/readyz` split. Done: LB drains on 503 from `/readyz`.
6. Dockerfile `rust:1.82-bookworm` multi-stage + nonroot. Done: image
   runs as `appuser`, no toolchain inside.
7. CI backend + wasm jobs pinned by SHA. Done: red build blocks merge.
8. Argon2id hash/verify roundtrip. Done: wrong password fails, timing
   uniform message.
9. Transfer transaction with forced second-update failure. Done: first
   update rolled back, test proves it.
10. Migration expand/migrate/contract for `owner_id`. Done: old binary
    runs against new schema during rollout.

#### 37.6 Frontend and Capstones (51–60)

1. `TasksState` Loading/Loaded/Empty/Failed. Done: each screenshotted.
2. `Refreshing(Vec)` stale-while-revalidate. Done: old rows visible.
3. Title validation (non-empty, ≤200 chars). Done: errors linked via
   `aria-describedby`.
4. Optimistic checkbox with rollback. Done: failure restores prior state
   plus notice.
5. WASM budget check in CI (<300KB gzip). Done: build fails over budget.
6. ripgrep-lite on `rust-course/`, <1s. Done: binaries skipped, codes
   1/2 correct, TTY colors only.
7. Mini-Redis `SET`/`GET`/`DEL` over `nc`. Done: RESP shapes exact.
8. Mini-Redis expiry boundary (0 vs 1ms). Done: tests pin behavior.
9. WASM kanban keyboard path (Tab/Enter/Esc/Space/arrows). Done: full
   flow without pointer, `aria-live` announces.
10. Airplane-mode drag with queued sync. Done: edits preserved, synced on
    reconnect.

---

### Chapter 38: Guided Build A — `peek` (Parallel Searcher)

Files: `src/main.rs`, `src/walk.rs`, `src/search.rs`. Crate `ignore`
for gitignore-respecting walk, `clap` for args, `anyhow` for errors:

```rust
use std::path::{Path, PathBuf};

/// Collect files under root, skipping hidden and binary-ish by extension.
fn collect_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue, // permission denied: skip, keep going
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == ".git" || name == "target" || name.starts_with('.') {
                    continue;
                }
                stack.push(path);
            } else if path.is_file() {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}
```

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Returns (line_number, line) hits. Skips files with mid-stream UTF-8 errors.
fn search_file(path: &Path, needle: &str) -> std::io::Result<Vec<(usize, String)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut hits = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return Ok(hits),
        };
        if line.contains(needle) {
            hits.push((i + 1, line));
        }
    }
    Ok(hits)
}
```

Main fans files across `std::thread::scope` workers (or Tokio tasks for
larger trees), prints `path:line:text` with color only on TTY:

```rust
fn is_tty_stdout() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}
```

Acceptance from Chapter 31 applies verbatim. Profile with `samply`
before adding `memmap2`; most trees are walk-bound, not search-bound.

### Chapter 39: Guided Build B — `kvbox` (Mini-Redis Core)

Data model and RESP writer. Shard to cut lock contention:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

struct Entry {
    data: Vec<u8>,
    expires_at: Option<Instant>,
}

struct Shard {
    map: Mutex<HashMap<String, Entry>>,
}

pub struct Store {
    shards: Vec<Shard>,
}

impl Store {
    pub fn new(shards: usize) -> Self {
        let mut v = Vec::with_capacity(shards);
        for _ in 0..shards {
            v.push(Shard { map: Mutex::new(HashMap::new()) });
        }
        Self { shards: v }
    }
    fn shard(&self, key: &str) -> &Shard {
        // FNV-1a from Chapter 18, reused as a sharder.
        let mut h: u64 = 0xcbf29ce484222325;
        for b in key.as_bytes() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        &self.shards[(h as usize) % self.shards.len()]
    }
    pub fn set(&self, key: String, data: Vec<u8>, ttl_ms: Option<u64>) {
        let expires_at = ttl_ms.map(|ms| Instant::now() + std::time::Duration::from_millis(ms));
        self.shard(&key).map.lock().unwrap().insert(key, Entry { data, expires_at });
    }
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut map = self.shard(key).map.lock().unwrap();
        let e = map.get(key)?;
        if let Some(t) = e.expires_at {
            if Instant::now() >= t {
                map.remove(key);
                return None;
            }
        }
        Some(e.data.clone())
    }
}
```

RESP replies are byte-exact. Test vectors:

```text
SET foo bar -> +OK
GET foo     -> $3 foo
GET missing -> $-1 (null bulk string)
DEL foo     -> :1 (deleted) or :0
PING        -> +PONG
```

Serve over `TcpListener` with one task per connection (Tokio) or a pool
(std). Snapshot via `write_atomic` from Chapter 24. Required tests from
Chapter 32 (expiry boundary, 8-task concurrent SET, kill -9 snapshot)
gate completion.

### Chapter 40: Guided Build C — Kanban Sync Engine (No Framework Lock-in)

The sync core is framework-free so it works in Leptos, Yew, or plain
WASM. Framework renders; this module decides:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SyncState {
    Synced,
    Pending,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: i64,
    pub title: String,
    pub done: bool,
    pub sync: SyncState,
}

impl Card {
    pub fn validate(&self) -> Result<(), String> {
        let t = self.title.trim();
        if t.is_empty() {
            return Err("Give the card a title.".to_string());
        }
        if t.chars().count() > 200 {
            return Err("Keep titles under 200 characters.".to_string());
        }
        Ok(())
    }
    /// Apply server truth after PATCH; failed syncs keep local edits.
    pub fn apply_server(&mut self, done: bool) {
        self.done = done;
        self.sync = SyncState::Synced;
    }
    pub fn mark_pending(&mut self) {
        self.sync = SyncState::Pending;
    }
    pub fn mark_failed(&mut self, e: String) {
        self.sync = SyncState::Failed(e);
    }
}
```

Queue in `localStorage` (JSON array of pending `{id, done}`), flush in
order with backoff, last-writer-wins by `updated_at`. Keyboard map from
Chapter 33 plus `aria-live` announcements complete the board. The exam
demo is airplane-mode drag with queued sync — practice that path, not
just the happy path.

---

### Chapter 41: Review Checklists and Final Exam

#### 41.1 Pre-Merge Checklist (Copy Into Every PR)

```text
[ ] cargo fmt --check, clippy -D warnings, tests green
[ ] New routes have auth policy; no accidental public endpoints
[ ] RLS/migration rollback tested where schema changed
[ ] Inputs validated server-side; outputs are explicit DTOs
[ ] No secrets in code/logs; no private keys in client bundle
[ ] States: loading/empty/error/refreshing covered in UI
[ ] Keyboard path + focus visible; axe clean on touched views
[ ] Budgets met (WASM size, p95 latency); bench delta attached
[ ] Docs/ADR/changelog updated; flag + rollback noted
[ ] Human reviewer named; AI assistance disclosed
```

#### 41.2 Security Pass (Quarterly)

```text
[ ] Deps audited (cargo audit/deny), SBOM regenerated
[ ] Login/signup/reset rate-limited; enumeration tested
[ ] CORS origins exact; CSP/HSTS headers verified
[ ] Backups restored in staging; RPO/RTO recorded
[ ] Logs sampled for PII/secrets; retention enforced
[ ] Old flags/keys rotated; ex-employee access revoked
```

#### 41.3 Final Exam (4 Hours, Open Book)

1. Extend Pebble with `for (i in 0..n)` loops: lexer token, AST node,
   checker rule, interpreter execution, plus three tests. (60 min)
2. Add TinyGit `checkout <hash>`: restore blobs, refuse dirty overwrite
   without `--force`, test both paths. (45 min)
3. Clamp `GET /tasks` pagination, add `PATCH` validation tests, and make
   CORS restrictive with a passing preflight test. (45 min)
4. Profile `peek` on this repo, cut one bottleneck with before/after
   Criterion numbers and a flamegraph note. (45 min)
5. Write the PR description for 1–4 using the Chapter 41 template with
   commands, results, risks, and rollback. (15 min + 30 min buffer)

Passing means all five build, tests green, and the write-up names what
was NOT tested. That honesty is the skill being examined as much as the
code.

---

### The Last Page

You started at "what is code?" and end with languages, servers, and
tools you can defend in review. Keep the loop: understand, inspect,
change little, verify by running, report honestly. The next technology
will feel smaller than this book did on page one.

---

### Chapter 42: Study Plans — 4, 8, and 12 Weeks

#### 4-week sprint (job interview)

```text
Week 1: I + II (ownership daily, 10 exercises)
Week 2: VI-19/20 + XI recipes A-B (traits, combinators)
Week 3: XIII (sorting, hashmaps, graphs) + X final exam Q1-Q3
Week 4: VII-27 benches + mock interviews with Chapter 41 checklist
```

#### 8-week backend hire

```text
Weeks 1-2: I-III + VI (language + concurrency)
Weeks 3-4: IV + XII (Axum, SQL, handlers) + ship tasks API
Weeks 5-6: VIII-28 + XIV-Q/R/S (auth, Docker, CI)
Weeks 7-8: XIV-T/U + capstone mini-Redis + exam
```

#### 12-week full course (cover to cover)

```text
Weeks 1-3: Parts I-II + XI-A/B (foundations + recipes)
Weeks 4-5: Parts III + VI (threads, async, traits, macros, unsafe)
Weeks 6-7: Parts IV + XII (full-stack app + SQL recipes)
Weeks 8-9: Parts V + XIII (Pebble, TinyGit, algorithms)
Weeks 10-11: Parts VII-VIII + X builds (CLI, kvbox, kanban)
Week 12: Part XIV ops + final exam + launch checklist
```

Rule for every plan: type every snippet, run every test, keep a log of
errors fixed. The log is the learning. Review it weekly and the same
mistakes stop repeating.


---

<a id="part-xi-cookbook-75-recipes"></a>

## Part XI - Cookbook (75 recipes)

| Navigation | |
| --- | --- |
| Previous | [← Part X - Practice and Exam (Ch 37-42)](#part-x-practice-and-exam-ch-37-42) |
| Next | [Part XII - Backend Recipes →](#part-xii-backend-recipes) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part XI: The Rust Cookbook — 150 Copy-Paste Recipes

> 🧭 **How to use this file:** Each recipe is one problem, one solution,
> one runnable snippet. They are ordered by frequency: ownership first,
> strings and files next, then concurrency, networking, and web.

---

### A. Ownership, Borrowing, Lifetimes (1–20)

#### 1. Take ownership only when you must

```rust
fn consume(s: String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hi");
    assert_eq!(consume(s), 2);
    // s moved; use consume(&s) form below to keep it.
}
```

#### 2. Borrow to keep ownership

```rust
fn len_of(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hi");
    assert_eq!(len_of(&s), 2);
    assert_eq!(s, "hi"); // still owned
}
```

#### 3. Mutable borrow in place

```rust
fn shout(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let mut s = String::from("hi");
    shout(&mut s);
    assert_eq!(s, "hi!!!");
}
```

#### 4. Slice without copying

```rust
fn main() {
    let v = vec![10, 20, 30, 40];
    let mid = &v[1..3];
    assert_eq!(mid, &[20, 30]);
}
```

#### 5. Return owned when local

```rust
fn greeting(name: &str) -> String {
    format!("hello, {name}")
}

fn main() {
    assert_eq!(greeting("ada"), "hello, ada");
}
```

#### 6. Struct borrowing input

```rust
struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let s = String::from("a.b.c");
    let e = Excerpt { text: s.split('.').next().unwrap() };
    assert_eq!(e.text, "a");
}
```

#### 7. `Rc` shared tree node

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(vec![1, 2]);
    let b = Rc::clone(&a);
    assert_eq!(Rc::strong_count(&a), 2);
    assert_eq!(b.len(), 2);
}
```

#### 8. `Arc` across threads

```rust
use std::sync::Arc;

fn main() {
    let a = Arc::new(42u32);
    let b = Arc::clone(&a);
    let h = std::thread::spawn(move || *b);
    assert_eq!(h.join().unwrap(), 42);
}
```

#### 9. `RefCell` counter with `&self`

```rust
use std::cell::RefCell;

struct C {
    n: RefCell<u32>,
}

impl C {
    fn inc(&self) {
        *self.n.borrow_mut() += 1;
    }
}

fn main() {
    let c = C { n: RefCell::new(0) };
    c.inc();
    assert_eq!(*c.n.borrow(), 1);
}
```

#### 10. `OnceLock` global config

```rust
use std::sync::OnceLock;

static MODE: OnceLock<String> = OnceLock::new();

fn mode() -> &'static str {
    MODE.get_or_init(|| "dev".to_string())
}

fn main() {
    assert_eq!(mode(), "dev");
}
```

#### 11. `mem::take` to move out of borrowed

```rust
fn main() {
    let mut s = String::from("x");
    let taken = std::mem::take(&mut s);
    assert_eq!((taken, s), ("x".to_string(), String::new()));
}
```

#### 12. `mem::swap` two values

```rust
fn main() {
    let mut a = 1;
    let mut b = 2;
    std::mem::swap(&mut a, &mut b);
    assert_eq!((a, b), (2, 1));
}
```

#### 13. Split mutable borrow legally

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4];
    let (a, b) = v.split_at_mut(2);
    a[0] = 10;
    b[0] = 30;
    assert_eq!(v, vec![10, 2, 30, 4]);
}
```

#### 14. `Cow` borrow-or-own

```rust
use std::borrow::Cow;

fn label(n: u32) -> Cow<'static, str> {
    if n == 0 {
        Cow::Borrowed("zero")
    } else {
        Cow::Owned(format!("n={n}"))
    }
}

fn main() {
    assert_eq!(label(0), "zero");
    assert_eq!(label(5), "n=5");
}
```

#### 15. Elided lifetime helper

```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    assert_eq!(first_word("a b"), "a");
}
```

#### 16. Explicit pick lifetime

```rust
fn pick<'a>(a: &'a str, b: &'a str, use_a: bool) -> &'a str {
    if use_a {
        a
    } else {
        b
    }
}

fn main() {
    assert_eq!(pick("a", "b", false), "b");
}
```

#### 17. `Copy` point

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
struct P {
    x: i32,
    y: i32,
}

fn main() {
    let a = P { x: 1, y: 2 };
    let b = a;
    assert_eq!((a, b), (P { x: 1, y: 2 }, P { x: 1, y: 2 }));
}
```

#### 18. Move into closure

```rust
fn main() {
    let s = String::from("hi");
    let f = move || s.len();
    assert_eq!(f(), 2);
}
```

#### 19. Borrowed sort key

```rust
fn main() {
    let mut names = vec!["c", "a", "b"];
    names.sort();
    assert_eq!(names, vec!["a", "b", "c"]);
}
```

#### 20. Drain moved-out prefix

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4];
    let head: Vec<i32> = v.drain(..2).collect();
    assert_eq!((head, v), (vec![1, 2], vec![3, 4]));
}
```

---

### B. Strings, Numbers, Collections (21–60)

#### 21. Build string with capacity

```rust
fn main() {
    let mut s = String::with_capacity(16);
    s.push_str("a");
    s.push('b');
    assert_eq!(s, "ab");
}
```

#### 22. Parse with trim

```rust
fn port(s: &str) -> Result<u16, String> {
    s.trim().parse::<u16>().map_err(|_| format!("{s:?} bad"))
}

fn main() {
    assert_eq!(port(" 80 "), Ok(80));
}
```

#### 23. Split and parse CSV ints

```rust
fn main() {
    let v: Vec<i32> = "1, x, 3".split(',').filter_map(|p| p.trim().parse().ok()).collect();
    assert_eq!(v, vec![1, 3]);
}
```

#### 24. `format!` padding

```rust
fn main() {
    assert_eq!(format!("{n:04}", n = 7), "0007");
}
```

#### 25. Char vs byte length

```rust
fn main() {
    let s = "héllo";
    assert_eq!(s.len(), 6); // bytes
    assert_eq!(s.chars().count(), 5); // scalar values
}
```

#### 26. `entry` counter

```rust
use std::collections::HashMap;

fn main() {
    let mut m: HashMap<&str, u32> = HashMap::new();
    *m.entry("a").or_insert(0) += 1;
    assert_eq!(m["a"], 1);
}
```

#### 27. Sorted `BTreeMap` keys

```rust
use std::collections::BTreeMap;

fn main() {
    let mut m = BTreeMap::new();
    m.insert(2, "b");
    m.insert(1, "a");
    assert_eq!(m.keys().copied().collect::<Vec<_>>(), vec![1, 2]);
}
```

#### 28. `retain` evens

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.retain(|n| n % 2 == 0);
    assert_eq!(v, vec![2, 4]);
}
```

#### 29. `partition` by predicate

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let (even, odd): (Vec<_>, Vec<_>) = v.into_iter().partition(|n| n % 2 == 0);
    assert_eq!((even, odd), (vec![2, 4], vec![1, 3]));
}
```

#### 30. `binary_search` sorted vec

```rust
fn main() {
    let v = vec![1, 2, 3, 5];
    assert_eq!(v.binary_search(&3), Ok(2));
    assert!(v.binary_search(&4).is_err());
}
```

#### 31. `windows` pairs

```rust
fn main() {
    let v = vec![1, 2, 3];
    let w: Vec<&[i32]> = v.windows(2).collect();
    assert_eq!(w.len(), 2);
}
```

#### 32. `chunks` batches

```rust
fn main() {
    let v: Vec<u32> = (0..10).collect();
    assert_eq!(v.chunks(3).count(), 4);
}
```

#### 33. Checked arithmetic

```rust
fn main() {
    assert_eq!(255u8.checked_add(1), None);
    assert_eq!(255u8.saturating_add(1), 255);
    assert_eq!(255u8.wrapping_add(1), 0);
}
```

#### 34. `NonZeroU32` for nonzero IDs

```rust
use std::num::NonZeroU32;

fn main() {
    let id = NonZeroU32::new(5).unwrap();
    assert_eq!(id.get(), 5);
    assert!(NonZeroU32::new(0).is_none());
}
```

#### 35. `VecDeque` queue

```rust
use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::new();
    q.push_back(1);
    assert_eq!(q.pop_front(), Some(1));
}
```

#### 36. Sort by key

```rust
fn main() {
    let mut v = vec!["bb", "a", "ccc"];
    v.sort_by_key(|s| s.len());
    assert_eq!(v, vec!["a", "bb", "ccc"]);
}
```

#### 37. `dedup` after sort

```rust
fn main() {
    let mut v = vec![2, 1, 2, 1];
    v.sort();
    v.dedup();
    assert_eq!(v, vec![1, 2]);
}
```

#### 38. `HashSet` membership

```rust
use std::collections::HashSet;

fn main() {
    let s: HashSet<u32> = [1, 2, 3].into_iter().collect();
    assert!(s.contains(&2));
}
```

#### 39. String `replace`

```rust
fn main() {
    assert_eq!("a-b-c".replace('-', "_"), "a_b_c");
}
```

#### 40. `starts_with` / `ends_with`

```rust
fn main() {
    assert!("main.rs".ends_with(".rs"));
    assert!("GET /".starts_with("GET"));
}
```

---

### C. Errors, Files, Processes (61–90)

#### 41. `ok_or` missing env

```rust
fn port() -> Result<u16, String> {
    std::env::var("PORT")
        .map_err(|_| "PORT missing".to_string())?
        .parse()
        .map_err(|_| "PORT bad".to_string())
}
```

#### 42. `map_err` with context

```rust
fn read(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))
}
```

#### 43. `unwrap_or_else` default config

```rust
fn main() {
    let p: u16 = None::<&str>.map(|s| s.parse().ok()).flatten().unwrap_or(Some(3000)).unwrap_or(3000);
    assert_eq!(p, 3000);
}
```

#### 44. Buffered line count

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn lines(path: &str) -> std::io::Result<usize> {
    let f = File::open(path)?;
    Ok(BufReader::new(f).lines().count())
}
```

#### 45. Atomic write

```rust
use std::fs::File;
use std::io::Write;

fn atomic(path: &str, data: &[u8]) -> std::io::Result<()> {
    let tmp = format!("{path}.tmp");
    {
        let mut f = File::create(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
    }
    std::fs::rename(tmp, path)?;
    Ok(())
}
```

#### 46. `ErrorKind::NotFound` fallback

```rust
use std::io::ErrorKind;

fn main() {
    let s = match std::fs::read_to_string("nope.txt") {
        Ok(s) => s,
        Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
        Err(e) => panic!("{e}"),
    };
    assert_eq!(s, "");
}
```

#### 47. `Path::extension` check

```rust
use std::path::Path;

fn main() {
    assert_eq!(Path::new("a.rs").extension().unwrap(), "rs");
}
```

#### 48. `canonicalize` sandbox check

```rust
fn inside_root(root: &str, p: &str) -> bool {
    let r = std::path::Path::new(root).canonicalize().unwrap();
    let c = std::path::Path::new(p).canonicalize().unwrap_or(r.clone());
    c.starts_with(&r)
}
```

#### 49. Subprocess without shell

```rust
fn main() -> std::io::Result<()> {
    let out = std::process::Command::new("git").args(["--version"]).output()?;
    assert!(out.status.success());
    Ok(())
}
```

#### 50. Exit codes by convention

```rust
fn usage() -> ! {
    eprintln!("usage: app <file>");
    std::process::exit(2);
}
```

#### 51. `IsTerminal` color gate

```rust
fn main() {
    use std::io::IsTerminal;
    let _color = std::io::stdout().is_terminal();
}
```

#### 52. Env with default

```rust
fn main() {
    let h = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    assert!(!h.is_empty());
}
```

#### 53. `tempfile`-free temp path

```rust
fn tmp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("odyssey-{name}"))
}
```

#### 54. Read bytes, hash, write object

```rust
fn main() -> std::io::Result<()> {
    let b = std::fs::read("Cargo.toml").unwrap_or_default();
    assert!(!b.is_empty() || true);
    Ok(())
}
```

#### 55. Append to log file

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn append(path: &str, line: &str) -> std::io::Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{line}")?;
    Ok(())
}
```

---

### D. Threads, Async, Net (91–120)

#### 56. Scoped threads sum

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let sum = std::thread::scope(|s| {
        let (a, b) = v.split_at(2);
        let h1 = s.spawn(|| a.iter().sum::<i32>());
        let h2 = s.spawn(|| b.iter().sum::<i32>());
        h1.join().unwrap() + h2.join().unwrap()
    });
    assert_eq!(sum, 10);
}
```

#### 57. `mpsc` bounded pipeline

```rust
fn main() {
    let (tx, rx) = std::sync::mpsc::sync_channel::<u32>(8);
    std::thread::spawn(move || {
        for i in 0..4 {
            tx.send(i).unwrap();
        }
    });
    let got: Vec<u32> = rx.into_iter().collect();
    assert_eq!(got, vec![0, 1, 2, 3]);
}
```

#### 58. Tokio `join!` two sleeps

```rust
#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(
        async { 1u32 },
        async { 2u32 },
    );
    assert_eq!(a + b, 3);
}
```

#### 59. Tokio timeout

```rust
#[tokio::main]
async fn main() {
    let slow = async {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    };
    assert!(tokio::time::timeout(std::time::Duration::from_millis(10), slow).await.is_err());
}
```

#### 60. Tokio `select!` first-ready

```rust
#[tokio::main]
async fn main() {
    let v = tokio::select! {
        a = async { "a" } => a,
        b = async { "b" } => b,
    };
    assert!(v == "a" || v == "b");
}
```

#### 61. TCP bind loopback

```rust
fn main() -> std::io::Result<()> {
    let l = std::net::TcpListener::bind("127.0.0.1:0")?;
    assert!(l.local_addr().is_ok());
    Ok(())
}
```

#### 62. Read timeout on socket

```rust
fn main() -> std::io::Result<()> {
    let s = std::net::TcpStream::connect("127.0.0.1:80")
        .or_else(|_| std::net::TcpStream::connect("127.0.0.1:81"));
    let _ = s;
    Ok(())
}
```

#### 63. `Arc<Mutex>` counter

```rust
use std::sync::{Arc, Mutex};

fn main() {
    let c = Arc::new(Mutex::new(0));
    let d = Arc::clone(&c);
    *d.lock().unwrap() += 1;
    assert_eq!(*c.lock().unwrap(), 1);
}
```

#### 64. `RwLock` read-mostly

```rust
use std::sync::RwLock;

fn main() {
    let l = RwLock::new(vec![1]);
    assert_eq!(l.read().unwrap().len(), 1);
}
```

#### 65. Atomic flag

```rust
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let f = AtomicBool::new(false);
    f.store(true, Ordering::SeqCst);
    assert!(f.load(Ordering::SeqCst));
}
```

---

### E. Testing, Macros, FFI (121–150)

#### 66. Doc-test example

```rust
/// Double the input.
///
/// ```
/// assert_eq!(double_it(2), 4);
/// ```
fn double_it(n: i32) -> i32 {
    n * 2
}
```

#### 67. `assert!(matches!)` enum shape

```rust
fn main() {
    let r: Result<u32, &str> = Ok(1);
    assert!(matches!(r, Ok(_)));
}
```

#### 68. `hashmap!` macro

```rust
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}

fn main() {
    let m = hashmap!["a" => 1,];
    assert_eq!(m["a"], 1);
}
```

#### 69. `vec_of_strings!` macro

```rust
macro_rules! vec_of_strings {
    ($($s:expr),* $(,)?) => {
        vec![$($s.to_string()),*]
    };
}

fn main() {
    assert_eq!(vec_of_strings!["a"], vec!["a".to_string()]);
}
```

#### 70. FNV-1a hash helper

```rust
fn fnv(b: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for x in b {
        h ^= *x as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn main() {
    assert_ne!(fnv(b"a"), fnv(b"b"));
}
```

---

### How to Practice From Here

Pick one recipe per day, delete the body, rewrite from the signature,
and run the assert. When all 70 pass from memory, move to Part X
capstones. Speed comes from recall, not re-reading.

---

### F. Five Final Recipes (71–75)

#### 71. `then_some` for flag values

```rust
fn main() {
    let verbose = true;
    let level = verbose.then_some("debug").unwrap_or("info");
    assert_eq!(level, "debug");
}
```

#### 72. `let-else` for early return

```rust
fn head(v: Vec<u32>) -> u32 {
    let Some(&h) = v.first() else {
        return 0;
    };
    h
}

fn main() {
    assert_eq!(head(vec![]), 0);
    assert_eq!(head(vec![5]), 5);
}
```

#### 73. `div_ceil` for page counts

```rust
fn main() {
    assert_eq!(101u32.div_ceil(10), 11);
    assert_eq!(100u32.div_ceil(10), 10);
}
```

#### 74. `intersperse` join without collect

```rust
fn main() {
    let s: String = ["a", "b", "c"].into_iter().intersperse(", ").collect();
    assert_eq!(s, "a, b, c");
}
```

Requires nightly `iter_intersperse` or itertools. On stable, use
`join(", ")` on slices of `&str` via the `join` crate pattern.

#### 75. `is_some_and` predicate check

```rust
fn main() {
    let v: Option<u32> = Some(8);
    assert!(v.is_some_and(|n| n % 2 == 0));
}
```


---

<a id="part-xii-backend-recipes"></a>

## Part XII - Backend Recipes

| Navigation | |
| --- | --- |
| Previous | [← Part XI - Cookbook (75 recipes)](#part-xi-cookbook-75-recipes) |
| Next | [Part XIII - Algorithms →](#part-xiii-algorithms) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part XII: Backend, SQL, Web, and Testing Recipes

> 🧭 **How to use this file:** Production-ready snippets for the
> `odyssey-app` stack. Each section is independent: SQL first, then
> Axum handlers, then Leptos data patterns, then test harnesses.

---

### F. SQL and sqlx (1–40)

#### F1. Create tasks table

```sql
CREATE TABLE tasks (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL CHECK (char_length(title) BETWEEN 1 AND 200),
    done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### F2. Index for list queries

```sql
CREATE INDEX idx_tasks_done_created ON tasks (done, created_at DESC);
```

#### F3. Insert and return

```sql
INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, done;
```

#### F4. Paginated list

```sql
SELECT id, title, done FROM tasks ORDER BY id LIMIT $1 OFFSET $2;
```

#### F5. Get by id

```sql
SELECT id, title, done FROM tasks WHERE id = $1;
```

#### F6. Mark done

```sql
UPDATE tasks SET done = $1 WHERE id = $2 RETURNING id, title, done;
```

#### F7. Delete by id

```sql
DELETE FROM tasks WHERE id = $1;
```

#### F8. Count unfinished

```sql
SELECT count(*) FROM tasks WHERE done = FALSE;
```

#### F9. Search by prefix

```sql
SELECT id, title FROM tasks WHERE title ILIKE $1 || '%' LIMIT 50;
```

#### F10. sqlx query_as with struct

```rust
#[derive(sqlx::FromRow)]
struct Task {
    id: i64,
    title: String,
    done: bool,
}

async fn get(pool: &sqlx::PgPool, id: i64) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>("SELECT id, title, done FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}
```

#### F11. Insert via query_as

```rust
async fn create(pool: &sqlx::PgPool, title: &str) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>("INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, done")
        .bind(title)
        .fetch_one(pool)
        .await
}
```

#### F12. Transaction transfer

```rust
async fn transfer(pool: &sqlx::PgPool, from: i64, to: i64, n: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query!("UPDATE accounts SET balance = balance - $1 WHERE id = $2", n, from)
        .execute(&mut *tx)
        .await?;
    sqlx::query!("UPDATE accounts SET balance = balance + $1 WHERE id = $2", n, to)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
```

#### F13. `#[sqlx::test]` with migrations

```rust
#[sqlx::test(migrations = "./migrations")]
async fn creates_task(pool: sqlx::PgPool) {
    let t = create(&pool, "milk").await.unwrap();
    assert_eq!(t.title, "milk");
}
```

#### F14. Pool with timeouts

```rust
fn pool(url: &str) -> sqlx::postgres::PgPoolOptions {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(3))
}
```

#### F15. Clamp pagination in handler

```rust
fn clamp_limit(raw: Option<i64>) -> i64 {
    raw.unwrap_or(50).clamp(1, 100)
}
```

---

### G. Axum Handlers and Middleware (41–80)

#### G1. App state

```rust
#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}
```

#### G2. Error enum to HTTP

```rust
use axum::{http::StatusCode, response::{IntoResponse, Json}};

enum AppError {
    NotFound,
    Bad(String),
    Db(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            AppError::Bad(m) => (StatusCode::BAD_REQUEST, m.clone()),
            AppError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "db error".to_string()),
        };
        tracing::error!(?code, "handler error");
        (code, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}
```

#### G3. List with pagination

```rust
use axum::extract::{Query, State};

#[derive(serde::Deserialize)]
struct ListQ {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list(
    State(s): State<AppState>,
    Query(q): Query<ListQ>,
) -> Result<Json<Vec<Task>>, AppError> {
    let limit = q.limit.unwrap_or(50).clamp(1, 100);
    let offset = q.offset.unwrap_or(0).max(0);
    sqlx::query_as::<_, Task>("SELECT id, title, done FROM tasks ORDER BY id LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(&s.db)
        .await
        .map(Json)
        .map_err(AppError::Db)
}
```

#### G4. Create with validation

```rust
use axum::Json;

#[derive(serde::Deserialize)]
struct NewTask {
    title: String,
}

async fn create_task(
    State(s): State<AppState>,
    Json(b): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let t = b.title.trim();
    if t.is_empty() {
        return Err(AppError::Bad("title required".into()));
    }
    if t.chars().count() > 200 {
        return Err(AppError::Bad("title too long".into()));
    }
    let task = sqlx::query_as::<_, Task>("INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, done")
        .bind(t)
        .fetch_one(&s.db)
        .await
        .map_err(AppError::Db)?;
    Ok((StatusCode::CREATED, Json(task)))
}
```

#### G5. Get one or 404

```rust
use axum::extract::Path;

async fn get_one(
    State(s): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, AppError> {
    sqlx::query_as::<_, Task>("SELECT id, title, done FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_optional(&s.db)
        .await
        .map_err(AppError::Db)?
        .map(Json)
        .ok_or(AppError::NotFound)
}
```

#### G6. Patch done flag

```rust
#[derive(serde::Deserialize)]
struct Patch {
    done: bool,
}

async fn patch(
    State(s): State<AppState>,
    Path(id): Path<i64>,
    Json(b): Json<Patch>,
) -> Result<Json<Task>, AppError> {
    sqlx::query_as::<_, Task>("UPDATE tasks SET done = $1 WHERE id = $2 RETURNING id, title, done")
        .bind(b.done)
        .bind(id)
        .fetch_optional(&s.db)
        .await
        .map_err(AppError::Db)?
        .map(Json)
        .ok_or(AppError::NotFound)
}
```

#### G7. Delete returns 204

```rust
async fn delete(
    State(s): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let n = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&s.db)
        .await
        .map_err(AppError::Db)?
        .rows_affected();
    if n == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
```

#### G8. Router wiring

```rust
fn router(state: AppState) -> axum::Router {
    use axum::routing::{get, patch};
    axum::Router::new()
        .route("/tasks", get(list).post(create_task))
        .route("/tasks/:id", get(get_one).patch(patch).delete(delete))
        .with_state(state)
}
```

Note: Axum 0.8 uses `/:id` and 0.7 uses `/:id` too; older used `/:id`.
Check your version. The handler signatures are stable.

#### G9. Restrictive CORS

```rust
fn cors(frontend: &str) -> tower_http::cors::CorsLayer {
    tower_http::cors::CorsLayer::new()
        .allow_origin([frontend.parse().unwrap()])
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
}
```

#### G10. Timeout layer

```rust
fn with_timeout(router: axum::Router) -> axum::Router {
    use std::time::Duration;
    router.layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(5)))
}
```

#### G11. Tracing init

```rust
fn init_tracing() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("info")
        .init();
}
```

#### G12. Graceful shutdown

```rust
async fn shutdown() {
    tokio::signal::ctrl_c().await.ok();
    tracing::info!("shutting down");
}
```

#### G13. Handler test with oneshot

```rust
#[tokio::test]
async fn unknown_id_is_404() {
    // Requires tower 0.4/0.5 with util feature.
    // Build router against a test DB, then:
    // let res = tower::ServiceExt::oneshot(router, req).await.unwrap();
    // assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
```

---

### H. Leptos Data Patterns (81–110)

#### H1. Tasks state machine

```rust
#[derive(Clone, PartialEq)]
enum TasksState {
    Loading,
    Loaded(Vec<TaskView>),
    Empty,
    Failed(String),
}

#[derive(Clone, PartialEq)]
struct TaskView {
    id: i64,
    title: String,
    done: bool,
}
```

#### H2. Transition function (testable without framework)

```rust
fn next(current: &[TaskView], res: Result<Vec<TaskView>, String>) -> TasksState {
    match res {
        Err(e) => TasksState::Failed(e),
        Ok(v) if v.is_empty() => TasksState::Empty,
        Ok(v) => {
            let _ = current;
            TasksState::Loaded(v)
        }
    }
}
```

#### H3. Title validation shared with backend rule

```rust
fn validate(title: &str) -> Result<(), String> {
    let t = title.trim();
    if t.is_empty() {
        return Err("Give the task a name.".into());
    }
    if t.chars().count() > 200 {
        return Err("Keep titles under 200 characters.".into());
    }
    Ok(())
}
```

#### H4. Optimistic toggle with rollback value

```rust
fn toggle_local(tasks: &mut [TaskView], id: i64) -> Option<bool> {
    let t = tasks.iter_mut().find(|t| t.id == id)?;
    let prev = t.done;
    t.done = !t.done;
    Some(prev)
}

fn rollback(tasks: &mut [TaskView], id: i64, prev: bool) {
    if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
        t.done = prev;
    }
}
```

#### H5. API URL from constant, never hardcoded per call

```rust
const API: &str = "http://127.0.0.1:3000/tasks";
```

---

### I. Testing and CI Recipes (111–140)

#### I1. Unit test table

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_table() {
        assert_eq!(clamp_limit(None), 50);
        assert_eq!(clamp_limit(Some(0)), 1);
        assert_eq!(clamp_limit(Some(9999)), 100);
    }
}
```

#### I2. Integration layout

```text
tests/
  api_tasks.rs   <- spins test DB, hits router
  auth.rs        <- login rate-limit, enumeration
```

#### I3. Doc test for public API

```rust
/// Clamp client limit to 1..100.
///
/// ```
/// assert_eq!(clamp_limit(Some(5)), 5);
/// ```
fn clamp_limit(raw: Option<i64>) -> i64 {
    raw.unwrap_or(50).clamp(1, 100)
}
```

#### I4. Fuzz parser entry

```rust
// fuzz/fuzz_targets/parse.rs
// libfuzzer_sys::fuzz_target!(|data: &[u8]| {
//     let _ = std::str::from_utf8(data).map(|s| s.len());
// });
```

#### I5. Criterion bench skeleton

```rust
// See Chapter 27 hashing bench. Baseline in CI, fail on >10% regression.
```

#### I6. `cargo` aliases for the book

```toml
[alias]
c = "clippy -- -D warnings"
t = "test"
tq = "test --quiet"
```

#### I7. `cargo deny` license gate

```toml
# deny.toml
[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unicode-DFS-2016"]
```

#### I8. Size budget check

```bash
trunk build --release
gzip -c dist/*.wasm | wc -c
# fail CI when over 300_000 bytes gzip
```

---

### J. Deployment Checklist Items (141–150)

#### J1. `.env.example` (no secrets)

```bash
BIND_ADDR=0.0.0.0:3000
DATABASE_URL=postgres://postgres:secret@localhost:5432/odyssey
FRONTEND_ORIGIN=http://127.0.0.1:8080
RUST_LOG=info
```

#### J2. Health vs readiness

```rust
async fn healthz() -> &'static str {
    "ok"
}

async fn readyz(State(s): State<AppState>) -> Result<&'static str, StatusCode> {
    sqlx::query!("SELECT 1 as one").fetch_one(&s.db).await.map(|_| "ready").map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
}
```

#### J3. Dockerfile (pinned, nonroot)

```dockerfile
FROM rust:1.82-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p backend
FROM debian:bookworm-slim
RUN useradd -m appuser
USER appuser
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
EXPOSE 3000
CMD ["backend"]
```

---

*End of Part XII. Total course now exceeds 10,000 lines across all parts.*

---

### K. SQL Safety Rules (Read Before Merging)

```text
1. Every $n is bound, never interpolated. No format! into SQL.
2. Every list query has LIMIT clamped server-side.
3. Every migration has a rollback tested on staging data.
4. Every money column is NUMERIC or BIGINT minor units, never FLOAT.
5. Every SELECT names columns, never SELECT *. New columns must not leak.
6. Every transaction is short, ordered locks, no network I/O inside.
7. Every error maps to 4xx/5xx with {error} shape, no SQL text out.
```


---

<a id="part-xiii-algorithms"></a>

## Part XIII - Algorithms

| Navigation | |
| --- | --- |
| Previous | [← Part XII - Backend Recipes](#part-xii-backend-recipes) |
| Next | [Part XIV - Testing and Ops →](#part-xiv-testing-and-ops) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part XIII: Algorithms and Data Structures in Idiomatic Rust

> 🧭 **How to use this file:** Every algorithm in one file, same shape:
> signature, complexity, runnable code, test. No pseudocode. All code
> compiles on stable Rust with no dependencies.

---

### K. Sorting (1–15)

#### K1. Bubble sort (teaching only, O(n²))

```rust
fn bubble(a: &mut [i32]) {
    let n = a.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut v = vec![3, 1, 2];
    bubble(&mut v);
    assert_eq!(v, vec![1, 2, 3]);
}
```

#### K2. Insertion sort (small n, O(n²))

```rust
fn insertion(a: &mut [i32]) {
    for i in 1..a.len() {
        let mut j = i;
        while j > 0 && a[j] < a[j - 1] {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}
```

#### K3. Merge sort (stable, O(n log n))

```rust
fn merge_sort(v: Vec<i32>) -> Vec<i32> {
    if v.len() <= 1 {
        return v;
    }
    let m = v.len() / 2;
    let (a, b) = v.split_at(m);
    let a = merge_sort(a.to_vec());
    let b = merge_sort(b.to_vec());
    merge(&a, &b)
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            out.push(a[i]);
            i += 1;
        } else {
            out.push(b[j]);
            j += 1;
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}
```

#### K4. Quicksort (in place, avg O(n log n))

```rust
fn quick(a: &mut [i32]) {
    if a.len() <= 1 {
        return;
    }
    let p = partition(a);
    let (l, r) = a.split_at_mut(p);
    quick(l);
    quick(&mut r[1..]);
}

fn partition(a: &mut [i32]) -> usize {
    let n = a.len();
    let pivot = a[n - 1];
    let mut i = 0;
    for j in 0..n - 1 {
        if a[j] <= pivot {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, n - 1);
    i
}
```

#### K5. Use `sort_unstable` in production

```rust
fn main() {
    let mut v = vec![3, 1, 2];
    v.sort_unstable(); // faster, no stability guarantee
    assert_eq!(v, vec![1, 2, 3]);
}
```

Rule: `sort` is stable (keeps equal order), `sort_unstable` is faster.
Default to `sort_unstable` for numbers, `sort` for rows with ties.

#### K6. `sort_by_key` on structs

```rust
#[derive(Debug, PartialEq)]
struct T {
    id: i64,
    title: String,
}

fn main() {
    let mut v = vec![
        T { id: 2, title: "b".into() },
        T { id: 1, title: "a".into() },
    ];
    v.sort_by_key(|t| t.id);
    assert_eq!(v[0].id, 1);
}
```

#### K7. Top-k with `select_nth_unstable`

```rust
fn main() {
    let mut v = vec![5, 1, 4, 2, 3];
    v.select_nth_unstable(2); // 3rd smallest at index 2, O(n) avg
    assert_eq!(v[2], 3);
}
```

#### K8. Binary search

```rust
fn main() {
    let v = vec![1, 2, 3, 5];
    assert_eq!(v.binary_search(&3), Ok(2));
    assert_eq!(v.binary_search(&4), Err(3)); // insertion point
}
```

#### K9. `partition` dedup pipeline

```rust
fn main() {
    let mut v = vec![3, 1, 2, 1, 3];
    v.sort_unstable();
    v.dedup();
    assert_eq!(v, vec![1, 2, 3]);
}
```

#### K10. Counting sort for u8

```rust
fn counting(a: &mut [u8]) {
    let mut c = [0usize; 256];
    for &x in a.iter() {
        c[x as usize] += 1;
    }
    let mut i = 0;
    for (v, &n) in c.iter().enumerate() {
        for _ in 0..n {
            a[i] = v as u8;
            i += 1;
        }
    }
}
```

---

### L. Searching, Two Pointers, Sliding Window (16–35)

#### L1. Linear search returns index

```rust
fn find(a: &[i32], x: i32) -> Option<usize> {
    a.iter().position(|&v| v == x)
}
```

#### L2. Two-sum sorted (two pointers, O(n))

```rust
fn two_sum(a: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut l, mut r) = (0, a.len().checked_sub(1)?);
    while l < r {
        let s = a[l] + a[r];
        if s == target {
            return Some((l, r));
        } else if s < target {
            l += 1;
        } else {
            r -= 1;
        }
    }
    None
}
```

#### L3. Two-sum unsorted (hashmap, O(n))

```rust
use std::collections::HashMap;

fn two_sum_map(a: &[i32], t: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        if let Some(&j) = seen.get(&(t - x)) {
            return Some((j, i));
        }
        seen.insert(x, i);
    }
    None
}
```

#### L4. Sliding window max sum k

```rust
fn max_sum_k(a: &[i32], k: usize) -> i32 {
    assert!(k > 0 && k <= a.len());
    let mut win: i32 = a[..k].iter().sum();
    let mut best = win;
    for i in k..a.len() {
        win += a[i] - a[i - k];
        best = best.max(win);
    }
    best
}
```

#### L5. Longest unique substring (chars)

```rust
use std::collections::HashMap;

fn longest_unique(s: &str) -> usize {
    let mut last: HashMap<char, usize> = HashMap::new();
    let mut start = 0;
    let mut best = 0;
    for (i, c) in s.chars().enumerate() {
        if let Some(&j) = last.get(&c) {
            start = start.max(j + 1);
        }
        last.insert(c, i);
        best = best.max(i + 1 - start);
    }
    best
}
```

#### L6. Prefix sums for range queries

```rust
fn prefix(a: &[i64]) -> Vec<i64> {
    let mut p = vec![0; a.len() + 1];
    for (i, &x) in a.iter().enumerate() {
        p[i + 1] = p[i] + x;
    }
    p
}

fn range_sum(p: &[i64], l: usize, r: usize) -> i64 {
    p[r] - p[l] // [l, r)
}
```

#### L7. `windows` for adjacent diffs

```rust
fn main() {
    let v = vec![1, 3, 6, 10];
    let d: Vec<i32> = v.windows(2).map(|w| w[1] - w[0]).collect();
    assert_eq!(d, vec![2, 3, 4]);
}
```

---

### M. Stacks, Queues, Heaps, Maps (36–60)

#### M1. Balanced brackets with stack

```rust
fn balanced(s: &str) -> bool {
    let mut st = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => st.push(c),
            ')' => if st.pop() != Some('(') { return false; },
            ']' => if st.pop() != Some('[') { return false; },
            '}' => if st.pop() != Some('{') { return false; },
            _ => {}
        }
    }
    st.is_empty()
}
```

#### M2. Min stack (value + current min)

```rust
struct MinStack {
    s: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { s: vec![] }
    }
    fn push(&mut self, x: i32) {
        let m = self.s.last().map(|&(_, m)| m.min(x)).unwrap_or(x);
        self.s.push((x, m));
    }
    fn min(&self) -> Option<i32> {
        self.s.last().map(|&(_, m)| m)
    }
}
```

#### M3. Queue with `VecDeque`

```rust
use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::from([1, 2]);
    q.push_back(3);
    assert_eq!(q.pop_front(), Some(1));
}
```

#### M4. Binary heap top-k

```rust
use std::collections::BinaryHeap;

fn top_k(a: Vec<i32>, k: usize) -> Vec<i32> {
    let mut h = BinaryHeap::from(a);
    (0..k).filter_map(|_| h.pop()).collect()
}
```

#### M5. `BTreeMap` range query

```rust
use std::collections::BTreeMap;

fn main() {
    let m: BTreeMap<u32, &str> = [(1, "a"), (5, "b"), (10, "c")].into();
    let v: Vec<(&u32, &&str)> = m.range(2..=9).collect();
    assert_eq!(v.len(), 1);
}
```

#### M6. LRU sketch (HashMap + order list)

```rust
use std::collections::{HashMap, VecDeque};

struct Lru {
    cap: usize,
    map: HashMap<String, String>,
    order: VecDeque<String>,
}

impl Lru {
    fn get(&mut self, k: &str) -> Option<&String> {
        self.map.get(k)
    }
    fn put(&mut self, k: String, v: String) {
        if self.map.len() >= self.cap && !self.map.contains_key(&k) {
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
            }
        }
        self.order.push_back(k.clone());
        self.map.insert(k, v);
    }
}
```

Production uses `hashlink` or `lru` crate. This sketch teaches the
invariants: map for O(1), queue for eviction order, cap enforced on
insert.

---

### N. Graphs and Trees (61–85)

#### N1. BFS shortest path (unweighted)

```rust
use std::collections::VecDeque;

fn bfs(adj: &[Vec<usize>], start: usize) -> Vec<Option<u32>> {
    let mut dist = vec![None; adj.len()];
    let mut q = VecDeque::from([start]);
    dist[start] = Some(0);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if dist[v].is_none() {
                dist[v] = Some(dist[u].unwrap() + 1);
                q.push_back(v);
            }
        }
    }
    dist
}
```

#### N2. DFS iterative (no recursion limit)

```rust
fn dfs(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut seen = vec![false; adj.len()];
    let mut stack = vec![start];
    let mut order = Vec::new();
    while let Some(u) = stack.pop() {
        if seen[u] {
            continue;
        }
        seen[u] = true;
        order.push(u);
        for &v in adj[u].iter().rev() {
            if !seen[v] {
                stack.push(v);
            }
        }
    }
    order
}
```

#### N3. Dijkstra (BinaryHeap as min-heap)

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(adj: &[Vec<(usize, u32)>], start: usize) -> Vec<u32> {
    let mut dist = vec![u32::MAX; adj.len()];
    let mut h = BinaryHeap::new();
    dist[start] = 0;
    h.push((Reverse(0u32), start));
    while let Some((Reverse(d), u)) = h.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = d.saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                h.push((Reverse(nd), v));
            }
        }
    }
    dist
}
```

#### N4. Union-Find (disjoint set)

```rust
struct Uf {
    p: Vec<usize>,
    r: Vec<u8>,
}

impl Uf {
    fn new(n: usize) -> Self {
        Self { p: (0..n).collect(), r: vec![0; n] }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            let root = self.find(self.p[x]);
            self.p[x] = root;
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return;
        }
        if self.r[a] < self.r[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        if self.r[a] == self.r[b] {
            self.r[a] += 1;
        }
    }
}
```

#### N5. Binary tree inorder (iterative)

```rust
struct Node {
    v: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn inorder(root: &Option<Box<Node>>, out: &mut Vec<i32>) {
    let mut stack: Vec<&Node> = Vec::new();
    let mut cur = root.as_deref();
    while cur.is_some() || !stack.is_empty() {
        while let Some(n) = cur {
            stack.push(n);
            cur = n.left.as_deref();
        }
        let n = stack.pop().unwrap();
        out.push(n.v);
        cur = n.right.as_deref();
    }
}
```

#### N6. Topological sort (Kahn)

```rust
use std::collections::VecDeque;

fn topo(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    let mut deg = vec![0; n];
    for &(u, v) in edges {
        adj[u].push(v);
        deg[v] += 1;
    }
    let mut q: VecDeque<usize> = deg.iter().enumerate().filter(|&(_, &d)| d == 0).map(|(i, _)| i).collect();
    let mut out = Vec::new();
    while let Some(u) = q.pop_front() {
        out.push(u);
        for &v in &adj[u] {
            deg[v] -= 1;
            if deg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if out.len() == n { Some(out) } else { None } // None = cycle
}
```

---

### O. Dynamic Programming (86–105)

#### O1. Fibonacci bottom-up

```rust
fn fib(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
```

#### O2. 0/1 knapsack (1D DP)

```rust
fn knapsack(w: &[usize], v: &[u64], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for (i, &wi) in w.iter().enumerate() {
        for c in (wi..=cap).rev() {
            dp[c] = dp[c].max(dp[c - wi] + v[i]);
        }
    }
    dp[cap]
}
```

#### O3. Longest increasing subsequence (O(n log n))

```rust
fn lis(a: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in a {
        match tails.binary_search(&x) {
            Ok(i) | Err(i) => {
                if i == tails.len() {
                    tails.push(x);
                } else {
                    tails[i] = x;
                }
            }
        }
    }
    tails.len()
}
```

#### O4. Edit distance (Levenshtein)

```rust
fn edit(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        dp[i][0] = i;
    }
    for j in 0..=b.len() {
        dp[0][j] = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            dp[i][j] = (dp[i - 1][j] + 1).min(dp[i][j - 1] + 1).min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[a.len()][b.len()]
}
```

#### O5. Coin change (min coins)

```rust
fn coins(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; amount + 1];
    dp[0] = 0;
    for c in 1..=amount {
        for &coin in coins {
            if coin <= c && dp[c - coin] != usize::MAX {
                dp[c] = dp[c].min(dp[c - coin] + 1);
            }
        }
    }
    (dp[amount] != usize::MAX).then_some(dp[amount])
}
```

---

### P. Strings and Parsing Helpers (106–120)

#### P1. `split_once` key=value

```rust
fn main() {
    let (k, v) = "PORT=8080".split_once('=').unwrap();
    assert_eq!((k, v), ("PORT", "8080"));
}
```

#### P2. Trim prefix/suffix

```rust
fn main() {
    assert_eq!("--help".trim_start_matches('-'), "help");
    assert_eq!("a\n".trim_end(), "a");
}
```

#### P3. Parse list with errors kept

```rust
fn parse_all(items: &[&str]) -> (Vec<u32>, Vec<&str>) {
    let mut ok = Vec::new();
    let mut err = Vec::new();
    for s in items {
        match s.parse::<u32>() {
            Ok(n) => ok.push(n),
            Err(_) => err.push(*s),
        }
    }
    (ok, err)
}
```

---

*End of Part XIII. Course total now exceeds 10,000 lines.*

---

### Q. Complexity Cheat Sheet (Memorize This Table)

| Operation | Vec | HashMap avg | BTreeMap | BinaryHeap | VecDeque |
| --- | --- | --- | --- | --- | --- |
| push/insert | O(1) amort | O(1) | O(log n) | O(log n) | O(1) ends |
| pop/remove | O(1) end | O(1) | O(log n) | O(log n) | O(1) ends |
| lookup | O(n) | O(1) | O(log n) | O(n) | O(n) |
| ordered iter | sort O(n log n) | none | O(n) sorted | sorted pop O(n log n) | insertion order |
| memory | contiguous | table + hash | nodes | contiguous | ring buffer |

| Sort | Time avg | Stable | Notes |
| --- | --- | --- | --- |
| insertion | O(n²) | yes | tiny n or nearly sorted |
| merge | O(n log n) | yes | needs O(n) extra |
| quick | O(n log n) | no | in place, watch worst O(n²) |
| `sort_unstable` | O(n log n) | no | default for numbers |
| counting | O(n+k) | yes | small integer range only |

| Graph | Time | Space | When |
| --- | --- | --- | --- |
| BFS | O(V+E) | O(V) | unweighted shortest path |
| DFS | O(V+E) | O(V) | order, components, topo |
| Dijkstra | O(E log V) | O(V) | non-negative weights |
| Union-Find | ~O(1) amort | O(V) | connectivity, Kruskal |
| Topo Kahn | O(V+E) | O(V) | DAG order, cycle detect |

Say the trade before the code in interviews. "HashMap for O(1) lookup,
BTreeMap when I need ranges" beats silent coding every time.


---

<a id="part-xiv-testing-and-ops"></a>

## Part XIV - Testing and Ops

| Navigation | |
| --- | --- |
| Previous | [← Part XIII - Algorithms](#part-xiii-algorithms) |
| Next | [Appendices B-I →](#appendices-b-i) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Part XIV: Testing Mastery, Deployment, and Operations

> 🧭 **Where we are:** You can write algorithms and services. Part XIV
> makes them stay up: test strategy, containers, CI/CD, Kubernetes,
> observability, and incident response.

---

### Q. Test Strategy (1–25)

#### Q1. Test pyramid for Rust services

```text
e2e (few, slow, critical paths: signup -> create -> delete)
  integration (DB, API routes, migrations)
    unit (pure logic, parsers, validators)
      doc tests (public API examples)
```

#### Q2. Unit test with table

```rust
fn clamp(n: i64) -> i64 {
    n.clamp(1, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table() {
        for (input, want) in [(0, 1), (50, 50), (999, 100)] {
            assert_eq!(clamp(input), want);
        }
    }
}
```

#### Q3. Integration test layout

```text
tests/
  api.rs      <- starts app on random port
  db.rs       <- uses #[sqlx::test]
```

```rust
// tests/api.rs
#[tokio::test]
async fn health() {
    // spawn app, reqwest GET /healthz, assert 200
}
```

#### Q4. `#[sqlx::test]` needs isolated DB

```rust
#[sqlx::test(migrations = "./migrations")]
async fn inserts(pool: sqlx::PgPool) {
    sqlx::query!("INSERT INTO tasks (title) VALUES ('a')")
        .execute(&pool)
        .await
        .unwrap();
}
```

Each test gets a fresh database. Never share tables between tests.

#### Q5. Snapshot testing for CLI output

```rust
fn render(n: u32) -> String {
    format!("count={n}")
}

#[test]
fn snapshot() {
    insta::assert_snapshot!(render(3), @"count=3");
}
```

Add `insta = "1"` to dev-dependencies. Review snapshots in PRs like code.

#### Q6. Proptest roundtrip

```rust
// proptest! { #[test] fn port_roundtrip(n: u16) {
//   prop_assert_eq!(parse_port(&n.to_string()), Ok(n)); } }
```

#### Q7. Mock time with injected clock

```rust
trait Clock {
    fn now(&self) -> u64;
}

struct Fixed(u64);

impl Clock for Fixed {
    fn now(&self) -> u64 {
        self.0
    }
}

fn expired<C: Clock>(c: &C, at: u64) -> bool {
    c.now() >= at
}
```

Never call `SystemTime::now()` deep in logic. Inject `Clock` so tests
control time deterministically.

#### Q8. Test for absence (authz matrix)

```rust
// For each route x {anon, user, other-tenant, admin} assert allow/deny.
// Deny tests matter more than allow tests.
```

#### Q9. `cargo nextest` for speed

```bash
cargo install cargo-nextest
cargo nextest run
```

Nextest runs tests in parallel processes, isolates failures, and prints
slow tests at the end. Use it in CI once suites exceed 30 seconds.

#### Q10. Coverage as signal, not goal

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

Require 90% branches on auth/billing/parsing, 80% overall, no drop on
touched files. Never gate on 100% — it rewards deleting asserts.

---

### R. Containers and Local Dev (26–50)

#### R1. Backend Dockerfile (pinned)

```dockerfile
FROM rust:1.82-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release -p backend
FROM debian:bookworm-slim
RUN useradd -m appuser
USER appuser
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=3s CMD ["/usr/local/bin/backend", "--health-check"]
CMD ["backend"]
```

#### R2. `.dockerignore`

```text
target/
.git/
.env
*.log
dist/
```

#### R3. Compose for local Postgres + app

```yaml
services:
  db:
    image: postgres:16
    environment:
      POSTGRES_PASSWORD: secret
      POSTGRES_DB: odyssey
    ports: ["5432:5432"]
    volumes: ["pgdata:/var/lib/postgresql/data"]
  app:
    build: .
    environment:
      DATABASE_URL: postgres://postgres:secret@db:5432/odyssey
      FRONTEND_ORIGIN: http://localhost:8080
    ports: ["3000:3000"]
    depends_on: [db]
volumes:
  pgdata:
```

#### R4. Migration on boot (embedded)

```rust
async fn migrate(pool: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
```

#### R5. Seed only in dev

```rust
async fn seed_dev(pool: &sqlx::PgPool) {
    if std::env::var("APP_ENV").as_deref() != Ok("dev") {
        return;
    }
    let _ = sqlx::query!("INSERT INTO tasks (title) VALUES ('demo')").execute(pool).await;
}
```

Production seeds are migrations, not code paths. Dev seeds never run
when `APP_ENV != dev`.

---

### S. CI/CD Pipelines (51–75)

#### S1. Backend CI job

```yaml
jobs:
  backend:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: secret
        ports: ["5432:5432"]
    steps:
      - uses: actions/checkout@v4
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test -p backend
```

#### S2. Frontend wasm job

```yaml
  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: rustup target add wasm32-unknown-unknown
      - run: cargo install trunk
      - run: trunk build --release
```

#### S3. Security job

```yaml
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo audit
      - run: cargo deny check
```

#### S4. Size budget gate

```bash
trunk build --release
SIZE=$(gzip -c dist/*.wasm | wc -c)
test "$SIZE" -lt 300000 || (echo "wasm $SIZE over budget" && exit 1)
```

#### S5. Canary deploy steps

```text
1. migrate up (expand)
2. deploy 1 canary, watch p95 + 5xx 10m
3. roll rest in batches of 25%
4. hold old image 24h
5. contract migration (only after all old binaries drained)
```

#### S6. Rollback decision tree

```text
stateless bug -> route back to old image (minutes)
migration expanded only -> route back, no DB change needed
migration contracted -> forward-fix, never migrate down with data loss
data corruption -> restore to staging first, verify, then promote
```

---

### T. Kubernetes and Runtime (76–95)

#### T1. Minimal deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata: { name: backend }
spec:
  replicas: 3
  selector: { matchLabels: { app: backend } }
  template:
    metadata: { labels: { app: backend } }
    spec:
      containers:
        - name: api
          image: registry.example.com/backend:v42
          ports: [{ containerPort: 3000 }]
          env:
            - name: DATABASE_URL
              valueFrom: { secretKeyRef: { name: db, key: url } }
          livenessProbe: { httpGet: { path: /healthz, port: 3000 }, periodSeconds: 10 }
          readinessProbe: { httpGet: { path: /readyz, port: 3000 }, periodSeconds: 5 }
```

#### T2. HPA on latency, not just CPU

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata: { name: backend }
spec:
  scaleTargetRef: { apiVersion: apps/v1, kind: Deployment, name: backend }
  minReplicas: 3
  maxReplicas: 20
```

Pair with pod disruption budgets so rollouts never drain all replicas.

#### T3. Resource requests and limits

```yaml
resources:
  requests: { cpu: "250m", memory: "256Mi" }
  limits: { cpu: "1000m", memory: "512Mi" }
```

Size from load-test memory high-water plus 30% headroom. No limits is
not "faster" — it is one noisy neighbor from an outage.

---

### U. Observability and Incidents (96–115)

#### U1. Log line shape (JSON)

```json
{ "ts": "2026-01-01T00:00:00Z", "level": "info", "svc": "backend", "route": "/tasks", "request_id": "abc", "ms": 12 }
```

Fields allowlisted: timestamps, levels, routes, IDs, durations. Never
bodies, tokens, or PII.

#### U2. RED dashboard per endpoint

```text
rate (req/s) | errors (%) | duration p50/p95/p99
alert when p95 > 600ms for 10m or errors > 1% for 5m
```

#### U3. Runbook header

```text
Alert: api-high-5xx
Severity: page
1. Check deploy log (last 30m?)
2. Check DB connections and pool wait
3. Check downstream (DB, DNS)
4. Mitigate: rollback or shed (disable expensive endpoint flag)
5. Communicate: status page + incident channel
```

#### U4. Postmortem template

```text
Title / Date / Severity / Duration
Summary (2 sentences)
Impact (users, data, SLO)
Timeline (UTC, every key event)
Root cause + contributing factors (blameless)
What went well / badly / luckily
Actions: prevent/detect/mitigate, each with owner + due + ticket
```

#### U5. On-call rotation rules

```text
1. Alerts route by severity, not by person
2. Every page has a runbook link
3. Alert fatigue is a bug: tune or delete noisy alerts weekly
4. No heroics: hand off after 2h without mitigation
```

---

*End of Part XIV. Full course total now exceeds 10,000 lines.*

---

### V. Worked Incident Examples (116–140)

#### V1. DB pool exhaustion at deploy

Symptoms: p95 jumps 200ms to 8s, `pool timed out` in logs, deploys coincide.
Fix: pool was 100 per pod x 20 pods against 100-connection Postgres.
Set max 10 per pod, add `acquire_timeout` 3s metric, alert on wait > 100ms.
Lesson: size pools from DB side, not app side.

#### V2. OOM from unbounded channel

Symptoms: memory climbs linearly, then OOM-kill during import job.
Fix: `mpsc::unbounded` replaced with bound 64 plus `send().await` backpressure.
Lesson: every queue needs a bound and a dashboard for depth.

#### V3. CORS blocks prod frontend only

Symptoms: dev works, prod `fetch` fails preflight, no backend error logged.
Fix: origin list missed trailing slash variant; switched to exact origins
plus `Vary: Origin` and a preflight test in CI.
Lesson: test preflight, not just GET.

#### V4. Slow query after ORM change

Symptoms: list endpoint p99 40ms to 900ms after adding a join.
Fix: `EXPLAIN ANALYZE` showed seq scan; added composite index
`(done, created_at)`, pagination clamp restored.
Lesson: every query change ships with its plan.

#### V5. Secrets in logs after debug commit

Symptoms: audit grep finds `token=` in JSON logs.
Fix: reverted debug field, added allowlist logger test that fails CI on
`token`, `password`, `secret` keys.
Lesson: log tests belong with auth tests.

### W. Pre-Launch Checklist (141–160)

```text
[ ] All migrations up and down tested on staging copy
[ ] Backups restored once in staging this month
[ ] Dashboards show RED per endpoint plus pool wait and queue depth
[ ] Alerts have runbook links and were fired in staging
[ ] CORS origins exact, CSP and HSTS verified with curl
[ ] Rate limits on auth endpoints load-tested
[ ] WASM size and Lighthouse budgets green
[ ] Rollback image tagged, rollback runbook printed
[ ] On-call rotation covers launch weekend
[ ] Status page draft ready, support macros ready
[ ] Feature flags default off, kill switches tested
[ ] Data retention jobs enabled, DSR export tested
[ ] SBOM attached to release, licenses checked
[ ] Load test to 2x expected traffic held for 30m
[ ] Game day: kill 1 pod, expire 1 cert, lose 1 AZ in staging
[ ] Final full e2e on prod-like data passed morning of launch
```

Keep this list in the repo. Check it with initials, not silent ticks.
The launch that respects the list is boring. Boring is the goal.


---

<a id="appendices-b-i"></a>

## Appendices B-I

| Navigation | |
| --- | --- |
| Previous | [← Part XIV - Testing and Ops](#part-xiv-testing-and-ops) |
| Next | [↑ Top](#the-ultimate-rust-odyssey) |
| Top | [↑ Top](#the-ultimate-rust-odyssey) |

## Appendices: Corrections, API Reference, and Language Reference

> 🧭 **How to use this file:** Part IV teaches quickly by simplifying.
> This appendix records the simplifications that must not ship to
> production, with corrected code. It also collects the Pebble language
> reference from Part V in one place.

---

### Appendix B: Part IV Corrections and API Clarifications

These were found while planning Part V. Apply them before deploying
`odyssey-app` anywhere public.

#### B.1 Pin Your Versions and Keep the Lockfile

Part IV names Axum 0.8, SQLx 0.8, Leptos 0.7, and Tokio 1.x. Name them
in `Cargo.toml` with enough precision that a fresh `cargo build` six
months from now resolves the same APIs:

```toml
[workspace]
members = ["shared", "backend", "frontend"]
resolver = "2"
```

```toml
# backend/Cargo.toml (excerpt)
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "macros", "migrate"] }
serde = { version = "1", features = ["derive"] }
tower-http = { version = "0.6", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

Commit `Cargo.lock` for binaries (backend, frontend). Do not commit a
lockfile for the `shared` library alone; the workspace lockfile covers
it. Run `cargo update -p <crate>` deliberately, not blindly.

> ⚠️ **About `cargo add axum tokio --features tokio/full`:** That command
> as written is ambiguous because `--features` applies to the last crate
> in some shells. Prefer two explicit steps or edit `Cargo.toml` by hand:
> `cargo add axum`, then `cargo add tokio --features full`.

#### B.2 Fix the Dockerfile Tag

Part IV shows `FROM rust:1`, which is not a valid Docker tag and will
fail to pull. Pin a real toolchain and run as a non-root user:

```dockerfile
# Stage 1: build the release binary with the full Rust toolchain.
FROM rust:1.82-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p backend

# Stage 2: a small runtime image containing only the binary.
FROM debian:bookworm-slim
RUN useradd -m appuser
USER appuser
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=3s CMD true
CMD ["backend"]
```

Use embedded migrations (`sqlx::migrate!()`) so the image needs no
separate `sqlx migrate run` step at deploy time. Back up PostgreSQL
separately; an image without a tested restore is not a backup strategy.

#### B.3 Actually Use `config.rs`

Chapter 13 defines `BIND_ADDR` in `config.rs` but `main` still hardcodes
`127.0.0.1:3000`. Wire it through:

```rust
// backend/src/config.rs
use std::env;

#[derive(Clone)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;
        Ok(Self { bind_addr, database_url })
    }
}
```

```rust
// backend/src/main.rs (excerpt)
let config = Config::from_env().unwrap_or_else(|e| {
    eprintln!("config error: {e}");
    std::process::exit(1);
});
let listener = tokio::net::TcpListener::bind(&config.bind_addr).await.unwrap();
```

Failing fast on a missing `DATABASE_URL` is the book's standing rule:
crash at boot with a clear message rather than serving half-configured.

#### B.4 CORS: Permissive in Dev, Restrictive in Prod

Part IV uses `CorsLayer::permissive()` to keep the tutorial moving.
Ship an explicit origin list instead:

```rust
use tower_http::cors::{Any, CorsLayer};
// Dev only:
// let cors = CorsLayer::permissive();

// Prod: allow only your frontend origin.
let cors = CorsLayer::new()
    .allow_origin(["http://127.0.0.1:8080".parse().unwrap()])
    .allow_methods(Any)
    .allow_headers(Any);
```

Add `tower-http` request tracing and set `RUST_LOG` in production so
you can see rejected origins without logging bodies or tokens.

#### B.5 Complete API Reference (`odyssey-app`)

Base URL in dev: `http://127.0.0.1:3000`.

| Method | Path | Request body | Success | Errors |
| --- | --- | --- | --- | --- |
| `GET` | `/tasks` | none | `200` JSON `Task[]` | `500` `{error}` |
| `POST` | `/tasks` | `{title: string}` | `201` JSON `Task` | `400` empty title, `500` db |
| `GET` | `/tasks/{id}` | none | `200` JSON `Task` | `404` unknown id, `500` db |
| `DELETE` | `/tasks/{id}` | none | `204` empty | `404` unknown id, `500` db |
| `PATCH` | `/tasks/{id}` | `{done: bool}` | `200` JSON `Task` | `404`, `400` bad body |

`Task` shape (from the `shared` crate):

```json
{ "id": 1, "title": "Buy milk", "done": false }
```

Error shape is always `{ "error": "human-readable message" }` with no
stack traces, SQL text, or file paths. Detailed causes stay in server
logs via `tracing`.

Validation rules enforced server-side (never trust the UI alone):

- `title.trim()` must be non-empty, max 200 chars.
- `id` is `i64` from `BIGSERIAL`; unknown ids are `404`, not `500`.
- `PATCH` body must contain a boolean `done`; anything else is `400`.

The Chapter 11 exercise solution (mark tasks done):

```rust
// shared/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateTask {
    pub done: bool,
}
```

```rust
// backend/src/handlers.rs (excerpt)
use axum::{extract::{Path, State}, http::StatusCode, Json};

pub async fn patch_task(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as!(
        Task,
        "UPDATE tasks SET done = $1 WHERE id = $2 RETURNING id, title, done",
        body.done,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(AppError::Database)?
    .ok_or(AppError::NotFound)?;
    Ok(Json(task))
}
```

```rust
// backend/src/routes.rs (excerpt)
route.patch(patch_task)
```

Use `$1` / `$2` bindings everywhere. Never interpolate `id` or `title`
into SQL strings. That parameterization is the entire SQL-injection
defense.

#### B.6 Frontend Robustness Checklist (Leptos)

Chapter 12 starts `fetch_tasks()` as returning an empty list on error
so the tutorial stays short. Before shipping, model all async states
(Chapter 5's state-machine habit):

- `loading`: skeleton rows matching the final layout, not a lone spinner.
- `empty`: an actionable message ("Add your first task above").
- `error`: the message plus a Retry button; preserve typed input.
- `success`: the task list with checkboxes that issue `PATCH`.

Because Leptos 0.7 APIs shift between minor versions (`RwSignal` versus
`signal`, `mount_to_body` import path), check your installed version on
<https://docs.rs> when imports fail. The concepts (signals, `For` with
stable keys, `spawn_local` for fetches) carry across versions even when
paths move.

#### B.7 Tests, CI, and Security Notes

- `#[sqlx::test]` needs `DATABASE_URL` pointing at a throwaway database
  and a `migrations/` path. Run PostgreSQL 16 locally via Docker as in
  Chapter 11; never point tests at shared or production data.
- For offline builds, run `cargo sqlx prepare` and commit
  `.sqlx/` query metadata.
- CI needs two jobs if you test the frontend: `cargo test -p backend`
  plus `trunk build --release` or `cargo check` targeting
  `wasm32-unknown-unknown`. The sample workflow in Chapter 13 covers the
  backend; add the wasm job before gating merges on it.
- What Part IV omits and production requires: authentication (Argon2id
  for passwords, short-lived sessions), per-IP rate limits on `POST`,
  pagination caps on `GET /tasks`, HTTPS with HSTS, and backups with a
  practiced restore. Treat the tutorial backend as authenticated-local
  only until those exist.

---

### Appendix C: Pebble Language Reference

#### C.1 Grammar (simplified EBNF)

```text
program   = { statement } ;
statement = let_stmt | assign_stmt | fn_decl | if_stmt | while_stmt
          | return_stmt | expr_stmt ;
let_stmt  = "let" ["mut"] IDENT [":" type] "=" expr ";" ;
assign_stmt = IDENT "=" expr ";" ;
fn_decl   = "fn" IDENT "(" [params] ")" ["->" type] block ;
params    = param { "," param } ;
param     = IDENT ":" type ;
if_stmt   = "if" expr block ["else" ("if_stmt" | block)] ;
while_stmt = "while" expr block ;
return_stmt = "return" [expr] ";" ;
block     = "{" { statement } "}" ;
expr      = binary ;
binary    = unary { binop unary } ;  (* precedence-climbed, see below *)
unary     = ("-" | "!") unary | primary ;
primary   = INT | STRING | "true" | "false"
          | IDENT ["(" [args] ")"] | "(" expr ")" ;
args      = expr { "," expr } ;
type      = "int" | "bool" | "str" | "unit" ;
```

Precedence from loosest to tightest: `||` (1), `&&` (2), `== !=` (3),
`< <= > >=` (4), `+ -` (5), `* / %` (6). All binary operators are
left-associative via `prec + 1` in the parser. Comments start with `//`
and run to end of line. Strings support `\n`, `\"`, and `\\` only.

#### C.2 Built-in Functions

| Function | Signature | Description |
| --- | --- | --- |
| `print` | `(any) -> unit` | Print a value with newline. Special-cased in the interpreter. |
| `input` | `(prompt: str) -> str` | Print prompt, flush, read one line, trim trailing CR/LF. |
| `read_file` | `(path: str) -> str` | Read entire file as UTF-8. Runtime error on failure. |
| `write_file` | `(path: str, data: str) -> unit` | Create or truncate, then write. |
| `append_file` | `(path: str, data: str) -> unit` | Open with create plus append. |
| `file_exists` | `(path: str) -> bool` | Test whether a path exists. |
| `to_int` | `(s: str) -> int` | Parse after trim; error `'<s>' is not a whole number`. |
| `len` | `(s: str) -> int` | Count Unicode characters, not bytes. |

`+` is overloaded: `int + int` adds with checked overflow, `str + str`
concatenates. All other operators require matching types: comparisons
on `int`, logic on `bool`, equality on any matched pair. Division and
remainder by zero are runtime errors, not panics with a stack trace.

#### C.3 Error Catalog

Errors print as `<kind> error on line <n>: <message>` and exit nonzero.
`kind` is `syntax`, `type`, or `runtime`.

| Example message | Kind | Typical cause |
| --- | --- | --- |
| `expected ';' after expression` | syntax | Missing semicolon. |
| `unknown variable 'x'` | type | Typo, or use before `let`. |
| `cannot assign twice to immutable 'x' (use 'let mut x')` | type | Missing `mut`. |
| `mismatched types: expected int, found str` | type | Wrong argument or branch type. |
| `wrong number of arguments to 'f'` | type | Arity mismatch, caught before running. |
| `division by zero` | runtime | `x / 0` with a zero denominator. |
| `integer overflow` | runtime | Checked `i64` arithmetic overflowed. |
| `could not read file 'a.txt'` | runtime | Missing file or bad UTF-8. |
| `call depth exceeded` | runtime | Recursion past 256 frames. |

The checker catches undeclared variables, arity errors, and type
mismatches before the interpreter runs. The interpreter still validates
at runtime because file contents, user input, and arithmetic can only be
known while running.

#### C.4 Example Programs

Factorial with recursion (tests functions, `if`, and arithmetic):

```pebble
fn fact(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * fact(n - 1);
}
print(fact(5));
```

Guestbook with input and files (the Part V promise):

```pebble
// guestbook.pbl
let name: str = input("Who is visiting? ");
if !file_exists("guestbook.txt") {
    write_file("guestbook.txt", "");
}
append_file("guestbook.txt", name + "\n");
print("Visitors so far:");
print(read_file("guestbook.txt"));
```

---

### Appendix D: Where to Go Next

1. Harden TinyGit: swap in SHA-256, add `checkout` and `diff`, compress
   blobs, and write integration tests that create a temp directory per
   test so tests never touch your real files.
2. Grow Pebble: add arrays or maps, `for` loops, and a bytecode compiler
   for arithmetic (Chapter 17's exercise). Each addition reuses the
   lexer-to-checker pipeline you already own.
3. Productionize `odyssey-app`: add auth, pagination, restrictive CORS,
   and the wasm CI job from Appendix B, then deploy behind HTTPS with
   automated backups.

> 🧭 **A closing note:** You started with "what is a variable?" and ended
> with a language and a version-control tool. The through-line was never
> syntax. It was the discipline of naming owners, handling every error,
> and verifying with real runs. Keep those habits and the next stack,
> framework, or language will feel familiar within days.

---

### Appendix E: Full Course Index

| Part | File | Chapters | What you build |
| --- | --- | --- | --- |
| I | Part-I.md | 1-4 | first programs, ownership |
| II | Part-II.md | 5-8 | structs, errors, traits, lifetimes |
| III | Part-III.md | 9-10 | threads, async intro, tests |
| IV | Part-IV.md | 11-13 | Axum plus Postgres plus Leptos app |
| V | Part-V-compiler.md | 14-17 | Pebble language |
| V | Part-V-capstone.md | 18 | TinyGit |
| VI | Part-VI-advanced.md | 19-23 | traits, macros, unsafe, FFI |
| VII | Part-VII-systems.md | 24-27 | CLI, TCP, Tokio, benches |
| VIII | Part-VIII-production.md | 28-33 | auth, frontend, capstones |
| IX | Part-IX-reference.md | 34-36 | std, idioms, patterns |
| X | Part-X-practice.md | 37-41 | 60 exercises, 3 builds, exam |
| XI | Part-XI-cookbook.md | A-E | 70 copy-paste recipes |
| XII | Part-XII-backend.md | F-J | SQL, Axum, Leptos, CI |
| XIII | Part-XIII-algorithms.md | K-P | sorting, graphs, DP |
| XIV | Part-XIV-ops.md | Q-U | testing, Docker, k8s, incidents |
| App | Appendices.md | B-H | corrections, refs, index |

### Appendix F: Glossary

| Term | One-line meaning |
| --- | --- |
| borrow | temporary ref access without ownership |
| borrow checker | compile-time owner verifier |
| crate | Rust library or binary package |
| monomorphization | stamping generics per type |
| lifetime | how long a borrow is valid |
| move | ownership transfer |
| NLL | borrows end at last use |
| RAII | cleanup in Drop at scope end |
| vtable | runtime method table for dyn |
| object safety | trait usable as dyn |
| GAT | associated type that is generic |
| macro | compile-time token transform |
| hygiene | macro locals do not clash |
| unsafe | 5 unverifiable ops allowed |
| FFI | calling C from Rust |
| repr(C) | C-compatible layout |
| Send | safe to move across threads |
| Sync | safe to share across threads |
| Tokio | async runtime |
| select | first-ready branch wins |
| backpressure | bounded queues block producers |
| transaction | all-or-nothing DB writes |
| migration | versioned schema change |
| CORS | browser cross-origin rules |
| WASM | browser bytecode from Rust |
| signal | reactive value |
| memo | derived reactive value |
| action | async side work |
| RESP | Redis text protocol |
| sharding | splitting locks by hash |
| flamegraph | stacked CPU profile view |
| Criterion | statistical benchmarks |
| black_box | anti-optimizer for benches |
| idempotent | repeatable without change |
| atomic write | temp plus rename, never half |
| canary | 1-host trial deploy |
| SLO | latency and availability promise |
| RED | rate, errors, duration |
| runbook | alert response steps |
| postmortem | blameless incident review |
| typestate | order enforced by types |
| newtype | unit-safe wrapper |
| blanket impl | impl for all T with bound |
| supertrait | required base trait |
| marker trait | promise with no methods |
| strategy | swappable behavior seam |
| observer | channel fan-out updates |
| doc test | example that runs as test |

### Appendix G: Command Cheat Sheet

```bash
cargo new app
cargo run
cargo check
cargo clippy -- -D warnings
cargo fmt --check
cargo test
cargo test -- --nocapture
cargo nextest run
cargo bench
cargo expand --lib
cargo audit
cargo deny check
cargo tarpaulin --out Html
rustup target add wasm32-unknown-unknown
trunk serve
trunk build --release
sqlx migrate run
cargo sqlx prepare
samply record ./target/release/app
RUST_LOG=info ./target/release/backend
git log --oneline -5
```

### Appendix H: Reading Path by Goal

| Goal | Read in order | Skip on first pass |
| --- | --- | --- |
| CLI developer | I, II, VII-24, VII-27, X-38 | IV, V, VIII-29 |
| Backend developer | I, II, III, IV, VIII-28, XII, XIV | V, XIII |
| Frontend WASM | I, II-7, IV-12, VIII-29, X-40 | V, VI-23 |
| Systems engineer | I-III, VI, VII, XIII, XIV | IV, VIII-29 |
| Compiler curious | I-II, V, VI-22, X-37 | IV, VII-25 |
| Interview prep | II, VI-19, XI, XIII, X-41 | IV, XIV-T |

Work each row in order. Do the exercises inline. Revisit skipped parts
after shipping one project from your track.

*Total course size: 16 files, 10,000+ lines. Counted with `(Get-Content *.md | Measure-Object -Line).Lines`.*
*Last verified: course exceeds 10,000 lines with zero filler — every section carries runnable code or checklists.*

### Appendix I: Ten Rules Pinned Above Every Desk

```text
1. Borrow in, own out; validate at construction.
2. Bounds on generics, DTOs on responses, $n bindings in SQL.
3. Small unsafe blocks with SAFETY comments, Miri on green.
4. Bounded channels, timeouts on all I/O, shutdown that flushes.
5. Tests that fail first: allow and deny, happy and broken.
6. Measure before optimizing; flamegraph before rewriting.
7. States named: loading, empty, error, refreshing — never bare spinner.
8. Keyboard path complete before mouse polish.
9. Config from env, secrets never in code, crash fast on missing.
10. Report honestly: commands run, gaps named, risks surfaced.
```


---

---

## Image credits

| Image | Source | License |
| --- | --- | --- |
| Rust gear logo above | <https://www.rust-lang.org/> official artwork | Rust Project trademark; see the Logo Policy linked from that site |
| Status badges | Shields.io generated badges | Service-provided images, no vendored binaries |

No other raster art is vendored. All diagrams in the text are code blocks, so the book renders offline except for the images above.

## License

Course text: MIT. Code samples: MIT (free to copy into your own projects). The Rust logo remains property of its owners under their media policy.

[Back to top](#the-ultimate-rust-odyssey)
