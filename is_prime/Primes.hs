-- Eine Lösung in Haskell.
-- Zum Ausführen ghc installieren und z.B in bash:
-- ghc -Wall Primes.hs && ./Primes

-- Typdeklaration (optional)
prime :: (Integral a) => a -> Bool
-- 29   .............................
prime = \n->all((>0).(mod n))[2..n-1]

-- Ausgabe
main :: IO ()
main = do
	let somePrimes = take 100 [x | x <- [2..], prime x] :: [Integer]
	print somePrimes
