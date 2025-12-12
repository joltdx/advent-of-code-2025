# advent-of-code-2025
[Advent of Code 2025](https://adventofcode.com/2025) in [Rust](https://rust-lang.org/)

This year I'm trying to actually comment my code to remember what I did, and mostly why I did some of the thing in the way that I actually did some of the things...

I'm not really a rustacean, but I am really getting to like this language :)

Obviously asking some AI for help and review, but I'm actually thinking and coding for myself...

## Day 1 - Secret Entrance (The one with the dial on the safe)
>part 1 @ 59.942µs
<br>part 2 @ 56.68µs

Some different variations, but nothing fancy


## Day 2 - Gift Shop (The one with the invalid product IDs)
>part 1 @ 88.874µs
<br>part 2 @ 595.101µs

Low performance string stuff in part 1, a bit more mathy part 2

## Day 3 - Lobby (The one with the battery bank **jolt**ages)
>part 1 @ 800.545µs
<br>part 2 @ 380.642µs

Super slow solution for part 1 with strings and characters and parsing to switch types.
Way more performant byte stuff in part 2 

## Day 4 - Printing Departement (The one with the rolls of paper)
>part 1 @ 1.329051ms
<br>part 2 @ 6.715637ms

Surprisingly long runtime, but just very basic byte reading and counting in part 1.
Part 2 includes a roll coordinate queue for a bit more efficiency with rechecking

## Day 5 - Cafeteria (The one with the ingredient ranges)
>part 1 @ 8.595µs
<br>part 2 @ 599ns

Wow! This one was fun to implement... Sorting, merging and then a straight forward
loop to check for fresh or not. And part 2 in basically a one-liner :) 

## Day 6 - Trash Compactor (The one with the right-to-left vertical math)
>part 1 @ 791.17µs
<br>part 2 @ 239.57µs

Nothing fancy. Reading data, parsing and vectorizing in part 1, which is slower than
the part 2 with less allocation and parsing stuff and more just data access and arithmetics...

## Day 7 - Laboratories (The one with the beam splitter and quantum timelines)
>part 1 @ 171.734µs
<br>part 2 @ 170.385µs

Just looping and counting for part 1 and recursion and memoization in part 2

## Day 8 - Playground (The one with the junction boxes and the connected circuits)
>part 1 @ 109.814µs
<br>part 2 @ 5.146312ms

A bit more Rusty now with structs and impls. Calculating distances and then connecting as required.
First solution keeping track in a `Vec<Vec<usize>>`, was quite slow, so on suggestion from an AI,
refactored into some kind of a "Disjoint Set Union" or "Union-Find", which ended up being about
15-20 times faster

## Day 9 - Movie Theater (The one with the floor tiles and largest rectangle)
>part 1 @ 945.452µs
<br>part 2 @ 34.257712ms

Wow, part 1 was just very easily checked for all combinations. For part 2 I first tried with some
ray-casting algorithm, but it was way too slow. The input is not a crazy mess of edges back and forth
and right next to eachother, so I ended up checking just the edges of the rectangles, and some point
in the middle to find possible rectangles. Will not hold for a general case solution, but it did
for this day's AoC

## Day 10 - Factory (The one with the machines with buttons and lights and joltages)
>part 1 @ 249.165µs
<br>part 2 @ 195.316406ms

For part 1, I am very happy with generating binary representations of things and pressing
buttons using XOR.

For part 2, haha, well, it ended up finally working and being performant through an iterative
process of me trying stuff, having Gemini as a side-kick to suggest things, refactor code,
optimizing whatnots, rinsing and repeating from step 1 until it was first of all giving a
correct answer and secondly until running fast enough. Learned a lot in the process, next
time I might consider finding a crate for it though... Crazy stuff

## Day 11 - Reactor (The one with the device outputs path counting)
>part 1 @ 10.431µs
<br>part 2 @ 253.785µs

Just simple DFS for part 1, with modification for part 2 to allow for avoiding a certain node
(to be able to multiply sub paths for the end result) and memoization instead of just keeping
track of visited nodes in a specific path.

## Day 12 - Christmas Tree Farm (The one with the presents in grid fitting problem)
>part 1 @ 256.368106ms
<br>There is no part 2

Wow, this was actually a pretty complex box fitting problem to get to run fast... Luckily
it seems we were played a bit of a trick with the input being very solvable. (Spoiler: it
would just have been enought to just see if the grid was big enough to fit all the boxes,
not actually trying to optimize their placement). Didn't notice that until after I had
actually done such a solution, using bit operations for performance. Ah well, it's for
fun and games and it was indeed fun and I gained some developer XP as always