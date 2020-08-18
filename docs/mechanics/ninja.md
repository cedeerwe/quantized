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

## Example round of fight

Current battlefield:

```
.-.-.-.
| |E| |
.-.-.-.
| | | |
.-.-.-.
| |P| |
.-.-.-.
```

The player has the following distributions to choose from:

- Fireball:
  - 70%: Initiative 35, Fire attack 10 range 3
  - 30%: Initiative 70, Fire attack 15 melee, 3 fire damage self
- Blink Strike:
  - 25%: Initiative 15, Teleport to the left of the enemy, deal 5 damage melee
  - 25%: Initiative 15, Teleport to the right of the enemy, deal 5 damage melee
  - 25%: Initiative 15, Teleport to the top of the enemy, deal 5 damage melee
  - 25%: Initiative 15, Teleport to the bottom of the enemy, deal 5 damage melee
- Paralyzing Gaze:
  - 80% Initiative 40, Immobilize all targets in range 2,
  - 20% Initiative 80, Stun a single target in range 2

The enemy has declared the following action distribution:

- 75%: Initiative 50, Move 3 attack 10
- 25%: Initiative 10, Shield 5 self for this round

The player picked the fireball skill, as it has good outcomes in all scenarios.

- 70% x 75% = 52.5% - Player starts with initiative 35 and deals 10 fire damage range 3.
- 70% x 25% = 17.5% - Enemy starts with initiative 10 and shields himself for 5. Player then attacks ranged for 10, dealing 5 damage.
- 30% x 75% = 22.5% - The enemy starts with initiative 50, moves to the player and strikes for 10. At initiative 70, the player has the enemy at close range and he can attack melee for 15, hurting himself for 5 in the process
- 30% x 25% = 7.5% - The enemy starts with shield 5 at initiative 10 and the player cannot attack melee, as there is no enemy nearby. He still suffers the 5 damage though.

### Additional Ideas

- The player might have some items, e.g. magma shield, which prevents the first 5 fire damage of each action. This would prevent the 5 damage dealt to himself by the fireball.
- Some of the skills might refer to abillities, instead of exact actions. These abilities might be levelled and upgraded by the player in the course of the runs.
- Some of the skills might have a refresh rate, or refresh probability. E.g., the fireball might have a refresh probability of 50%, which means that on average it should be refreshed after 2 rounds, but at the same time it might take ages. The player should make sure, that he uses this skill only if it really makes sense.
- The fight does not feel like a ninja anymore :/.
