# RustPlayground Experiments

This directory holds the runnable experiments that the wiki reflects. The
experiments are intentionally small and bottom-up: each one adds a concrete
thing to the current Rust model.

## 1. `hello_world`

Path: [`hello_world/`](hello_world/)

What it covered:

- writing a Rust source file named `main.rs`
- defining `fn main() {}`
- printing with `println!("Hello, world!");`
- compiling directly with `rustc main.rs`
- running the compiled binary with `./main`

Current model:

`main.rs` is source code. `rustc main.rs` compiles it. `./main` runs the
compiled executable.

## 2. `hello_cargo`

Path: [`hello_cargo/`](hello_cargo/)

What it covered:

- creating a Cargo package with `cargo new hello_cargo`
- seeing `Cargo.toml`
- seeing `Cargo.lock`
- editing or inspecting `src/main.rs`
- running the package with `cargo run`
- checking the package with `cargo check`

Current model:

Cargo organizes a Rust project as a package. `Cargo.toml` is the manifest,
`src/main.rs` is the starter executable source file, and Cargo commands operate
from that package structure.

## 3. `hello_variables`

Path: [`hello_variables/`](hello_variables/)

What it covered:

- creating another Cargo package
- binding a name with `let name = "Eli";`
- printing a variable with `println!("Hello, {name}!");`
- checking the package with `cargo check`

Current model:

`let` gives a value a name. `println!` can use that name inside `{}` in the
format string.

## 4. `hello_comments`

Path: [`hello_comments/`](hello_comments/)

What it covered:

- creating another Cargo package
- writing a standalone `//` comment
- writing an end-of-line `//` comment after code
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`//` comments are source text for people reading the program. They do not
change the program output.

## 5. `hello_mutability`

Path: [`hello_mutability/`](hello_mutability/)

What it covered:

- using comments to annotate the source
- seeing `cargo check` reject reassignment without `mut`
- making a binding mutable with `let mut name = "Eli";`
- reassigning the binding with `name = "Rust";`
- running the package with `cargo run`

Current model:

Rust bindings are immutable by default. `mut` marks a binding as one that can
be assigned a new value later.

## 6. `hello_shadowing`

Path: [`hello_shadowing/`](hello_shadowing/)

What it covered:

- using comments to annotate the source
- creating a binding with `let name = "Eli";`
- creating a second binding with the same name using `let name = "Rust";`
- seeing that `cargo check` accepts shadowing
- running the package with `cargo run`

Current model:

Repeating `let` with the same name creates a new binding. The newer binding
shadows the older binding.

## 7. `hello_scope`

Path: [`hello_scope/`](hello_scope/)

What it covered:

- using comments to annotate the source
- creating an inner `{}` block inside `fn main()`
- creating a binding inside that inner block
- seeing inner shadowing end when the block ends
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

Bindings live within a scope. An inner block can have its own binding with the
same name, and that inner binding stops being used after the block ends.

## 8. `hello_scope_error`

Path: [`hello_scope_error/`](hello_scope_error/)

What it covered:

- using comments to annotate the source
- seeing `cargo check` reject use of an inner-block binding after the block
- recording ``error[E0425]: cannot find value `name` in this scope``
- fixing the program by adding an outer binding
- running the fixed package with `cargo run`

Current model:

A binding created inside an inner block is not available after that block ends.
Code outside the block needs a binding that is still in scope.

## 9. `hello_integer`

Path: [`hello_integer/`](hello_integer/)

What it covered:

- binding an integer literal with `let count = 3;`
- printing that binding with `println!("Count: {count}");`
- seeing that Rust can infer the type without a written annotation
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

Bindings can name values other than string literals. `3` is an integer literal,
and Rust can infer its type in this simple program.

## 10. `hello_addition`

Path: [`hello_addition/`](hello_addition/)

What it covered:

- binding two integer literals with `let apples = 3;` and `let oranges = 4;`
- using the `+` operator in `let total = apples + oranges;`
- seeing that an arithmetic expression evaluates to one value
- printing the result with `println!("Total: {total}");`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`+` adds numeric values. `apples + oranges` is an arithmetic expression, and
the resulting value can be bound with `let`.

## 11. `hello_division`

Path: [`hello_division/`](hello_division/)

What it covered:

- binding two integer literals with `let total = 10;` and `let groups = 3;`
- using `/` in `let each = total / groups;`
- using `%` in `let leftover = total % groups;`
- naming the left and right operands of a binary operator expression
- seeing integer division keep the whole-number quotient
- seeing remainder give what is left over
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`total / groups` and `total % groups` use the same operands. With integer
values, `/` gives the quotient and `%` gives the remainder.

## 12. `hello_subtract_multiply`

Path: [`hello_subtract_multiply/`](hello_subtract_multiply/)

What it covered:

- binding three integer literals with `let starting = 12;`, `let removed = 5;`,
  and `let multiplier = 3;`
- using `-` in `let difference = starting - removed;`
- using `*` in `let product = difference * multiplier;`
- seeing subtraction produce the difference between two integer values
- seeing multiplication produce the product of two integer values
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`starting - removed` subtracts the right operand from the left operand.
`difference * multiplier` multiplies two operands.

## 13. `hello_comparison`

Path: [`hello_comparison/`](hello_comparison/)

What it covered:

- binding two integer literals with `let apples = 7;` and `let oranges = 5;`
- using `>` in `let more_apples = apples > oranges;`
- using `==` in `let same_amount = apples == oranges;`
- seeing comparison expressions evaluate to boolean values
- seeing `true` and `false` printed from boolean bindings
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`apples > oranges` and `apples == oranges` compare two operands. A comparison
expression evaluates to a boolean value: either `true` or `false`.

## 14. `hello_more_comparisons`

Path: [`hello_more_comparisons/`](hello_more_comparisons/)

What it covered:

- using `<` in `let fewer_apples = apples < oranges;`
- using `!=` in `let different_amount = apples != oranges;`
- using `>=` in `let at_least_as_many = apples >= oranges;`
- using `<=` in `let at_most_as_many = apples <= oranges;`
- seeing the remaining comparison operators evaluate to boolean values
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

Comparison expressions compare two operands and evaluate to `true` or `false`.
The first-pass comparison set is now `>`, `==`, `<`, `!=`, `>=`, and `<=`.

## 15. `hello_if`

Path: [`hello_if/`](hello_if/)

What it covered:

- using `if apples > oranges { ... }`
- using `if apples == oranges { ... }`
- seeing a true condition run its block
- seeing a false condition skip its block when there is no `else`
- using a comparison expression as an `if` condition
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`if` uses a boolean condition to decide whether a block runs. Without `else`, a
false condition skips the block and execution continues after it.

## 16. `hello_else`

Path: [`hello_else/`](hello_else/)

What it covered:

- using `if apples > oranges { ... } else { ... }`
- seeing a false condition skip the first branch
- seeing the `else` branch run when the condition is false
- using `else` as an alternative block with no condition of its own
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`if ... else` chooses between two branches. If the condition is true, the first
branch runs. If the condition is false, the `else` branch runs.

## 17. `hello_else_if`

Path: [`hello_else_if/`](hello_else_if/)

What it covered:

- using `if apples > oranges { ... } else if apples < oranges { ... } else { ... }`
- checking another condition after the first condition is false
- seeing the first true branch run
- seeing later branches get skipped after a true branch runs
- using the final `else` as the fallback branch
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`else if` adds another condition to an `if` expression. Rust checks branches in
order, runs the first branch whose condition is true, and skips the rest.

## 18. `hello_else_if_chain`

Path: [`hello_else_if_chain/`](hello_else_if_chain/)

What it covered:

- using two `else if` branches in one chain
- using `%` and `==` inside branch conditions
- seeing Rust check branch conditions in order
- seeing the first true branch run
- seeing a later true condition get skipped after an earlier true branch
- using the final `else` as the fallback branch
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

An `else if` chain can contain multiple `else if` branches. Rust checks the
branches from top to bottom and stops after the first true branch.

## 19. `hello_if_value`

Path: [`hello_if_value/`](hello_if_value/)

What it covered:

- using an `if` expression on the right side of a `let` statement
- binding the result with `let message = if ... { ... } else { ... };`
- using string literal expressions as branch values
- leaving branch value expressions without semicolons
- ending the whole `let` statement with a semicolon
- printing the chosen value with `println!("{message}");`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

An `if` expression can choose a value. The branch that runs produces the value,
and the surrounding `let` statement can bind that value to a name.

## 20. `hello_if_type_error`

Path: [`hello_if_type_error/`](hello_if_type_error/)

What it covered:

- making an `if` expression branch produce `"More apples"`
- making the `else` branch produce `0`
- seeing `cargo check` report `error[E0308]: if and else have incompatible types`
- seeing Rust expect `&str` because of the first branch
- fixing the program by making both branch values string literals
- checking the fixed package with `cargo check`
- running the fixed package with `cargo run`

Current model:

An `if` expression that produces a value needs one result type. If the branches
could produce incompatible types, `cargo check` rejects the program.

## 21. `hello_type_annotation`

Path: [`hello_type_annotation/`](hello_type_annotation/)

What it covered:

- writing an explicit type annotation with `let count: i32 = 3;`
- placing the type after the binding name and before `=`
- using `i32` as the first written integer type
- printing the annotated binding with `println!("Count: {count}");`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

Rust can infer simple types, but the source can also write a type annotation.
`let count: i32 = 3;` binds `count` to `3` and explicitly says the binding has
type `i32`.

## 22. `hello_function`

Path: [`hello_function/`](hello_function/)

What it covered:

- defining a helper function with `fn add_one(number: i32) -> i32`
- calling that helper from `main` with `add_one(count)`
- passing `count` as an argument
- using `number: i32` as a function parameter
- writing `-> i32` as the function return type
- returning the final expression `number + 1` without a semicolon
- binding the returned value with `let next_count = add_one(count);`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

A function can take a typed parameter and return a typed value. Calling the
function is an expression, so its return value can be bound with `let`.

## 23. `hello_two_parameters`

Path: [`hello_two_parameters/`](hello_two_parameters/)

What it covered:

- defining a helper function with `fn add(left: i32, right: i32) -> i32`
- writing two comma-separated parameter declarations
- calling that helper from `main` with `add(apples, oranges)`
- passing two arguments in one function call
- seeing the first argument match the first parameter
- seeing the second argument match the second parameter
- returning the final expression `left + right`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

A function can take more than one typed parameter. Function calls pass
arguments in the same order as the parameter list.

## 24. `hello_argument_order`

Path: [`hello_argument_order/`](hello_argument_order/)

What it covered:

- defining a helper function with `fn subtract(left: i32, right: i32) -> i32`
- calling `subtract(starting, removed)`
- calling `subtract(removed, starting)`
- seeing the first argument become `left`
- seeing the second argument become `right`
- using subtraction to make argument order visible
- returning the final expression `left - right`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

Function arguments match parameters by position. With subtraction, changing
argument order changes the result because `left - right` subtracts the right
operand from the left operand.

## 25. `hello_return`

Path: [`hello_return/`](hello_return/)

What it covered:

- defining a helper function with `fn add_one(number: i32) -> i32`
- using `return number + 1;`
- seeing explicit `return` send a value back to the caller
- comparing explicit `return` with the earlier final-expression return shape
- keeping the function return type as `-> i32`
- binding the returned value with `let next_count = add_one(count);`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

A function can return the final expression implicitly, or it can use `return`
to send a value back explicitly. The explicit `return` form in this experiment
still returns the same value as the earlier final-expression version.

## 26. `hello_early_return`

Path: [`hello_early_return/`](hello_early_return/)

What it covered:

- defining `fn describe_count(count: i32) -> &'static str`
- using `if count == 0 { ... }`
- using `return "none";` inside the `if` block
- seeing the function return before reaching its final expression
- using `"some"` as the final expression when the early return does not run
- returning string literals from a function
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`return` can send a value back before the function reaches its final
expression. If the early-return branch does not run, the function can still
return its final expression.

## 27. `hello_str_binding`

Path: [`hello_str_binding/`](hello_str_binding/)

What it covered:

- writing an explicit string literal binding type with `let name: &str = "Eli";`
- using `&str` as the first written string slice type
- contrasting `let name: &str = "Eli";` with the earlier inferred `let name = "Eli";`
- printing the `&str` binding with `println!("Name: {name}");`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`&str` is the type shape used for a binding that names a string literal. Rust
can infer it in simple code, but the source can also write it explicitly.

## 28. `hello_static_str_return`

Path: [`hello_static_str_return/`](hello_static_str_return/)

What it covered:

- defining `fn static_message() -> &'static str`
- returning a string literal from a no-argument function
- seeing why `fn static_message() -> &str` is too vague for this shape
- recording `error[E0106]: missing lifetime specifier`
- using `'static` to say the returned reference can live for the whole program
- binding the returned string literal with `let message = static_message();`
- printing the returned string literal with `println!("{message}");`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`&str` says "string slice reference", but a function return type also needs a
validity story for the returned reference. For a no-argument function returning
a string literal, `&'static str` supplies that story: the literal's text is
stored with the program, so the returned reference can live for the whole
program.

## 29. `hello_utf8_literal`

Path: [`hello_utf8_literal/`](hello_utf8_literal/)

What it covered:

- writing `let ascii_word: &str = "cafe";`
- writing `let utf8_word: &str = "café";`
- seeing that `"cafe"` uses only ASCII letters
- seeing that `"café"` is not ASCII because of `é`
- seeing that a non-ASCII string literal can still be valid UTF-8
- printing both `&str` bindings
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

ASCII text is the easy subset of UTF-8. A string literal can contain text that
is not ASCII and still be valid UTF-8. Rust accepts both `"cafe"` and `"café"`
as `&str` values.

## 30. `hello_utf8_len`

Path: [`hello_utf8_len/`](hello_utf8_len/)

What it covered:

- writing `let ascii_word: &str = "cafe";`
- writing `let utf8_word: &str = "café";`
- calling `.len()` on `ascii_word`
- calling `.len()` on `utf8_word`
- seeing `"cafe"` has byte length `4`
- seeing `"café"` has byte length `5`
- seeing `.len()` on `&str` return bytes, not visible letters
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`.len()` on `&str` reports byte length. ASCII text stays simple because each
ASCII character is one byte in UTF-8, while non-ASCII UTF-8 text can take more
bytes than it appears to have letters.

## 31. `hello_str_len_syntax`

Path: [`hello_str_len_syntax/`](hello_str_len_syntax/)

What it covered:

- reusing `let ascii_word: &str = "cafe";`
- calling the documented `str` method with `ascii_word.len()`
- calling the same documented method with `str::len(ascii_word)`
- seeing both calls return `4`
- seeing `.` as method-call syntax
- seeing `::` in the path `str::len`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`ascii_word.len()` is the usual method-call shape. `str::len(ascii_word)` names
the method through a path and passes the value in parentheses. Both forms call
the standard-library `len` method for `str`, so the byte-length rule is
unchanged.

## 32. `hello_utf8_chars_count`

Path: [`hello_utf8_chars_count/`](hello_utf8_chars_count/)

What it covered:

- writing `let word: &str = "café";`
- calling `word.len()` to count bytes
- calling `word.chars()` to create an iterator over `char` values
- calling `.count()` on that iterator
- seeing `"café"` has byte length `5`
- seeing `"café"` has `char` count `4`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`word.len()` counts UTF-8 bytes. `word.chars()` creates an iterator over Rust
`char` values, and `.count()` consumes that iterator to count them. For
`"café"`, those two counts differ: `5` bytes and `4` `char` values.

## 33. `hello_chars_next`

Path: [`hello_chars_next/`](hello_chars_next/)

What it covered:

- writing `let word: &str = "café";`
- creating a mutable `Chars` iterator with `let mut chars = word.chars();`
- calling `chars.next()` repeatedly
- seeing `next` return `Some('c')`, `Some('a')`, `Some('f')`, and `Some('é')`
- seeing `next` return `None` after the sequence is finished
- printing `Option<char>` values with `{:?}`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

An iterator can produce items one at a time. `chars.next()` advances through
the `char` sequence from `"café"`, returning `Some(char)` while an item exists
and `None` when there is no next item.

## 34. `hello_enum`

Path: [`hello_enum/`](hello_enum/)

What it covered:

- defining a custom enum with `enum Direction { ... }`
- defining two variants, `Left` and `Right`
- constructing enum values with `Direction::Left` and `Direction::Right`
- binding those values with `let first_turn = ...` and `let second_turn = ...`
- adding `#[derive(Debug)]` before the enum definition
- printing custom enum values with `{:?}`
- seeing debug output use the variant names `Left` and `Right`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

An `enum` can be a custom type whose values are one of its variants.
`Direction::Left` and `Direction::Right` are both `Direction` values.
`#[derive(Debug)]` asks Rust to generate debug formatting so
`println!("{:?}", value)` can print those values.

## 35. `hello_match`

Path: [`hello_match/`](hello_match/)

What it covered:

- reusing the `Direction` enum from `hello_enum`
- defining a function that takes a `Direction` parameter and returns `&'static str`
- using `match turn { ... }` to branch on an enum variant
- writing match arms with `Direction::Left => "going left"`
- using `=>` to separate the pattern from the arm value
- seeing `match` choose the arm whose pattern matches the value
- using the match body as the function's final expression
- calling the function with both `Direction::Left` and `Direction::Right`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`match` looks at a value and chooses the arm whose pattern matches. Each arm
is a pattern, `=>`, and a value. Every variant of the enum must have a matching
arm.

## 36. `hello_match_option`

Path: [`hello_match_option/`](hello_match_option/)

What it covered:

- defining a function that takes an `Option<char>` parameter
- using `match item { Some(c) => ..., None => ... }`
- seeing `Some(c)` create a binding `c` for the contained `char`
- using `c` inside the arm's expression with `println!("found: {c}")`
- seeing the `None` arm run when the iterator is finished
- passing `chars.next()` directly as a function argument
- calling the function five times to see four `found:` lines and one `nothing left`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`match` on `Option<char>` can extract the value inside `Some`. The pattern
`Some(c)` creates a binding `c` that holds the contained `char`. The `None`
arm handles the case where no value is present.

## 37. `hello_for`

Path: [`hello_for/`](hello_for/)

What it covered:

- using `for c in word.chars() { ... }` to loop through each `char`
- seeing the loop body run once for each item in the iterator
- seeing `c` bound to the current `char` on each pass
- seeing the loop end after the last item
- seeing execution continue after the loop with `println!("done")`
- connecting `for` to the manual `.next()` calls in `hello_chars_next`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

A `for` loop runs a block once for each item in an iterator. The loop variable
receives the item directly, not the `Option`. The loop stops when the iterator
is exhausted.

## 38. `hello_array_for`

Path: [`hello_array_for/`](hello_array_for/)

What it covered:

- writing an array expression with `let numbers = [3, 4, 5];`
- seeing array values written as comma-separated elements inside square brackets
- using `for number in numbers { ... }` to loop through each array element
- seeing `number` bound to one integer element on each pass
- seeing the loop end after the last array element
- seeing execution continue after the loop with `println!("done")`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

An array stores multiple values in a fixed sequence. A `for` loop can run once
for each element in that array, binding the loop variable to the current
element on each pass.

## 39. `hello_array_sum`

Path: [`hello_array_sum/`](hello_array_sum/)

What it covered:

- reusing the array expression `let numbers = [3, 4, 5];`
- creating a mutable integer binding with `let mut total = 0;`
- using `for number in numbers { ... }` to visit each array element
- adding each element to the running total with `total = total + number;`
- seeing `total` keep its updated value across loop passes
- printing each running total from inside the loop
- printing the final total after the loop
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

A mutable binding outside a loop can hold a value that changes during the loop.
Each pass can read the current value, compute a new value, and assign it back
to the same binding.

## 40. `hello_plus_equals`

Path: [`hello_plus_equals/`](hello_plus_equals/)

What it covered:

- reusing the array and running-total shape from `hello_array_sum`
- replacing `total = total + number;` with `total += number;`
- seeing `+=` add the right-side value into the mutable left-side binding
- seeing the running totals stay the same as the explicit addition version
- printing the final total after the loop
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`+=` is compound assignment. `total += number;` updates the existing mutable
`total` binding by adding `number` into it.

## 41. `hello_array_index`

Path: [`hello_array_index/`](hello_array_index/)

What it covered:

- reusing the array expression `let numbers = [3, 4, 5];`
- reading one array element with `numbers[0]`
- reading another array element with `numbers[1]`
- seeing array indexes start at `0`
- binding indexed values with `let first = ...;` and `let second = ...;`
- printing the indexed values
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

Array indexing reads one element at a chosen position. For `numbers`, index
`0` reads the first element and index `1` reads the second element.

## 42. `hello_array_index_error`

Path: [`hello_array_index_error/`](hello_array_index_error/)

What it covered:

- reusing the array expression `let numbers = [3, 4, 5];`
- trying the invalid index expression `numbers[3]`
- seeing `cargo run` fail before running the program
- recording the message `this operation will panic at runtime`
- seeing the bounds detail `the length is 3 but the index is 3`
- fixing the program by reading the last valid index with `numbers[2]`
- checking the fixed package with `cargo check`
- running the fixed package with `cargo run`

Current model:

A three-element array has indexes `0`, `1`, and `2`. Index `3` is outside the
array. When Rust can see that an index is outside the array, it can reject the
program before it runs.

## 43. `hello_array_len`

Path: [`hello_array_len/`](hello_array_len/)

What it covered:

- reusing the array expression `let numbers = [3, 4, 5];`
- calling the `len` method on an array with `numbers.len()`
- binding the returned length with `let length = ...;`
- seeing the returned length is `3`
- connecting array length to the valid indexes `0`, `1`, and `2`
- distinguishing array `.len()` from `&str` `.len()`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

For `[3, 4, 5]`, `numbers.len()` returns `3` because the array stores three
elements. The highest valid index is one less than the length, so index `2` is
the last valid index.

## 44. `hello_array_last_index`

Path: [`hello_array_last_index/`](hello_array_last_index/)

What it covered:

- reusing the array expression `let numbers = [3, 4, 5];`
- computing the last valid index with `numbers.len() - 1`
- writing the index binding type explicitly with `let last_index: usize = ...;`
- using a computed index in `numbers[last_index]`
- seeing the computed index is `2`
- seeing the last array value is `5`
- recording that `len() - 1` only works when the array is not empty
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

For a non-empty array, the last valid index is one less than the array length.
`numbers.len()` returns a `usize`, and array indexing expects a `usize`, so
`last_index` can go directly inside `[]`.

## 45. `hello_empty_array_len`

Path: [`hello_empty_array_len/`](hello_empty_array_len/)

What it covered:

- creating an empty array with `[]`
- writing the full array type with `let numbers: [i32; 0] = [];`
- seeing `[i32; 0]` name the element type and length
- calling `.len()` on an empty array
- seeing the returned length is `0`
- connecting empty arrays to the `len() - 1` guardrail
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

`[]` can be an array with no elements when the type is known. `[i32; 0]` means
an array of `i32` values with length `0`, and `.len()` returns `0`.

## 46. `hello_empty_array_last_index_error`

Path: [`hello_empty_array_last_index_error/`](hello_empty_array_last_index_error/)

What it covered:

- reusing the empty array shape `let numbers: [i32; 0] = [];`
- trying to compute a last index with `numbers.len() - 1`
- seeing `cargo check` accept the package
- seeing `cargo run` panic before `println!`
- recording the message `attempt to subtract with overflow`
- connecting the failure to `usize` being unsigned
- naming the arithmetic failure as underflow, reported by Rust as overflow
- separating arithmetic overflow from array indexing

Current model:

An empty array has length `0`. Because `len()` returns `usize`, subtracting `1`
tries to compute a value below `0`. That is underflow for an unsigned integer;
Rust reports it as integer overflow in debug mode. The failure happens while
computing the index, before any array indexing happens.

## 47. `hello_array_last_index_if`

Path: [`hello_array_last_index_if/`](hello_array_last_index_if/)

What it covered:

- reusing the empty array shape `let numbers: [i32; 0] = [];`
- checking the array length with `if numbers.len() > 0`
- using a greater-than comparison as the `if` condition
- computing `numbers.len() - 1` only inside the non-empty branch
- running the `else` branch for the empty array
- printing `empty array`
- avoiding the underflow from `hello_empty_array_last_index_error`
- checking the package with `cargo check`
- running the package with `cargo run`

Current model:

An `if` condition can protect an operation that only works for some values.
For an empty array, `numbers.len() > 0` is false, so the program does not run
`numbers.len() - 1` and does not underflow.
