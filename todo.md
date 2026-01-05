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
#### Not done
- error handling
> This means "any syntactcally valid code causes no runtime errors because runtime error don't exist" a la [Decker](https://github.com/JohnEarnest/Decker)
> 
> Therefore we have:
> - An id value for each builtin if the stack is empty or at the bottom from dipping(0 for +/-,1 for *,[0 0 0] for  ).
> - Lisp-Style lists from nested boxes
> - Some form of operation pervasiveness from boxes (This will probably end up not being pervasive on nested boxes because that's hard. also trying to add `[1 2 [3 [4 5 6] 7]]` to `[1 [2 3 4] 5]` doesnt really make sense.)
#### Not done
- comments
> Just #....
#### Done
- syntax highlighting
> Only for nano. Sorry-not-sorry
> Make your own if you want your own.
#### Not done
- imports
> Some kind of meta-syntax/interpreter arguments/semantic comments. drw doesnt (and wont) have strings so must be not an actual command

# This is the point at which Drw will be considered 0.1.0 under [Semantic Versioning](https://semver.org)

## later(may or may not happen):
> more question marks means less likely that it happens
- alingual docs?
> I probably would be bad at this
- a raylib, actually implemented IDE turtle thing??
> This is my end goal for drw, allowing a nicer, more live-coding educational type of thing
