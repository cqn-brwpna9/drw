# Drw 

Drw is a stack based esolang for drawing turtle graphics

## Note:

A feature will have :( by it if it is unimplemented

## Commands   

### Drawing commands

|Name|Repl|Glyph|Description|
|---|---|---|---|
|Forward|`^`|`↑`|Pops one value from stack and goes forward that many steps.|
|Turn|`~`|`☇`|Pops one value from stack turns counterclockwise that many degrees(or radians).|
|Degree mode|`o`|`°`|Switches to degrees for turns, if in degree mode, pushes the number of degrees in a full circle(360). The turtle defualts to degree mode.|
|Radian mode|`r`|`㎭`|Switches to radians for turns, if in radian mode, pushes the number of radians in a full circle(τ).|
|Color|`c`|`⛯`|Pops 3 values in [0,256) from the stack and sets them to the turtles color in rgb. The turtle defaults to white pen color.|
|Pen down|`d`|`⥥`|Puts the turtles pen down. It is down by default.|
|Pen up|`u`|`⥣`|Pulls the turtles pen up.|
|Pen size|`s`|`◿`|Pops one value off the stack and sets the turtles pen size to that. Size is 1 by default|

### Stack manipulation commands

|Name|Repl|Glyph|Description|
|---|---|---|---|
|Duplicate|`.`|`.`|Duplicates the top stack value.|
|Swap|`:`|`:`|Swaps the two top stack values.|
|Dip and undip:(|`(` and `)`|`(` and `)`| `(` temporarliy pops the top stack value to an inacsessable stack that can be thought of as "above" the main stack. `)` pushes one value from the upper stack.|
|Pop|`p`|`◌`|Discards the top stack value.|
|Debug print:(|`?`|`?`|Print out the whole stack without popping it.|
|Box and Unbox:(|`B` and `U`|`〚` and `〛`|`〚` takes three numbers off the stack and combines them into a "box" which can be manipulated just like any value on the stack. Boxes do not support any other operations execpt coloring, which can either take three integers in [0, 256) or one box with all contained values in [0, 256). `〛` puts the three values in a box back on the stack. Boxes mostly exist to make color manipulation code less annoying|

Without boxes duplicating a color would be a horrible, unreadable string of duplicates, flips and dips. After boxes it is `〚.` With `(〛)〛` to unbox the colors.


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

|Name|Repl|Glyph|Description|
|---|---|---|---|
|Add|`+`|`+`|Adds the top two numbers on the stack.|
|Subtract|`-`|`-`|Subtracts the top two numbers on stack|
|Multiply|`*`|`×`|Multiplies the top two numbers on the stack.|
|Divide|`/`|`÷`|Divides the top two numbers on the stack.|
|Modulo|`%`|`%`|Modulos the top two numbers on the stack.|
|Power|`P`|`⁺`|Rases the top value on the stack to the value below it.|
|Log|`l`|`₊`|Takes the logarithim of the top stack value of base the value below it.|
|E|`e`|`e`|Pushes e.| 
|Square root|`q`|`√`|Takes the square root of the top of the stack.|
|Sine|`S`|`∿`|Takes the sine of the top of the stack.|
|Ceilng|`C`|`⌈`|Rounds to the number above the current number.|
|Floor|`f`|`⌊`|Rounds to the number below the current number.|
|Round|`R`|`⁅`|Rounds to the nearest integer.|
|Less than|`<`|`<`|Pops two numbers from the stack, pushing 1 if the first is less than the second, pushing 0 otherwise|
|Greater than|`>`|`>`|Pops two numbers from the stack, pushing 1 if the first is more than the second, pushing 0 otherwise|
|Equal|`=`|`=`|Pops two numbers from the stack, pushing 1 if the first is equal to second, pushing 0 otherwise|

### Control flow commands

|Name|Repl|Glyph|Description|
|---|---|---|---|
|Repeat loop|`[...]`|`[...]`|Pops one number off the stack, and executes the code within that many times|
|while loop:(|`{...}`|`{...}`|Executes the code within `{...}` until `0` is on top of the stack when the code within `{...}` is finished running.|
