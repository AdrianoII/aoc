tst :: String
tst = "1\n20\n3\n\n10\n\n22"

split :: Eq a => a -> [a] -> [[a]]
split delimiter [] = []
split delimiter xs = takeWhile (/= delimiter) xs : split delimiter (if null possible_next_group then [] else tail possible_next_group)
  where
    possible_next_group = dropWhile (/= delimiter) xs

main = interact $ show . maximum . map (sum . map read) . split "" . split '\n'

-- 66616