# Experimental!
# runs all tests
# expects to be run in /tests directory
Lib ← $"../lib/_.drwm"°□˜getmap{"./list_test.drw"} {"list"}
&fld"."
▽⊸≡◇⍣(⍩⋅1°$"_.drw")⋅0
≡◇(
  &p""◌&sc&pf⊸$"Press enter to run _:"
  &runi ⍣({"cargo" "r" ⟜("-m" Lib)}){"cargo" "r" ∘}
  &p$"Got exit code: _"
)
