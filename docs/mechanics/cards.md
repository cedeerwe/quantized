# Cards

The player has a deck of cards, where each card has a random distribution of effects written on it. On the beginning of the round, the player looks through his cards and chooses X cards, which he will use in this round. After picking these cards, their effects are picked acording to the random distribution and they are not random anymore. This way we manage to make the game slightly more predictable, as the actions themselves are not pure RNG. 

A standard situation is that I see 4 cards with a potential combo, but there is only a 40% chance that all of them will randomly pick the correct effect at the same time. On the other hand, the player can "hedge" against bad outcomes by picking cards, where all of the effects would make sense in the current round.

The opponents should provide probability distribution for their actions before the player picks their card from the deck. This way, there is going to be enough interaction between the two sides as well as some predictability and not only pure RNG.