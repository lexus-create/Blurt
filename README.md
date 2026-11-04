# Blurt
A tiny, WIP programming language written in rust that contains a lexer, parser and interpreter. So far can only print.


let animal = "frog";
let legs = 4;

# say something
blurt "The {animal} has {legs} legs";


Prints: "The frog has 4 legs"

## Running it

Put your code in a file like hello.blurt, then:


cargo run -- hello.blurt


## How it works

- Every statement ends with ;
- blurt "text";` prints text
- let name = "text"; or let name = 5; stores a value
- {name} inside a blurt string fills in the variable
- # starts a comment that lasts until the end of the line

## What's missing

- Math
- Error messages with line numbers
- Quotes inside strings
- "if" and loops

## Code layout

lexer.rs splits text into tokens, parser.rs turns them into statements, and interpreter.rs runs them.
