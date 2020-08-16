# Spellmaker

In each round of the encounter, the player will craft a spell and casts in on a given target. The player has at its disposal multiple base spells and also their potential improvements. Let us see an example of a base spell.

```
Fireball
- 60% chance to deal 10 fire damage to the target
- 15% chance to deal 15 fire damage to the entity to the left of the target
- 15% chance to deal 15 fire damage to the entity to the right of the target
- 10% chance to deal 5 fire damage to self
```

The base spell is pretty clear in its effect. What about support spells?

```
Frostburn
- 80% chance to convert all fire damage of the base spell to double damage in frost
- 20% chance to convert all fire damage of the base spell to healing
```

Combining these two spells, we have up to 8 different outcomes available. We have

- 48% to deal 20 frost damage to the target,
- 12 % chance to deal 30 frost damage to the entity to the left of the target,
- 12 % chance to deal 30 frost damage to the entity to the right of the target,
- 12% chance to heal the target for 10,
- 8% chance to deal 10 frost damage to self,
- 3% chance to heal the entity to the left of the target for 15,
- 3% chance to heal the entity to the right of the target for 15,
- 2% chance to heal ourself for 5.

The player might have multiple improvements to the base spell, and we might allow to use more than one improvement at any given time. This will always cause higher volatility, for the cost of better mean effect.

## Brainstorming questions

- How would the enemy decide on his actions? Will he also have some spells available and decides to combined them in some fashion?
- How many spells should we allow to combine?
- What happens to the spells, which we have used? Should they have durability? Or do they need some time to "refresh"?
- Initiative?
