tst = "A Y\nB X\nC Z"

loses :: Char -> Int
loses 'A' = 2
loses 'B' = 3
loses 'C' = 1

wins :: Char -> Int
wins 'A' = 3
wins 'B' = 1
wins 'C' = 2

draw :: Char -> Int
draw 'A' = 1
draw 'B' = 2
draw 'C' = 3

evalMatch :: String -> Int
evalMatch (p : _ : 'X' : _) = wins p + 0
evalMatch (p : _ : 'Y' : _) = draw p + 3
evalMatch (p : _ : 'Z' : _) = loses p + 6

main = interact (show . sum . map evalMatch . lines)

-- 15442%
