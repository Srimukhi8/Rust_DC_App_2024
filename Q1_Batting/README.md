# Batting

Nandha is bored of IPL, and wants to create a new version of batting, which increases the stakes. He wants you to build a program which will correctly score this version of batting, and will print out the scorecard correctly at the end.

## Rules of batting
 - An over is 6 balls played.
 - The possible runs in each ball is {0, 1, 2, 3, 4, 5, 6}.
 - A boundary is any ball in which 4 or more runs are scored.
 - The final score is the sum of all the runs scored.
 - 10 overs are played (usually, look for the twist!)

## Twists in batting
 - If no boundaries are scored in an over, everything proceeds as normal.
 - The next three cases of overs have special bonus powers!
 - If all balls in an over are boundaries, the runs scored in the next two overs are doubled.
 - If the first two balls of an over are sixes, the runs scored in the rest of the over are doubled (only if they are all not boundaries!).
 - If the last two balls of an over are sixes, the runs scored in the next over are doubled (only if the first four were not boundaries!).
 - If a maiden over is played (no runs scored in the entire over), the next over cannot have any bonus power applied.
 - If a bonus power requires extra overs to be played, they will be played. For example, if all balls in the 10th over are boundaries, two extra overs will be played and the runs in those overs are doubled.

## Scorecard
 - The scorecard should be printed out at the end of the game.
 - Each over is represented like so
   (1, 2, 3, 4, 5, 6)
    - where the numbers are the runs scored in each ball.
    - If a boundary is scored, the number is prefixed with a '*'.
    - If a bonus effect is applied, that run has a x2 next to it: (1x2, 2x2, 3x2, 4x2, 5x2, 6x2).
 - If extra overs are played, they should be printed out as well.
  
## Final score
 - You must have a method to calculate the final score as well.

## Sample usage
Normal over:
 ```
bat(5) 
bat(4) 
bat(2) 
bat(1) 
bat(4) 
bat(3) 
 ```

 Special over:
 ```
bat(5) 
bat(4) 
bat(6) 
bat(4) 
bat(4) 
bat(6) 
 ```
 will cause the next two overs to be doubled.
