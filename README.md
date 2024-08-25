# Rust

## Cargo

It is the Rust's `build system and package manager`

### Check cargo version.

> cargo --version

### 1.0 Create cargo project.

### 1.1 Initiate cargo. 

> cargo init

### 1.2 Create a new project.

> cargo new `new project name`

### 2.0 Execute cargo.

> cargo run

### 3. Build.

> cargo build


## Common programming concepta

[Refer](https://doc.rust-lang.org/book/ch03-02-data-types.html)

### 1. Variable and Mutablity

Refer check_data function

### 2. Data type

There are two types of
1. scalar
    1. [Interger](#2.1.1)
    2. floating-point numbers
    3. Booleans
    4. Characters
2. compound

#### 2.1.1 Interger {#2.1.1}

signed interger in
> -(2^(n-1)) to 2^n - 1, eg if n=8, -128 - 127

unsigned
> 2^n -1 eg if n=8, 0 - 255 


<p> Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as `arch`: `64 bits` if you’re on a `64-bit` architecture and `32 bits` if you’re on a `32-bit` architecture.






