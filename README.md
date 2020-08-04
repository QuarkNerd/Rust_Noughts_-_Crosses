Noughts and Crosses console app.

## What is this?
A machine learning system for learning Noughts and Crosses based on [MENACE](https://medium.com/@ODSC/how-300-matchboxes-learned-to-play-tic-tac-toe-using-menace-35e0e4c29fc).


## To Start
1. either download a release or the repo
2. install rust
3. run the command ```cargo run```

Upon running, you will be faced with a menu interface. To play against the computer to train a computer you must add one or more computer players or a computer learners.

## What are types of opponents?
computer learner - The basic type of player. Each move picked using a weighted random distribution. When a new computer learner is created, all moves have the same chance. If the the learning flag is turned on, the weights will update at the end of each game. You can also load or save these to a file. They will be saved in the "player_strategy" folder. 

computer player - can't learn. Can only be created from a save file (use same files as a computer learners). They will always choose what they think is the best move, rather than be random. You can track the games of these because there is information is displayed as "Computer Player - {a} - {b} - {c}". Where {a}, {b}, {c} are the number of wins, draws and losses. You cannot save a computer player

## Training regimes
To create a training regime, create a file in the "training_regime" folder. In each line add two player names (in the order that they will play), separated by a single space. See example file for guidance