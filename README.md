# Rust Arithmetic Expression Evaluator

This is a simple program that takes as input an arithmetic
expression and outputs its evaluation

## How to use

You must provide an arithmetic expression to the program. 

### Operators
Here are the operations handled:

- PLUS (+)
- MINUS (-)
- MULTIPLY (x) or (*) in standard input
- DIVIDE (/)
- POWER (^)

All numbers and operators must be separated by a space,
except for unary operators, in which case they must be attached to the number.
 
 Here are the unary operators handled:

- MINUS (-)

You can only use one unary operator for one number (e.g `--2` is invalid input)

### Variables
You can also define variables, and use them in the computation. For that, you need
to start the program without any arguments. Here is the syntax to define a variable:
```
variable=expression
```
where `variable` is the name of the variable and `expression` an arithmetic expression.
You can only define one variable by line. When defining new variables, you can use the
previous ones that defined before 

## Examples

`$` means that this is a terminal command

`>` means that this is the standard input

the output is the last line

```
$ ./main 1 + 3 x 2
7
```

```
$ ./main 1 + 3 x -2
-5
```

```
$ ./main
> a=3 + 2
> b= a + 1
> a + b
11
```

```
$ ./main
> a = 3 + 2
> b=a * 2
> a + b
15
```