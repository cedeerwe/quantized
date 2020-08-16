# Ninja

This version would include a small board, on which the encounter is actually happening. Imagine something like this:

```
.-.-.-.
| |E| |
.-.-.-.
| | | |
.-.-.-.
| |P| |
.-.-.-.
```
P stands for player an E stands for enemy. At the start of a round, we see that the enemy is taking the following action:
```
- 50%: move 1 + attack 5 meelee
- 50%: attack 7 range 2
```
This means, that if we make an action, which will be something like:
```
10% Kill an enemy
90% move 1
```
we are actually okay, because either we kill him, or we will simply go out of range.

One can imagine much more complex situations, but it might still make sense even with total randomness like this.