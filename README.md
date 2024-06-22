# Miden Assembly Code Formatter

This is a very basic Miden Assembly code formatter. 

#### Basic Rules this formatter follows:
1) It indents using 4 spaces when it encounters: `begin`, `if`, `else`, `proc`, `export`, `repeat`, `while`, `end`.
2) It removes trailing spaces
3) It deletes lines with more than 2 empty spaces

That's pretty much it. 

Would be cool to auto format comments, and maybe some other things. 

#### Testing
```
cargo install --path .
```

```
cargo masm-fmt "src/asm/example3.masm"
```
