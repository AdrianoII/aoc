tst = "A Y\nB X\nC Z"

data RPS = Rock | Papper | Scissors deriving (Show, Eq)

charToRps :: Char -> RPS
charToRps c
  | c == 'A' || c == 'X' = Rock
  | c == 'B' || c == 'Y' = Papper
  | c == 'C' || c == 'Z' = Scissors

matchToRps :: String -> (RPS, RPS)
matchToRps (p1 : _ : p2 : _) = (charToRps p1, charToRps p2)

rpsToInt :: RPS -> Int
rpsToInt Rock = 1
rpsToInt Papper = 2
rpsToInt Scissors = 3

evalMatch :: (RPS, RPS) -> Int
evalMatch (p1, p2) = rpsToInt p2 + result p1 p2
  where
    result p1 p2
      | p2 == p1 = 3
      | p2 == Rock && p1 == Scissors = 6
      | p2 == Papper && p1 == Rock = 6
      | p2 == Scissors && p1 == Papper = 6
      | otherwise = 0

main = interact $ show . sum . map (evalMatch . matchToRps) . lines

-- 15422%