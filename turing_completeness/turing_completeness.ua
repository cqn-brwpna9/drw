# Any program from the turing complete subset of brainfuck(<>[]+-) can be translated into drw. note that this then makes a bf impl with an everexpanding tape and "limitless" (f64) cells
ConvMap ← ˜getmap "<>[]+-" {")" "(" ".{" ".}" "1+" "1:-"}
&pf "Input bf code:"
&sc
▽⊸∊"<>[]+-"
&p/◇⊂ ConvMap
