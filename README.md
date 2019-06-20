# Rust Arithmetic Expression Evaluator

This is a simple program that takes as input an arithmetic
expression and outputs its evaluation

## How to use

You must provide an arithmetic expression to the program. 

### Operators
Here are the operations handled:

- PLUS (+)
- MINUS (-)
- MULTIPLY (x)
- DIVIDE (/)
- POWER (^)

All numbers and operators must be separated by a space,
except for unary operators. Here are the unary operators handled:

- MINUS (-)

You can only use one unary operator for one number (e.g `--2` is invalid input)

### Variables
TODO

## Examples
```
$ ./main 1 + 3 x 2
7
```

```
$ ./main 1 + 3 x -2
-5
```

