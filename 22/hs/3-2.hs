import Data.Char (ord)
import Data.Set (elemAt, fromList, intersection)

tst = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"

group3 :: [a] -> [[a]]
group3 [] = []
group3 xs = take 3 xs : group3 (drop 3 xs)

getPriority :: Char -> Int
getPriority c
  | c `elem` ['a' .. 'z'] = ord c - 96
  | c `elem` ['A' .. 'Z'] = ord c - 38

countOutliers :: [String] -> Int
countOutliers xs@[x1, x2, x3] = getPriority $ elemAt 0 badge
  where
    possible_badge = intersection (fromList x1) (fromList x2)
    badge = intersection possible_badge (fromList x3)

main = interact (show . sum . map countOutliers . group3 . lines)

-- 2413%