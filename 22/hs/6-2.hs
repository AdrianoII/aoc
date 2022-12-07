import Data.List (nub)

tst = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n"

findMarker :: String -> Int
findMarker xs
  | isMarker xs = 14
  | otherwise = 1 + findMarker (tail xs)
  where
    isMarker = (== 14) . length . nub . take 14

main = interact (show . findMarker . head . lines)

-- 3059