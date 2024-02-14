# Illusions
This was just used to play around with some illusion type projects.

## Lines
This illusion plays with change recognition within the human brain. After enough "static" is produced, one can recognize the lines being changed, but almost instantly afterwards, the brain loses track of the changes which occurred. When drawing quickly, your brain will recognize a line being drawn, but if you pause it you cannot pick out the line anymore.

In the default configuration, a line is drawn between random points. Each point in the line is changed in color by toggling between two colors (think xor). There is also the ability to use random colors instead of two static colors.

### Controls:
- Space: Pause/Unpause
- NumpadAdd : Speed up line drawing
- NumpadSubtract: Slow down line drawing
- R: toggle between two colors and random colors
- Escape: Exit the program

The two "static" colors may be configured in `src/lines/resources/line_state` in the `LineState::default()` implementation