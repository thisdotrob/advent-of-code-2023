# Advent of Code 2023

This repo contains the solutions I've written for 2023's Advent of Code challenges, in Rust.

I'm aiming to complete all of the challenges this year whilst also using it as an opportunity to practice writing Rust. I'll therefore be balancing solving each challenge in a reasonable amount of time with spending time exploring ways of writing each in Rust.

I'll be making efforts to write clean code that is relatively readable whilst I work towards a solution, but will leave further refactoring tasks in a to do list to tackle after I've completed all the challenges.

Likewise with testing I will be adding tests as I go where I need them to help me build up a solution, but won't be writing the level of tests I would if I were working in a team and knew that other people would be reading the code and referring to them.

## Notes

This section contains some notes I've made whilst tackling the solutions so I can revisit and improve them later.

### Day 3

Rust features and idioms investigated:
- appending to a string using `+`
- using a HashSet
- stripping newlines from a string
- converting `Vec<&str>` into `Vec<Vec<char>>`
- differences between `iter()` and `into_iter()`
- difference between casting with `as` and using `try_into()` and `into()`
- reading and setting environment variables
- de-referencing and casting integers when comparing them
- grouping tests by the function being tested, running only those tests for a single function

### Day 4

Rust features and idioms investigated:
- using `split_whitespace()` to remove multiple whitespace chars
- using `2_u32` to set integer type
- the `pow()` function
- functions can't take `[&str]` for argument, must take `&[&str]` so size is known at compile time

### Day 5

Rust features and idioms investigated:
- creating a `HashMap` and updating values in it with `.entry(k).and_modify(|x| ...)`
- using tuple structs
- using `fold()`
- using `chunks()` and `chunks_exact()` to iterate over non-overlapping pairs from a vec
- trying to use pattern matching in a for loop (couldn't get this working)

### Day 6

Learnt that Rust is *fast*. I almost assumed that part 2 would need changes since it was so inefficient but thought I'd run it anyway to see how long it took. It was quick enough that no changes were needed!

Rust features and idioms investigated:
- using [[Rust/`dbg!` macro]]
    - learnt that it actually prints out the name of the variable passed to it and the line number
- assigning a range to it's own variable
    - had only used it in place before e.g. in a for loop
- experienced `PosOverflow` when trying to parse a string integer into an `i32`
    - switched to using `u64` instead

### Day 8

I decided to convert the node chars into numbers and use them as indexes into an array where the left / right instructions are stored. Considered two options to convert the chars to numbers:
- set A = 0, Z = 25 and then add the converted letters:
    - `AAA` = 0 + 0 + 0 = 0
    - `ZZZ` = 25 + 25 + 25 = 150
    - thought this might mean an array of size 150 would be needed
    - realised that it wouldn't work as there would be overlap e.g. `AAZ` == `ZAA`
- As above but bit shift the numbers:
    - 5 binary digits are needed for each char (25 = `11001`)
    - so 15 binary digits are needed for each node location
    - `ZZZ` will be `11001_11001_11001`
    - `AAA` will be `00000_00000_00000`
    - this is the option I decided on

Rust features and idioms investigated:
- applying bitwise OR and left shift
- getting the integer value of a char, changing them so A = 0, Z = 25
- setting a struct field to be a collection of chars:
```rust
struct Instructions<'a> {
    sequence: Chars<'a>,
}
```
- creating an endless iterator (would be nice to do it with without needing to `collect()` into the `Vec<char>` and use a `Chars` iterator for `sequence` instead):
```rust
struct Instructions {
    sequence: Vec<char>,
    current_index: usize,
}

impl Iterator for Instructions {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = Some(self.sequence[self.current_index]);

        self.current_index = if self.current_index == self.sequence.len() - 1 {
            0
        } else {
            self.current_index + 1
        };

        instruction
    }
}
```

### Day 9

Rust features and idioms investigated:
- using `.windows()` to iterate over overlapping pairs

### Day 10

I implemented what I later found out was a "ray tracing algorithm" to check if a point is contained in a polygon.

Rust features and idioms investigated:
- displaying stdout during test runs for passing *and* failing tests with `cargo test -- --nocapture`
- using `.any()` to check "can be one of many" conditions:
```rust
if ['-', '7', 'J']
	.iter()
	.any(|valid_symbol| *valid_symbol == symbol)
	{
		direction = Direction::Right;
	}
```
- setting a failure message in test assertions:
```rust
assert!(
 	!is_enclosed(&sketch_vec, &loop_positions, pos),
	"expected {:?} to not be enclosed",
	pos
);
```

### Day 11

Rust features and idioms investigated:
- transposing a vec of vecs
- creating a string of length `n` containing a repeated character

### Day 12

I created a dynamic programming table called `memo` to make pt2 run in an acceptable amount of time. It's still very slow.

Some thoughts on how to speed it up:
- change `memo` from being a `HashMap<[usize; 3], usize>` into a `HashMap<usize, HashMap<usize, HashMap<usize, usize>>>`
- change `memo` to be an array of arrays, `[[[usize; x]; y]; z]`, where:
    - `x` = max `springs` length - 1
    - `y` = max `pattern` length - 1
    - `z` max `springs` length - 1

Revisit the above options later and benchmark with `cargo bench`.

The code could also be tidied up if `memo` was turned into a property of a custom `Struct` that is initialised for each line. Wouldn't then need to pass `memo` in to every call to `score`.

Also did this day in Ruby - revisit and benchmark it vs. Rust.

### Day 13

Took me ages as I mis-interpreted the instructions. I thought you had to find a the line of reflection that had the *most* columns or rows reflected. You actually needed to find the one line of reflection that reflects out to at least *one* edge.

My first attempt at solving therefore created a "reflection count" for each line of reflection and then picked the highest. I calculated "reflection count" by creating a hash key for each row or column (noddy hash that just concatenated the elements) and increment the count whenever there were opposing rows/columns with a matching hash, working my way out from the line of reflection. I've saved this code on the `day_13` branch.

After realising my mistake I changed the code to find a single line of reflection and stop there, still using the hash keys to compare.

For pt2 you needed to change a single point in the grid, one that would give a different line of reflection. To do this I updated the code to allow a single character difference in the hash keys, only once per pattern.

Some thoughts on how to optimise the solution:
- compute keys on the fly instead of working them all out upfront
- use integer for keys instead of strings?
- Finding count of different characters in a string only needs to stop at 1

### Day 14

I opted to transpose the platform clockwise for pt1, since it was easier to move the boulders if working row-wise and I anticipated part 2 asking to tilt it in other directions. This turned out to be correct and I was able to reuse the row-wise tilt logic and call transpose between each call to tilt in the necessary directions.

Pt 2 still killed me though - I made the mistake of not really paying attention to how many cycles were required and thinking it was around 1 million. It was actually 1 billion. Thinking it was 1 million, I guessed that it might take a while to run but Rust would do it in a reasonable amount of time. When I came to run even the example input it was taking a loooooong time.

It took me a while to work out how to make it run faster. I noticed that the boulders should eventually settle into a repetitive loop of movements. I updated the code to track when a particular pattern on the platform had been seen before, and to stop when all patterns seen in the last 200 cycles had been seen the same number of cycles ago (i.e. all boulders had settled into a pattern where they *all* return to their original locations after the same `x` amount of cycles).

Refactoring ideas - the code is pretty horrible. I'd like to try making a custom Struct for the Platform, make transpose a method on it or keep track of the "orientation" (N, S, W, E) internally and get it into the right position when a `tilt_*` method is called (e.g. `tilt_north`). I also think I went overboard with using the `Option` type where I needn't have. Removing these and making the values required would clean things up somewhat.

Performance improvement ideas - it would be fun to try and make the `transpose` logic modify the platform in-place. At the moment it creats a new copy each time and allocates memory.

Another cool idea would be to add a terminal based graphical front end to show the boulders moving...

Rust features and idioms investigated:
- using `filter_map`, returning `Some(...)` for an element that should be kept.
- using `is_none()` to check for the `None`.
