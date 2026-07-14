# Quarto Python

#### What does 16 represent?

For pieces, 16 denotes the empty piece which is assigned to the current piece field when there isn't a current piece selected.
In the case of board positions, 16 denotes an empty cell. The reason for using 0 to 15 to represent the pieces is to get the 4-bit piece representation.

#### Encoding

A board state encoder and decoder are provided with the simulator. The encoding is a 34 character long string compromising 17 numbers. 
Single digit numbers are encoded as "0n" and double digit numbers are written normally. The first 16 numbers are the board encoded row-wise linear indexing.
The last number is the current piece to place at that state of the board.

#### Playing a game

To play a game, you need to pick two agents that implement the superclass of a generic Quarto agent and then call the play method. A simple example can be seen below:
```py
from quarto import *
import quarto_agents as qagents

game = QuartoGame(qagents.HumanPlayer(), qagents.RandomQuartoAgent(), gui_mode=True, bin_mode=True)
game.play(randomizeFirstMove=False)
```

You can set randomizeFirstMove to False if you want your agent to play the first move.By default, this is set to false and the simulator will play the first move (hence the first player's move) randomly, which involves randomly selecting a piece for Player 2. For experimental purposes, making this move provided no advantage/disadvantage so it was just randomized.

- **gui_mode** set to True shows a visual board after every move played.
- **bin_mode** set to True will show the pieces in binary form. Only works if gui_mode=True.

#### Creating your own agent

It is very easy to create your own agent by means of the API provided. Just inherit from the GenericQuartoAgent and override the two methods provided.
The game information received in those two methods is called quartoGameState. This is the structure of that data:
```
[
    game state encoding (string),
    available pieces (set of integers),
    available positions (set of integers)
]
```
You can reference the game state data structure like an array to get the appropriate data. Check with the quarto class getGameState() method to make sure you are receiving the correct information.

After creating your agent and overriding those methods, just put your agent as an argument in the initialization of a quarto game.
There are already a number of existing agents in the [quarto_agents](quarto_agents) directory. These include a human agent, random agent, negamax agent and genetic minimax agent.


