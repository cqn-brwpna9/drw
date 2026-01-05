# Drw 

Drw is a stack based esolang for drawing turtle graphics

## Note:

A feature will have :( by it if it is unimplemented

## Commands   

### Drawing commands

|Name|Symbol|Description|
|---|---|---|
|Forward|`^`|Pops one value from stack and goes forward that many steps.|
|Turn|`~`|Pops one value from stack turns counterclockwise that many degrees(or radians).|
|Degree mode|`o`|Switches to degrees for turns, if in degree mode, pushes the number of degrees in a full circle(360). The turtle defaults to degree mode.|
|Radian mode|`r`|Switches to radians for turns, if in radian mode, pushes the number of radians in a full circle(τ).|
|Color|`c`|Pops 3 values in [0,256) from the stack and sets them to the turtles color in RGB. The turtle defaults to white pen color.|
|Pen down|`d`|Puts the turtles pen down. It is down by default.|
|Pen up|`u`|Pulls the turtles pen up.|
|Pen size|`s`|Pops one value off the stack and sets the turtles pen size to that. Size is 1 by default|

### Stack manipulation commands

|Name|Symbol|Description|
|---|---|---|
|Duplicate|`.`|Duplicates the top stack value.|
|Swap|`:`|Swaps the two top stack values.|
|Dip and undip|`(` and `)`| `(` temporarily pops the top stack value to an inaccessible stack that can be thought of as "above" the main stack. `)` pushes one value from the upper stack.|
|Pop|`p`|Discards the top stack value.|
|Debug print|`?`|Print out the whole stack without popping it.|

### Box commands

|Name|Symbol|Description|
|---|---|---|
|Box|`B`|Takes three numbers, although it will soon also be able to take boxes as well, off the stack and combines them into a "box" which can be manipulated just like any value on the stack. Boxes do not support any other operations except coloring currently, although this will change later, which can either take three integers in [0, 256) or one box with all contained values in [0, 256).|
|Unbox|`U`|Puts the three values in a box back on the stack.|
|Is box:(|`I`|Pushes 1 if the value on top of the stack is a box, 0 otherwise. Does not consume its argument.|
 
Without boxes duplicating a color would be not easy. With boxes it is `B.` and `(U)U` to unbox the colors.


### Math commands
#### Note: All commands for which order is important take their arguments off the stack like this:
|Stack:|
|---|
|top|
|A|
|B|
|bottom|

Code: `%`

Result: `a%b`

|Name|Symbol|Description|
|---|---|---|
|Add|`+`|Adds the top two numbers on the stack.|
|Subtract|`-`|Subtracts the top two numbers on stack|
|Multiply|`*`|Multiplies the top two numbers on the stack.|
|Divide|`/`|Divides the top two numbers on the stack.|
|Modulo|`%`|Modules the top two numbers on the stack.|
|Power|`P`|Raises the top value on the stack to the value below it.|
|Log|`l`|Takes the logarithm of the top stack value of base the value below it.|
|E|`e`|Pushes e.| 
|Square root|`q`|Takes the square root of the top of the stack.|
|Sine|`S`|Takes the sine of the top of the stack.|
|Ceiling|`C`|Rounds to the number above the current number.|
|Floor|`f`|Rounds to the number below the current number.|
|Round|`R`|Rounds to the nearest integer.|
|Less than|`<`|Pops two numbers from the stack, pushing 1 if the first is less than the second, pushing 0 otherwise|
|Greater than|`>`|Pops two numbers from the stack, pushing 1 if the first is more than the second, pushing 0 otherwise|
|Equal|`=`|Pops two numbers from the stack, pushing 1 if the first is equal to second, pushing 0 otherwise|

### Control flow commands

|Name|Symbol|Description|
|---|---|---|
|Repeat loop|`[...]`|Pops one number off the stack, and executes the code within that many times.|
|While loop|`{...}`|Executes the code within `{...}` until `0` is on top of the stack when the code within `{...}` is finished running. Also pops off at the opening `{` and checks for zero. Can be used as an if statement by `<Conditonal>{<whatever code>0}`.|
|Function binding |`_`|Defines a function, which can be called from anywhere and take and return any number of arguments. The number of arguments can even be differing depending on the function's other arguments. Looks like `𝕗_<function body>`. Drw will just run the first non-function line in a program, then any other non-function lines after that. A function line starts with a name and `_`. A function is called by its name. A name is one Unicode codepoint (not grapheme) that is not a builtin. Builtins are `^~.:p+-*/%[]{}()orcdus?PleqSCfR><=BU_`|

### Drw's EBNF Syntax:

```
<program> <= {<line>};
<line> <= <function_definiton>|<code_line>;
<function_definiton> <= <function_name>, "_", <code_line>;
<function_name> <= ?any unicode codepoint? - <builtin>;
<code_line> <= {<builtin>|<number>|<function_name>|<loop>};
<loop> <= "{", <code_line>, "}"| "[", <code_line>, "]";
<number> <= {<digit>};
<digit> <= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
<builtin> <= "^"| "~"| "."| ":"| "p"| "+"| "-"| "*"| "/"| "%"| " "| "0"| "1"| "2"| "3"| "4"| "5"| "6"| "7"|"8"| "9"| "["| "]"| "{"| "}"| "("| ")"| "o"| "r"| "c"| "d"| "u"| "s"| "?"| "P"| "l"| "e"| "q"|"S"| "C"| "f"| "R"| ">"| "<"| "="| "B"| "U";
```
