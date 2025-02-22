Below is a programming challenge that will help you understand the difference between associated functions and methods in Rust.

---

### Problem: Rectangle Utility

**Objective:**  
Create a `Rectangle` struct that represents a rectangle with a width and a height. You will implement both an associated function and several methods on the `Rectangle` type.

**Requirements:**

1. **Define the `Rectangle` Struct:**
   - The struct should have two fields: `width` and `height` (both of type `f64`).

2. **Implement an Associated Function:**
   - Create an associated function named `new` that takes two parameters (width and height) and returns a new `Rectangle` instance.
   - **Hint:** This function is called on the type itself (e.g., `Rectangle::new(...)`).

3. **Implement the Following Methods:**
   - **area:**  
     A method that borrows the instance immutably (`&self`) and returns the area of the rectangle (i.e., width * height).
   - **perimeter:**  
     A method that borrows the instance immutably (`&self`) and returns the perimeter of the rectangle.
   - **is_square:**  
     A method that returns `true` if the rectangle is a square (i.e., width equals height) and `false` otherwise.
   - **scale:**  
     A method that takes a mutable reference to the instance (`&mut self`) and a scaling factor (of type `f64`), and multiplies both the width and height by this factor.

4. **Write a `main` Function:**
   - Use the associated function to create a new `Rectangle`.
   - Print its area, perimeter, and whether it is a square.
   - Use the `scale` method to change the dimensions of the rectangle.
   - Print the updated area and perimeter after scaling.

---

### Example Output

When you run your program, the output might look something like this (values will vary based on your test inputs):

```
Original Rectangle: width = 4.0, height = 6.0
Area: 24.0
Perimeter: 20.0
Is square: false

After scaling by a factor of 1.5:
New width = 6.0, New height = 9.0
Area: 54.0
Perimeter: 30.0
```

---

### What You'll Learn

- **Associated Functions:**  
  You'll see how functions like `new` are defined on the type and called without needing an instance.
  
- **Methods:**  
  You'll understand how methods like `area`, `perimeter`, `is_square`, and `scale` operate on instances of `Rectangle` and how mutable vs. immutable borrowing affects them.

Try solving this challenge, and you'll have a hands-on understanding of when to use an associated function versus a method in Rust!

### How to run
```bash
rustc main.rs
./main
```
