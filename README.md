# Miden Assembly Code Formatter

Basic Miden Assembly code formatter.

The following keywords affect indentation rules in Miden Assembly: `begin`, `proc`, `export`, `if`, `else`, `repeat`, `while`, `end`.

#### Basic Rules this formatter follows:
1) Adds correct indentation following the code formatting rules seen in the miden-base repository.
2) Removes trailing spaces.
3) Deletes lines with more than 2 empty spaces.

That's pretty much it. 

Would be cool to auto format comments, and maybe some other things.

This code formatter has been tested extensively, however, there may be edge cases where it fails.

#### Testing

#### Installing binary from crates
````
cargo install masm-formatter
````

#### Installing binary from repo:
```
cargo install --path .
```

#### Formatting all files in a directory:
```
cargo masm-fmt "src/**/*.masm"
```

#### Formatting a single file in a directory:
```
cargo masm-fmt "src/asm/example3.masm"
```
