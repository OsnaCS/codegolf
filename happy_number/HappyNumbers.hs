-- Eine unoptimierte Lösung in Haskell. 79(?) Zeichen.
-- Zum Ausführen ghc installieren und z.B in bash:
-- ghc -Wall HappyNumbers.hs && ./HappyNumbers
--

-- Hilfsfunktion, berechnet Summe der Quadrate der Ziffern
-- Typdeklaration (optional)
h :: Integral a => a -> a
--  ......................................................?
h = sum.map((^2).(`mod`10)).takeWhile(>0).iterate(`div`10)

-- Typdeklaration (optional)
happy :: Integral a => a -> Bool
--      .........................?
happy = (==1).until(`elem`[1,4])h

-- Ausgabe
main :: IO ()
main = do
	let xs = take 100 [x | x <- [1..], happy x] :: [Integer]
	print xs
