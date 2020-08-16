# Quantized

The idea of this project is to create a dungeon crawler game heavily based and themed around probabilities. What do I mean by that? Take as an example your classic **long sword** weapon, you might have encountered a million times in various games. Its description might have read as follows:

```
Long sword
Lore: "This is a sword, which is long."
Attack: +20
Reach: 5
Durability: 50
```

A classic. In Quantized, we plan to use descriptions as follows:

```
Long sword
Lore: "This is a sword, which is sometimes long"
Attack:
- 53% chance for +15 attack with reach 6
- 45% chance for +30 attack with reach 3
- 2% chance to lose 5 HP and destroy your sword in the process
```

Let's have another classic example:

```
Potion of Minor Healing
Lore: "A flask filled with a bright red liquid. Traditionally used for healing."
Effect: Heal 5.
```

The corresponding Quantized way of doing things:

```
Potion of Minor Healing
Lore: "A flask filled with a bright red liquid. Usually provides healing."
Effect:
- 70% chance to Heal 5
- 20% chance to Heal 10
- 7% chance for an allergic reaction
- 3% chance for getting drunk

```

Everything in our game including not only weapons and items, but also skills, enemies and encounters will have their random distribution shown for the player to make the best decisions.

## Disclaimer

This game is developed in [Rust](https://doc.rust-lang.org/book/), using not really estabilished libraries ([Bevy](https://bevyengine.org/news/introducing-bevy/)). It is a learning experience, therefore it might no be perfect and it might take time to reach something sensible. Please, be patient.

## Design document

We have already described the core idea of the game. In this section we specify the game in more detail. This section will be udpated as we move forward, as some of the ideas might become outdated and other might be updated.

### Randomness

Where it makes sense, we want to use random distributions for actions. For attacks, defenses, encounters, item usage, enemy actions, everything should be a random distribution visible to the player. The game will be random, but at the same time predicatably so.

### Turn-based

The game should be naturally turn-based, to give players enough time to pick the correct random action.

### Encounter-to-encounter

We will not aim to create a world, which can be explored by the player. Our aim is to focus on randomness, which is complex by itself and therefore the main playtime will be spent in encounters. Between encounters, the player will only choose which encounter he wants to go to next.

### Dungeon run

The game can be cleared in an hour or two. However, most of time your character will die in the process and you will have to start from scratch. No saves, no second chances. A good understanding of randomness and its consequences should be required for a successful pass of the game.

### Combos

While the randomness provides a decent amount of complexity, it might not be the main source of fun. For a lot of games, the fun is provided by combos -- interactions between various items, skills or cards, depending on the game. We would like to offer this layer of fun also in our game. This will be a real challenge, as making a cool interaction between random distributions might be difficult.

## Interested?

Get in touch!
