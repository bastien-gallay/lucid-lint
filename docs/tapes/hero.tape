# README hero — 15s single-shot demo.
#
# Shows lucid-lint linting its own demo file: diagnostics cluster, the
# shared-message hoist, the score block with sparkline bars, and the
# explain hint with real rule ids. One take, no edits, no narration.
#
# Render:
#   vhs docs/tapes/hero.tape
# Outputs to docs/src/assets/tty/hero.gif (committed for README).

Source docs/tapes/shared.tape

Output docs/src/assets/tty/hero.gif

# Warm up — hide the prompt flash while the shell initialises.
Hide
Type "clear"
Enter
Sleep 200ms
Show

# One beat before typing so the viewer can find the cursor.
Sleep 800ms

Type "lucid-lint check examples/sample.md"
Sleep 600ms
Enter

# Let the output settle and give the reader time to parse the score.
Sleep 5s
