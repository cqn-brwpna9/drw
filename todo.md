## first batch:
### Done
- forward
- turn
- dup
- swap
- pop
- add
- sub
- mul
- div
- mod
- rep loop

## second batch:
### Done
- the rest of the math commands
- the rest of the pen commands

## second and a half batch:
### Done
- movement to a homespun, faster turtle backend 

## third batch:
#### Done
- dip and undip
#### Done
- while loop
#### Done
- functions
#### Done
- boxes
#### Done
- error handling
> This means "any syntactcally valid code causes ~~no~~ *minimal* runtime errors because runtime errors *mostly* don't exist" a la [Decker](https://github.com/JohnEarnest/Decker)
> 
> Therefore we have:
> - [x] An id value for each builtin if the stack is empty or at the bottom from dipping(0 for +/-,1 for *,[0 0 0] for B)
> - [x] Lisp-Style lists from nested boxes
> - [x] Operation pervasiveness for boxes
>
> There are ~~2~~ *1* possible types of runtime errors:
> ~~- "cannot {operator} {arg1} and {arg2}" ~~
>
> ~~This happens when one tries to call a dyadic operator on two nested boxes because trying to add `[1 2 [3 [4 5 6] 7]]` to `[1 [2 3 4] 5]` doesnt really make sense.~~
>
> ~~operators that do not modify their arguments (like `:`) do not have this error~~
>
> Can actually do this. Will be: `+[a b [c d e]] [[f g [h i j]] k l]`=>`[[f+a g+a [h+a i+a j+a]] k+b [l+c l+d l+e]]`
>
> - "cannot iterate {arg1} times"
>
> This happens when one tries to use a box, negative number or not whole number as the argument to a `[]` loop
>
>
> `{}` loops count any value besides `0`, `[0,0,0]` or any box made of those two values as truthy 
#### Not done
- nil/null value
> A value that means nothing. It will be represented by `,`. is nil will be `;`. It comes from divison by zero, but also will be used for making lists and suchforth much easier. It will not replace id values.
#### Done
- comments
> Just #....
#### Done
- syntax highlighting
> Only for nano. Sorry-not-sorry
> Make your own if you want your own.
> I may make emacs at some point.
#### Not done
- imports
> Some kind of meta-syntax/interpreter arguments/semantic comments. drw doesnt (and wont [I guess you could store a string in the recursive boxes, but theres no printing faclity \<yet\>]) have strings so must be not an actual command
#### Not done
- isbox
> no errors and easily cooreced types(if you can even call them that) means it can be hard to figure out whats a box so this


# This is the point at which Drw will be considered 0.1.0 under [Semantic Versioning](https://semver.org)

## later(may or may not happen):
> more question marks means less likely that it happens
- alingual docs?
> I probably would be bad at this
- a raylib, actually implemented IDE turtle thing??
> This is my end goal for drw, allowing a nicer, more live-coding educational type of thing
- multiple backends?
> svg, png, the current one, animated, gcode maybe?
