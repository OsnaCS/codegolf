-- Haskell, 158 characters
-- runhaskell HappyNumbers.hs

-- 158 char solution
f :: Int -> (Int, String)
{-
|..................................................................................................................|
-}
f n =head$dropWhile((>n).fst)(zip[1000,900,500,400,100,90,50,40,10,9,5,4,1](words"M CM D CD C XC L XL X IX V IV I"))

g :: Int -> Int
{-
|..............|
-}
g n =n-(fst$f n)

r :: Int -> String
{-
|....|
|..................|
-}
r 0=""
r n=(snd.f)n++(r.g)n


{- 173 char solution
|...........................................................................................................................................................................|
r n=snd$foldl(\(k,r)(i,s)->let(d,m)=divMod k i in(m,r++(concat$replicate d s)))(n,"")(zip [1000,900,500,400,100,90,50,40,10,9,5,4,1](words"M CM D CD C XC L XL X IX V IV I"))
-}

main :: IO ()
main = do
  let xs = [1,2,3,4,5,6,14,15,19,42,1337,3999]
  let ys = map r xs
  let prettyPrint (a, b) = putStrLn $ concat [(show a), " -> ", b]
  mapM_ prettyPrint (zip xs ys)
