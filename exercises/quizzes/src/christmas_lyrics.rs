# The Twelve Days of Christmas Challenge

Welcome to this Rust programming challenge! In this task, you'll build a Christmas carol lyrics generator that prints out the lyrics to "The Twelve Days of Christmas" while making the most of Rust's control structures and data handling capabilities to avoid unnecessary repetition.

## Challenge Overview

Your goal is to write a Rust program that:

1. Prints the full lyrics of The Twelve Days of Christmas carol.
2. Uses loops, arrays, or functions to avoid hardcoding repetitive lines.
3. Ensures the lyrics are correctly formatted.

## Lyrics Breakdown

In The Twelve Days of Christmas, each verse follows this pattern:
```
On the [ordinal] day of Christmas, my true love sent to me:
[A list of gifts corresponding to the current day, in reverse order]
```
The list of gifts builds up over the twelve days:
Day	    Gift
1st	    A partridge in a pear tree
2nd	    Two turtle doves, and
3rd	    Three French hens
4th	    Four calling birds
5th	    Five golden rings
6th	    Six geese a-laying
7th	    Seven swans a-swimming
8th	    Eight maids a-milking
9th	    Nine ladies dancing
10th	Ten lords a-leaping
11th	Eleven pipers piping
12th	Twelve drummers drumming

## Write the Lyrics Generator

- Use arrays or vectors to store ordinal numbers and gifts.
- Use loops to iterate over each day.
- Use string formatting to construct each verse dynamically.




fn main() {
    println!("Hello, world!");
}
