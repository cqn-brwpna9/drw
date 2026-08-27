# Quoting

- uses `"...'` as brackets to place an ast on the stack
- `\`` executes a quote

## Representation

Rather than being represented as an opaque object on the stack, a quote is represented as a tree-like structure of boxes that can be manipulated using the `list` module
How representation works:
First, drw code is classified into three categories of token:
- Number(like `1`, `13297823498732498` or `7`)
- Command(like `+`, `-`, `(` or `p`)
- Control structure(like `{}`, `[]` or `"'` (quotes inside of quotes are allowed))
- User defined function call
Then, since control structures contain the things inside them, so the code is broken into a tree structure (Which is what drw normally uses as an ast).
In addition, user defined function definitions cannot be placed in quotes.
This is then stored in an ast node on the larger ast(the one the main program runs).
When a quote is reached at runtime, the ast is converted to a list (from the drw module) using the following rules:
- All ast nodes take up one list item, and that list item is a box.
- A number `n` is converted into a box that looks like this: `[0 n ,]`, then placed into a list
- A command is converted like this `[1 n ,]`, where `n` is the position of the command in the `ALLOWED_COMMANDS` constant in `src/ast.rs`
- A control structure looks like `[2 n m]` where `n` is the position of the structure in the `ALLOWED_BRACKETS` constant in `src/ast.rs`, and m is the stuff inside the control structure
- A user defined function call looks like `[3 u ,]` where `u` is the position of the function's name's codepoint in unicode, calculated using `char as u32`
Anything invalid in a quoted ast will result in an error.
Note that, while the order of the `ALLOWED_COMMANDS` and `ALLOWED_BRACKETS` lists can be expected to stay relativley stable, it may change, so it is not advised that its constants are hard-coded into programs. (but you do you)
As an example, the following quote (where `F_10^`):
```
"4[F90~]'
```
Is turned into these boxes:
```
〚〚0 4 ,〛〚〚2 0 〚〚3 70 ,〛 〚〚0 90 ,〛 〚〚1 1 ,〛 , ,〛 ,〛 ,〛〛 , ,〛 ,〛
```