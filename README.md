# AdventOfCode-2022
Learning a new language with [AoC2022](https://adventofcode.com/2022) - [Rust](https://www.rust-lang.org/)!

Using [VS code](https://code.visualstudio.com/) with [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) (duh!).

## Day 1

Took a lot more time than anticipated 😒. File reading is a bit too convoluted and unclear. Optionals everywhere. Also having to use `mut` on most variables seems wrong somehow... Overall first impression - not great! I'm up to a <i>fun</i> month 😅.

## Day 2

Relatively easy. Not the most optimal solution in terms of lines of code, but it works. Used `.expect()` to handle optionals (`Result` in Rust language) and tried using `match` expressions instead of `if`. They are quite nice. So far so good. Getting a lot of compiler warnings for unused functions from day 1 though, might get annoying...

## Day 3

Used a set for part 1 (could have also used an array I guess). For part 2 I had to convert the Lines iterator to `peekable` to be able to use `peek` method, which does not consume the iterator (move it forward). The new "peekable iterator" also has to be mutable (`mut`) for some reason. Overall relatively smooth sailing so far 😎.

## Day 4

Easy, no problems. Used `split()` and `collect()` for input parsing in addition to `expect()`, although I could have also used `unwrap()`, but it's not recommended (according to VS Code Rust plugin). A strange pattern I'm noticing is that part 2 is usually a bit easier than part 1. Or am I doing something wrong?

## Day 5

The hardest and most tedious part by far was parsing the input. I solved this by simulating the stacks - used a Rust version of a stack - `Vec` (basically a list/vector in C++), but had to push in front, so I used `VecDeq` instead (supports both `push_back()` and `push_front()`). From there I just simulated the crane moving the crates. Maybe not the most optimal, but I got to use Rust collections a bit more and `match` actually came in handy - the traditional way without `match` does not work in reality due some "mutable reference borrowing" magic I don't quite understand (yet!).

