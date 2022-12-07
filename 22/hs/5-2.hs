import Data.Char (isNumber)
import Data.List (transpose)
import Data.Maybe (mapMaybe)
import Text.Read (readMaybe)

tst = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"

splitIn :: Eq a => a -> [a] -> ([a], [a])
splitIn delimiter xs = (p1, p2)
  where
    (p1, _ : p2) = span (/= delimiter) xs

parseStack :: [String] -> [String]
parseStack rows = map (filter (/= ' ')) cols
  where
    cols = transpose $ map (\e -> [e !! i | i <- [1, 5 .. length e]]) rows

parseMoves :: [String] -> [[Int]]
parseMoves moves_lines = [(mapMaybe readMaybe . words) ml | ml <- moves_lines]

applyMove :: Int -> Int -> Int -> [String] -> [String]
applyMove n f t stack = concat $ before_temp : [f_col] : [after_temp]
  where
    pf = pred f
    pt = pred t
    (_, full_f_col : _) = splitAt pf stack
    (target_group, f_col) = splitAt n full_f_col
    (before_t, t_col : after_t) = splitAt pt stack
    temp = concat $ before_t : [target_group ++ t_col] : [after_t]
    (before_temp, _ : after_temp) = splitAt pf temp

applyMoves :: [[Int]] -> [String] -> [String]
applyMoves [] s = s
applyMoves ([n, f, t] : xs) s = applyMoves xs (applyMove n f t s)

parseInput :: [String] -> String
parseInput input = map head $ applyMoves moves stack
  where
    (stack_lines, moves_lines) = splitIn "" input
    stack = parseStack $ init stack_lines
    moves = parseMoves moves_lines

main = interact (parseInput . lines)

-- BPCZJLFJW