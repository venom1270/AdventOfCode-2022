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

## Day 6

Relatively easy. Had some complications with borrowing again. The line 

> let line = lines.next().unwrap().unwrap();

does not work, I had to declare `lines` as `mut`. I have some idea why, but still not 100% sure. Also there are way too many `unwrap()`-s neccessary. There is also no quick way to get a char at some index of a string, you have to use `.nth(n)` which is an iterator and thus has a complexity of `O(n)`! That may stem from strings being UTF-xy, but it just introduces unnecessary complexity, that is not "hidden" well. Similarly with the function `find()`, which does not return a character index, but a byte index! 🤯 Works well with ASCII, but otherwise it's not guaranteed to return a true character index. With all the worry about memory management and all the bugs that can come of it, it seems strange nobody considered this kind of behaviour to be dangerous. 🤔 Maybe I'm just missing something...

## Day 7

Right... I spent WAY TOO MUCH TIME on this day. I wanted to do a proper simulation of the file system console and implement structs with methods to control all the actions. The problem was in the memory manegement and borrowing system. It seems kind of impossible to have a recursive struct reference to self in (safe) Rust. There are some workarounds, but there are "roadblocks" almost everywhere. I get the point, but it just made this task a lot more tedious that it had (any right) to be. I literally spent multiple hours here, and there is so much code...

The end result works, the only optimization that I ran out of time to implement (have to go to sleep!! 😴) is the usage of HashMap to find directory based on name we want to move to (`cd` command in the task description).

A lot of the times I also just used VS Code autocomplete and "quick fixes", in addition to trial&error for correct placement of all the `&` symbols. And conversions between `&str` and `String` are very convoluted and lead to all sorts of issues (mainly with borrowing of `String` and then having to change to `&str` or come up with some other not-so-clean-looking solution).

Anyway, an annoying day, but I did write quite a bit of Rust and encountered all sorts of problems, so that makes for a great learning experience! Have to stay positive! 🙌

## Day 8

This one was short and easy. Used `Vec<Vec<>>` and simple checks to determine result. No (major) complications 😎.

## Day 9

Nice task, but kinda got stuck on part 2. I have a small bug which produces a wrong answer on my input, but all the test cases work, so it's going to take some more debugging to figure out where th problem is.

## Day 10

Fairly straightforward. I really enjoyed seeing the letters pop out at the end. Satisfying 😄. Nothign much to add here, I haven't used any new Rust constructs.

## Day 11

Not that difficult, but input parsing was a nightmare 😵‍💫. For part 2 I had to use `u64` types and also a trick to mod each number with LCM of all test division numbers. Used enums for the first time - they make code a lot more fancy and readable, I have to use them more often movign forward! Otherwise GG, done day 10 and day 11 today 🥳. Still have to debug day 9 though...