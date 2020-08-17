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

## Follow-up thoughts

At the beginning of the round, all of the enemies have their action distributions available to the player. So the player knows all the possibilities, what can happen, with their respective chances.

Each of the actions should also include an **initiative** number, which decides the ordering of the actions.

The player chooses his action distribution (somehow). Then, all these random distributions are going to be sampled and the rest of the turn proceeds in a non-random way. The initiatives decide the ordering of the actions and the actions themselves are also known to the player. Pure tactics follow.

The interesting question is, how should the player choose his action distribution? The output should look the same, as the action distribution for the enemies, which is

```
50%: Initiative 10, Move 2 + Attach 6 meelee
30%: Initiative 80, Attack 3, range 3
20%: Initiative 20, Shield 5, Heal 3 self
```

What should define the player, so that he can combine some stuff to arrive at such a distribution?
