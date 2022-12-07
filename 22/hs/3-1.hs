import Data.Char (ord)
import Data.List ()
import Data.Set (fromList, intersection, toList)

tst = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"

getPriority :: Char -> Int
getPriority c
  | c `elem` ['a' .. 'z'] = ord c - 96
  | c `elem` ['A' .. 'Z'] = ord c - 38

countOutliers :: String -> Int
countOutliers xs = (sum . map getPriority) (toList outliers)
  where
    splitI = length xs `div` 2
    (group1, group2) = splitAt splitI xs
    outliers = intersection (fromList group1) (fromList group2)

main = interact (show . sum . map countOutliers . lines)

-- 8394