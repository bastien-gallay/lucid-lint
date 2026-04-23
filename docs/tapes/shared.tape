# Shared VHS preset for lucid-lint terminal captures.
#
# Source this file at the top of every tape with `Source shared.tape`
# so every asset uses the same dimensions, theme, font, and padding.
# Individual tapes then set only `Output`, any per-tape tweaks, and
# the command sequence.

# Canvas ---------------------------------------------------------------
# 1000 × 600 matches the mdBook content column on a mid-range laptop
# and stays crisp when scaled down to 800 on narrower screens.
Set Width 1000
Set Height 600
Set Padding 20

# Typography -----------------------------------------------------------
# Commit Mono mirrors the mdBook monospace face so code blocks and
# terminal captures feel like one family. Falls back to the system
# monospace if the font is not installed.
Set FontFamily "Commit Mono, Menlo, monospace"
Set FontSize 14
Set LineHeight 1.4

# Theme ----------------------------------------------------------------
# Lucid-lint's brand dark: near-black background, lens blue accent,
# warm yellow for warnings, dim grey for info / metadata. Mirrors the
# mdBook Lucid dark palette so tapes drop in without a hue clash.
# VHS requires the theme JSON on a single line. Keep it ugly; it is
# the only way the parser accepts it.
Set Theme { "name": "lucid-dark", "background": "#10131a", "foreground": "#e6e8ee", "cursor": "#7aa2ff", "black": "#10131a", "red": "#ff6b6b", "green": "#69d58c", "yellow": "#f3c969", "blue": "#7aa2ff", "magenta": "#c58aff", "cyan": "#6fd3ff", "white": "#e6e8ee", "brightBlack": "#525866", "brightRed": "#ff8a8a", "brightGreen": "#8ee4a9", "brightYellow": "#ffd98a", "brightBlue": "#9fb8ff", "brightMagenta": "#d3a5ff", "brightCyan": "#9fe0ff", "brightWhite": "#ffffff" }

# Typing cadence -------------------------------------------------------
# TypingSpeed is per keystroke. 40ms feels deliberate without being
# slow; a trained reader can still follow what is being typed.
Set TypingSpeed 40ms
