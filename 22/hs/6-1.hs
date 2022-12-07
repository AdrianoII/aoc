import Data.List (nub)

tst = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n"

findMarker :: String -> Int
findMarker xs
  | isMarker xs = 4
  | otherwise = 1 + findMarker (tail xs)
  where
    isMarker = (== 4) . length . nub . take 4

main = interact (show . findMarker . head . lines)

-- 1987