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

Nice task, but kinda got stuck on part 2. I have a small bug which produces a wrong answer on my input, but all the test cases work, so it's going to take some more debugging to figure out where the problem is 🤔.

EDIT (13.12.2022): Finally figured it out! The problem was that I did a complete move for each knot in one operation, which meant knots did not "follow the rope", but were literally teleported totowards the correct position, hence my solution did not generate correct places the knots visited (only final positions were 100% correct). Big shoutout to [nsk4](https://github.com/nsk4) for providing me with his input/output so I could compare my results with correct ones. 🙌

Solution now contains a lot of debug code which I just commented out instead of deleting it.

## Day 10

Fairly straightforward. I really enjoyed seeing the letters pop out at the end. Satisfying 😄. Nothign much to add here, I haven't used any new Rust constructs.

## Day 11

Not that difficult, but input parsing was a nightmare 😵‍💫. For part 2 I had to use `u64` types and also a trick to mod each number with LCM of all test division numbers. Used enums for the first time - they make code a lot more fancy and readable, I have to use them more often movign forward! Otherwise GG, done day 10 and day 11 today 🥳. Still have to debug day 9 though...

## Day 12

That was a standard uni/leetcode problem - not much difficulties. Opted for a breadth first approach, mainly beacause recursion with memoization in Rust does not seem so trivial... unless there's some special trick to it. 🤔

## Day 13

Right... recursion. Seems I dogded it for just one measly day. Actually it went relatively smoothly, I just had to make sure no references are being held to an object I want to change (mutate). The code is a bit lenghty though, I'm sure there are some constructs that could make it shorter and more elegant. Input parsing was slightly tricky.

Otherwise I had some problems because I missed an edge case, but quickly found it and fixed it. For part 2 I just used bubble sorting to sort and then find the divider packets. The assumption was (that held true for my input) that divider packets are unique, so I dodn't have to mark them in any special way. My first thought was to add a `Divider(i32)` entry to `enum Element`, but that woul convolute all the `match`es taht I used throughout the code.

All in all, spent a bit more time than anticipated, but it was relatively smooth sailing otherwise.

## Day 14

This one was really easy. I just used a `HashSet` to store coordinate values (wall or sand) and kept inserting elements until stop condition. Nothing special to write about here 😄.

## Day 15

I was busy the last few days, so I'm 3 days behind! 😅 Aynway, his one one a bit tricky to me. No issues with Rust thankfully, but I did struggle a bit with finding a correct algorithm. Part 1 was done using `HashSet` and storing each coordinate in it, but that proved inefficient (i.e. slow) on part 2. So part 2 I used intervals, but it still takes a few seconds to find the distress signal. For the frequency I converted both coordinates to `u128` (although `u64` would be enough), which is big enough to hold the frequency value. Overall spent too much time tinkering with the intervals algorithm, but I managed to get it working in the end 😄.

## Day 16

Again spent wayyy too much time on this one... Rust is trying really hard to be as difficult as possible for this task. Today is not my day apparently. Still getting incorrect result for part 2. Must be a really small detail that I'm missing as the test case works fine. Very dirty solution. Debugging awaits... 🤯.

Ok, managed to solve it, but the solution is slightly fitted towards my input - might not work for all inputs 😅.

## Day 17

Part 1 done without difficulties. Tried to use enums and structs. Will do part 2 tomorrow (well, technically today...). Seems like a fun optimization challenge. I already have some ideas.

Whew, 2 and a half days in one day (well, technically a day and a bit...)! Not bad! 😎

## Day 18

This one was relatively easy. Reminds me of 3sum Leetcode question. For part 2 I just did DFS to discover air bubbles and use part 1 function to calculate number of surfaces. Implemented DFS with queue, because Rust. 😎

## Day 19

Used a straightforward approach for this one. It takes a while to complete (have to manually enable some heuristic memoization optimizations for part 1 and 2 separately to get results in real time) so it's not too optimized, but I managed to get the correct result. Used structs and an enum. Had no Rust/borrowing problems! 😎 

## Day 20

Well... I can't even get one star because I can't figure out the wrapping logic behind the procedure... don't even know how to do it by hand on paper 😭. There should be more test cases to showcase how exactly the algorithm should work 🤔.

## Day 21 

This one was relatively easy. Solved it by using a `HashMap` of monkeys and recursively calculating their numbers. No memoization, although it could make things more optimal in some cases. For part two I used a binary search to find the correct value. "Koko eating bananas" flashback - maybe that's where the inspiration for monkeys in the story came from 😎.


## Day 22 

Part 1 was easy, but part 2 is proving difficult because it's hard to test. I may have missed one digit when wrapping around the cube. I have to look at it later.

UPDATE (next day, 23. 12. 2022): Solved part 2 easily, all I needed was some sleep and a break from all the numbers 😅. There was a bug in wrapping logic - fixed some numbers and now it works 😄. The solution is hardcoded to a specific shape of input though.

## Day 23

That one was relatively straightforward and fun 😄! I used `HashMap`s where I could to gain performance. Part 2 takes a few seconds to compute.

Now I need to gather the willpower to return to day 20, and to think of a solution to part 2 of day 17...

## Day 24

Relatively simple, but used too much time here. Used A*, but got confused with memoization so I was stuck for an hour figuring out the meaning of exponential function 😅. A* probably wasn't neccessary, since pruning of "bad" states is sufficient enough to search the whole space.

The one thing in part 2 I was sceptical about is if I can just rerun A* three times - turns out I can! But what guarantees, that the best solution in the first part is the best solution for part 2 (two subsequent runs of A*)? Not sure about that... 🤔

## Day 25

That one was simple - as it should be on Christmas day 😄. But for part 2 (if there even is a part 2?) I need to get the three missing stars:

* Day 20 - part 1 
* Day 20 - part 2
* Day 17 - part 2

I hope I do it till New year 😎.

## Updates

### Solved day 20 (25. 12. 2022)

Finally solved day 20. Everything was correct, except for a small bug when calculating the final result. The catch was, that it **worked correctly on test input**, so that's why I couldn't figure it out 😵‍💫! Thankfully I solved it finally. This [reddit comment](https://www.reddit.com/r/adventofcode/comments/zqezkn/2022_day_20_solutions/j17piu9/) helped me to find the problem. As per the comment, I also used `i64::rem_euclid()` method instead of `%`. I did everything by myself, just compared the code to see where the difference was (there wasn't almost any, functionally). Part 2 was easy, had no problems there. Only Day 17 part 2 left 😎.

So, the problem was that I didn't use the current number index when searching for the final number positions (to calculate grove coordinates), but the original number index by mistake! 😅