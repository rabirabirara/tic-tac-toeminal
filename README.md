# tic-tac-toeminal
A simple terminal Tic-Tac-Toe game, with the naive minimax implementation as an AI adversary.

## How to use

Just run the executable in the terminal (use `cargo run` or something). You can make a move by pressing 1-9 and then Enter.  You can also quit by pressing 0 instead.

You will be given a guide board to show you which key places where.  At present, you can only play against the indomitable AI, which means you will never see the win screen.
Since this was just a prototype to showcase the implementation of the minimax algorithm, that is fine with me.  I may implement PVP play next, though I don't see the need.


## Simple explanation of minimax

Minimax is a search algorithm.  Tic-tac-toe is a search problem, as well as a 2-player game.  Minimax is designed for 2-player zero-sum games where 
each player takes turns, just like Tic-Tac-Toe (and Chess, for example).

What the minimax algorithm actually does follows along these lines.  Given a board, minimax will evaluate it (and produce moves) as such:

1. Take the current board and find every possible next move.  If you are player X, then it will produce every possible X move.
2. Recursively call minimax to evaluate those boards too; this time, switch turns so that you evaluate every possible O move.
3. Eventually, you will reach a game over state.  This is either a win, loss, or a draw.  
    1. If a win for you, return a positive number; this is the evaluation of this game-over board.  
    2. If a loss, return a negative number.
    3. If a draw, return 0.
4. Recall steps 1 and 2.  Our recursive calls have completed and we know the evaluation, recursively, of each of the possible moves you can make.
    1. If this call is on the player's turn, try to pick the move with the highest evaluation; that is, pick the move that leads you to a win.
    2. If this call is on the enemy's turn, try to pick the move with the lowest evaluation; that is, pick the move that leads the player to a loss and thus the enemy to a win.
5. Repeat as needed until the entire tree of possible moves and boards has an evaluation.  If you want to make moves, then just choose moves with the best evaluation.
Break ties as you need.

## Writing AI for adversarial games, such as Chess

Tic-Tac-Toe is decided not just because minimax has solved it in its entirety, but because it's so tiny that humans already did that.  
You could probably write down all the possible states of Tic-Tac-Toe yourself, eventually.  Well, you probably won't, because most of them are garbage.
The tree of all possible boards in Tic-Tac-Toe is actually still quite large.  If there are 9 possible moves, then 8 possible moves, then 7, ... etc. then 1, then we have 9!
possible boards.  (That is an upper bound; there are of course some boards which use less than 9 moves and end early.)
That's 362,880 boards!  Okay, maybe you won't be able to write all the states down.

Chess is even larger.  When searching for our next move, we think of the game as a tree.  The current board is like the root, or the seed of the tree;
all the possible moves are its branches.  The number of possible moves to make is called the "branching factor".

The branching factor of Tic-Tac-Toe is at most 9, and it gets smaller as the game goes on.  But Chess?  At just the start of the game it is possible to make 18 moves:
you can move any of 8 pawns in two ways, and you can move your knights.  That's 8 + 8 + 2 = 18.  And as the game opens up, the number of possible moves rises even further.
Typically, we estimate Chess to have a branching factor of around 35 - this means we expect to evaluate around 35 moves on an average chess board.

35!  That's massive.  So too is 35! - to be clear, 35 factorial.  Even if we assume that the branching factor or number of valid/good moves goes down by 1 each move we make,
we still have to examine 35! total chess positions.  If you google 35!, you will find something like 1.03e40, which means 1.03 * 10^40.  Which is a TRULY, TRULY MASSIVE number.

Chess is not solvable with the minimax algorithm, and certainly not decidable in its entirety anytime soon.  We are limited by both time and space - that is, if a modern 
computer were to investigate every possible move from a given position, it would take too long (the heat death of the universe would come first), and it would
use too much memory (I am always running out of memory on my phone).  This is why slightly nondeterministic methods such as Monte Carlo Tree Search or Deep Learning are
instead at the forefront of Chess engine design.


## Other types of games

Logic puzzles are some kinds of popular games as well.  For example, Minesweeper and Sudoku are famous examples of logic puzzles.  
Minesweeper, unlike most logic puzzles of its kind, is slightly probabilistic; it is possible to reach a point in the game where you need to make a guess.
(It's really frustrating when that happens.)

Nonetheless, games that rely on logic to solve an already designed level/board are solvable via constraint programming instead of search.  Constraint programming 
implements a kind of search, but it's based around logical precepts.  For example, if you see a '3' on a Minesweeper board, that means of the 8 squares around that number,
3 of those are bombs.  That is a rule, of the logical sort - but it involves a bit of guessing, of course.

Sudoku is the same way.  For example, the constraints of Sudoku determine that no row/column/3x3-grid repeats a number.  If we assign each square in the board to a Boolean
variable, we can formulate Sudoku as a Boolean formula and solve the Sudoku instance using a SAT solver.
