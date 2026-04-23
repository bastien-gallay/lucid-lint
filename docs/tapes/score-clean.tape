# score-clean — the happy path.
#
# Runs lucid-lint against a trivial fixture so the output hits the
# clean state: banner, "No issues found.", score 100/100, every
# sparkline full. This is the peak-end shot — the brand moment that
# rewards a passing CI.
#
# Render: vhs docs/tapes/score-clean.tape
# Output: docs/src/assets/tty/score-clean.gif

Source docs/tapes/shared.tape

Output docs/src/assets/tty/score-clean.gif

Hide
Type "clear"
Enter
Sleep 200ms
Show

Sleep 800ms

# --banner=always forces the wordmark even when stdout is a recorded
# TTY. Real interactive runs fire the banner automatically on clean
# results; we opt in here so the captured frame is deterministic.
Type "lucid-lint check --banner=always docs/tapes/fixtures/clean.md"
Sleep 600ms
Enter

Sleep 4s
