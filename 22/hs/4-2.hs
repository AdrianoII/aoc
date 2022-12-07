tst = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"

parseRange :: String -> [(Int, Int)]
parseRange xs = [(read lb1, read hb1), (read lb2, read hb2)]
  where
    (r1, _ : r2) = span (/= ',') xs
    (lb1, _ : hb1) = span (/= '-') r1
    (lb2, _ : hb2) = span (/= '-') r2

doOverlap :: [(Int, Int)] -> Bool
doOverlap [r1, r2] = doOverlap' r1 r2 || doOverlap' r2 r1
  where
    doOverlap' (lb1, hb1) (lb2, _) = lb1 <= lb2 && hb1 >= lb2

main = interact (show . length . filter doOverlap . map parseRange . lines)

-- 779