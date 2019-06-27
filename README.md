Sudoku solver written in Rust
=============================

Run
---

````
$ time cargo run
   Compiling sudoku v0.0.1 (/Users/daniel/Projects/sudoku)
    Finished dev [unoptimized + debuginfo] target(s) in 0.65s
     Running `target/debug/sudoku`
Input:
-------------------
|  3  |     |     |
|     |1 9 5|     |
|    8|     |  6  |
-------------------
|8    |  6  |     |
|4    |8    |    1|
|     |  2  |     |
-------------------
|  6  |     |2 8  |
|     |4 1 9|    5|
|     |     |  7  |
-------------------
Solution:
-------------------
|5 3 4|6 7 8|9 1 2|
|6 7 2|1 9 5|3 4 8|
|1 9 8|3 4 2|5 6 7|
-------------------
|8 5 9|7 6 1|4 2 3|
|4 2 6|8 5 3|7 9 1|
|7 1 3|9 2 4|8 5 6|
-------------------
|9 6 1|5 3 7|2 8 4|
|2 8 7|4 1 9|6 3 5|
|3 4 5|2 8 6|1 7 9|
-------------------
cargo run  57,62s user 0,53s system 100% cpu 58,089 total
````

TODO
----

* Since converting the return type of `fn solve_sudoku` to an `Option` value, the solver runs about 340 seconds instead of 60 seconds for `_playfield_wikipedia_de`. Investigate why.
* Better understand array values and call-by-value / call-by-reference semantics in Rust

Concepts
--------

* Sudoku
* depth-first search (DFS)
* brute force solver (no heuristics)

Links
-----

* [Wikipedia: Sudoku](https://en.wikipedia.org/wiki/Sudoku)
* [Summer Sudoku Easy](http://www.summersudoku.com/sudokuEasy.php)
* [Option Values](https://doc.rust-lang.org/rust-by-example/std/option.html)
* [Rust for C++ Programmers: Arrays and Vectors](https://github.com/aminb/rust-for-c/tree/master/arrays)
* [The Rust Programming Language: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)