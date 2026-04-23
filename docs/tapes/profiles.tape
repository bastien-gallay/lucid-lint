# profiles — three profiles, one file.
#
# Runs lucid-lint against the demo fixture under `dev-doc`, `public`,
# and `falc` in sequence. Each pass clears the screen first so the
# viewer sees one scorecard at a time; the score should drop as the
# profile tightens.
#
# Needs a taller canvas than the hero — each scorecard is ~12 lines
# and we want them to feel spacious one after the other.
#
# Render: vhs docs/tapes/profiles.tape
# Output: docs/src/assets/tty/profiles.gif

Source docs/tapes/shared.tape

Output docs/src/assets/tty/profiles.gif

Hide
Type "clear"
Enter
Sleep 200ms
Show

Sleep 600ms

Type "lucid-lint check --profile=dev-doc examples/sample.md"
Sleep 400ms
Enter
Sleep 3s

Type "clear"
Enter
Sleep 400ms

Type "lucid-lint check --profile=public examples/sample.md"
Sleep 400ms
Enter
Sleep 3s

Type "clear"
Enter
Sleep 400ms

Type "lucid-lint check --profile=falc examples/sample.md"
Sleep 400ms
Enter
Sleep 3s
