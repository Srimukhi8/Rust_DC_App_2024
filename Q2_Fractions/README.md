# Q2

```rust
impl<Frac> for Rust
```

In this task, you need to implement a simple struct for using fractions in Rust.

## The Task

You have been given some starting code in [`src/fraction.rs`](src/fraction.rs). You are also given two empty files [`src/fraction/ops.rs`](src/fraction/ops.rs) and [`src/fraction/conversions.rs`](src/fraction/conversions.rs). You are only allowed to modify these three files.  
You are asked to implement **Conversions** and **Operator Overloading** for the `Frac` struct.

The use-cases are mentioned briefly in the following sections.

### Conversions

In order to make the `Frac` struct more versatile, you would need it to be able to convert to and from other types. 

Here are some resources that you can refer to:
- https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
- https://www.geeksforgeeks.org/rust-from-and-into-traits/

For this part of the task, you need to implement the following conversions for the `Frac` struct. The implementation of these conversions should be done in the `src/fraction/conversions.rs` file. Some example use-cases are given below:

- **From** integers
    
    For simplicity, you will only need to implement the conversion **from** `i64` only. You can implement the following conversion functions:

    ```rust
    let a: Frac = Frac::from(1); // a = 1/1
    let b: Frac = Frac::from(-4i64); // b = -4/1
    ```

- **To** Floating-point numbers

    You would also need to implement the conversion **from** `Frac` **to** `f64`.

    ```rust
    let a: Frac = Frac::new(1, 2); // a = 1/2
    let b: f64 = f64::from(a); // b = 0.5
    ```

### Operator Overloading

Just like the primitive types such as `i64` in Rust, you should be able to use operators on the `Frac` struct. 

Here are some resources that you can refer to:
- https://doc.rust-lang.org/rust-by-example/trait/ops.html
- https://doc.rust-lang.org/std/ops/index.html

For this part of the task, you need to implement the following operators for the `Frac` struct. The implementation of these operators should be done in the `src/fraction/ops.rs` file. Some example use-cases are given below:

- Negation
    ```rust
    let a: Frac = Frac::new(1, 2);
    let b = -a;
    ```

- Addition, Subtraction, Multiplication, and Division

    ```rust
    let a: Frac = Frac::new(1, 2);
    let b: Frac = Frac::new(1, 3);

    let add: Frac = a + b;
    let sub: Frac = a - b;
    let mul: Frac = a * b;
    let div: Frac = a / b;
    ```

- Exponentiation
    ```rust
    let a: Frac = Frac::new(1, 2);
    let b: Frac = a.pow(2);
    ```

Here are some **bonus** operators that you can try to implement:

- Assignment operators
    ```rust
    let mut a: Frac = Frac::new(1, 2);
    let b: Frac = Frac::new(1, 3);

    a += b;
    a -= b;
    a *= b;
    a /= b;
    ```

- Comparison operators
    ```rust
    let a: Frac = Frac::new(1, 2);
    let b: Frac = Frac::new(1, 3);

    let eq: bool = a == b; // false
    let ne: bool = a != b; // true
    let lt: bool = a < b; // false
    let le: bool = a <= b; // false
    let gt: bool = a > b; // true
    let ge: bool = a >= b; // true
    ```

More detailed use-cases for the `Frac` struct are also included in the tests.  

- [`src/tests.rs`](src/tests.rs): contains the tests for the basic functionality of the `Frac` struct.  
- [`src/tests_bonus.rs`](src/tests_bonus.rs): contains the tests for the bonus functionality of the `Frac` struct.


### Running the tests

Make sure you are in the `Q2` directory. Then, you can run the tests using the following command:

```bash
cargo test
```

In order to run the bonus tests as well, you can use the following command:

```bash
cargo test --features bonus_tests
```
