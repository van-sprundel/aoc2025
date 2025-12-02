# Advent of Code 2025

I have but one challenge...

I will be keeping everything branchless :)

Oh. And no AI ofcourse <3

## Erm.. this is not branchless, there are still `cmp`'s!

You're so right and such a good boy for noticing. Most of these `cmp`'s are either not branches (they're not followed by any action) or they're branches that are created by certain functions. I would argue these are pretty useless to optimize for. I care more that the business logic is branchless. If you really want to spend your night replacing e.g. `rem_euclid` with a branchless version then feel free!

I'm also not looking to replace `input.lines()` since this will just become hard to read. Maybe after AOC is over thought :) I'm sure it's do-able.
