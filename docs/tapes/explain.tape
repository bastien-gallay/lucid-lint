# explain — rule reference walkthrough.
#
# Runs `lucid-lint explain structure.sentence-too-long` so the viewer
# sees the bundled rule documentation printed to the terminal: the
# "What it flags" paragraph, the "At a glance" table, the thresholds.
#
# The explain output is ~55 lines, taller than the default 600 px
# canvas. We bump the height to 900 so the interesting first third
# fits without scrolling.
#
# Render: vhs docs/tapes/explain.tape
# Output: docs/src/assets/tty/explain.gif

Source docs/tapes/shared.tape

# Override height — explain output is taller than the default canvas.
Set Height 900

Output docs/src/assets/tty/explain.gif

Hide
Type "clear"
Enter
Sleep 200ms
Show

Sleep 800ms

Type "lucid-lint explain structure.sentence-too-long"
Sleep 600ms
Enter

# Give the reader time to scan the "What it flags" paragraph and the
# at-a-glance table before the loop restarts.
Sleep 6s
