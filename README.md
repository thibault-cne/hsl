# HyperSpace Lang (HSL)

-----

:warning: This repository is *deprecated* :warning:

The HyperSpace compiler has been moved to [the main HyperSpace Lang repo](https://github.com/hs-lang/hsc).

-----

A Star Wars themed programming language with a compiler written in Rust.

## Introduction

Hello there ! You will find below an introduction to the HyperSpace Lang (HSL) and the work in progress to add new features.

Here is some quick technicals details : HSL is a strongly typed compiled language which support only 3 types for now : booleans, integers (negative and positive) and strings. Each keyword is a Star Wars quote or reference which makes every program quite funny to read. Finally newlines and spaces are not required but are encouraged for readabilty purposes (which is the aim of the language).

**Features**

For now HSL supports the following constructs :

- Variables
- Print to console
- Function definition (with parameters)

**TODO**

- Sementic controls
- Handle functions with variadic arguments
- Math operations (addition, substraction, multiplication, division and modulus)
- If then else blocks

**File extension**

Files written in the HyperSpace uses the `.hsl` file extension. You can find some examples in the [`examples`](./examples/) folder in order to write your own programs.

**Compilation**

For now HSL only compiles to _ARMv8_. The objective in the long term is to compile to multiple targets.

## Examples

You can find a bunch of examples in the `examples` folder. Here an example of a `Hello World!` program:


```hsl
Hypersignal printf Starfield 1                <(-.-)> Declare the external function printf with one string argument and variadcs 
  Holotext
Jamsignal                                     <(-.-)> End the declaration of extern function

A long time ago in a galaxy far, far away...  <(-.-)> Start of a function called galaxy (this is the name of the `main` function)
  Execute order printf                        <(-.-)> Call the printf function with only a string literal
    "Hello World!\n"
  Order executed
May the force be with you.                    <(-.-)> End of the galaxy (main) function

```
