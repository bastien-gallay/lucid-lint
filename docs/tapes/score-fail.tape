# score-fail — the gated path.
#
# Runs lucid-lint against the demo fixture with a tight --min-score
# gate so the run exits 1 and the `echo $?` afterwards shows the
# failure. Illustrates the CI use case: lint + score-gate + exit
# code, all in one frame.
#
# Render: vhs docs/tapes/score-fail.tape
# Output: docs/src/assets/tty/score-fail.gif

Source docs/tapes/shared.tape

Output docs/src/assets/tty/score-fail.gif

Hide
Type "clear"
Enter
Sleep 200ms
Show

Sleep 800ms

Type "lucid-lint check --min-score=85 examples/sample.md"
Sleep 600ms
Enter

# Wait long enough for the score block to settle before the next line.
Sleep 3s

Type "echo ""exit: $?"""
Sleep 400ms
Enter

Sleep 2s
