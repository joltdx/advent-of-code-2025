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