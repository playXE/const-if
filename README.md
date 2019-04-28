# const-if

This crate adds if-elif-else expression into your constant functions

# Why
Since `if` expression not implemented in current implementation of const-fn I decided to create this macro

# Example
```rust
    const fn min(x: i32, y: i32) -> i32 {
        const_if!(x < y => x;y)
    }
```
```rust
    const fn int_to_str(i: i32) -> &'static str {
        const_if!(
            i == 0 => "Zero";
            elif i == 1 => "One";
            elif i == 2 => "Two";
            elif i == 3 => "Three";
            elif i == 4 => "Four";
            elif i == 5 => "Five";
            elif i == 6 => "Six";
            elif i == 7 => "Seven";
            elif i == 8 => "Eight";
            elif i == 9 => "Nine";
            elif i == 10 => "Ten";
            else "Unknown"
        )
    }

```