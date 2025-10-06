# barnarok

Barnarok is a simple chess engine made entirely in Rust.

It can calculate the number of reachable board positions after a certain number of half-moves (plies), and given a certain initial FEN string.
For that, use: 

cargo run explore --depth {nb_of_plies} --fen {initial_fen_string}

"--fen" can be omitted to use the initial configuration of a chess board.
Add "--verbose" to print the detailed half-move tree (only works with a depth of 1 or 2).

The engine can also allow you to play a game of chess, and choose different strategies for each side.
Use:

cargo run play --wstrat {white_strategy} --bstrat {black_strategy}

Each strategy can be chosen from the following:
- "player": you will be asked to write the name of the selected move according to the Universal Chess Interface notation (for example 'b2b4').
- "random": the program will select a random legal move.
- "negamax": the program will use the negamax algorithm to find the best move (as far as it can see).
- "alphabeta": the program will use the alphabeta algorithm to find the best move more quickly.
- "alphabetaq": the program will use the alphabeta algorithm to find the best move more quickly, with quiescence to avoid stopping the search before an important move.

The "AI" is not very smart yet, but in the future it should be improved to win against humans.
For now, it always wins against the random strategy only.