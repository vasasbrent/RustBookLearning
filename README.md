# Rust Book Learning<!-- omit in toc -->
Workspace for example work and offshoot ideas while reading through the big book of Rust

- [1: Getting Started](#1-getting-started)
  - [1.2: Hello There](#12-hello-there)
  - [1.3: Hello Cargo](#13-hello-cargo)
- [2: Guessing Game](#2-guessing-game)
- [3: Common Programming Concepts](#3-common-programming-concepts)
  - [3.1: Variables and Mutability](#31-variables-and-mutability)

# Sections<!-- omit in toc -->

## 1: Getting Started
### 1.2: Hello There

* Special "main" function, drop hot here at execution, makes sense
  * Have not covered command line arguments/passing into main, soon to come hopefully

### 1.3: Hello Cargo

* Cargo just straight up removes all the difficulties from project setup (C perspective)
* ` Cargo check ` assesses whether a build is possible without generating an .exe
  * Dumb quick, use for sus code repeatedly
  * Edit: Use with CI to test updating packages automatically?

## 2: Guessing Game

Classic, Are you smarter than a CS161 student?

I am not.

* Match is super powerful
* "loop" is kind of odd 
  * expected "for" or some such
    * Looks like it's just for the early example
    * "for" is syntactially similar to the Python implementation
* eprintln! macro writes to io::stderr instead of io::stdout
  * useful for logs later? idk, we'll have to find out

Confused about "num" here:

![alt text](./readme_images/match_num.png)

Where does it come from?

It looks like num is a temporary name we give to the return variable.
The Ok arm of a result is allowed to contain more logic.
In this program I've expanded it to check if the input number (because we've now checked that it is a number) is within range.
If not, print a unique identifier and continue.
Pretty slick.

## 3: Common Programming Concepts

### 3.1: Variables and Mutability

