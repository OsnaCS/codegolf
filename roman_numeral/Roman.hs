-- Haskell, 177 Zeichen.
-- Zum Ausführen ghc installieren und z.B in bash:
-- ghc -Wall HappyNumbers.hs && ./HappyNumbers

-- Hilfsfunktion, 135 Zeichen
f :: Int -> (Int, String)
f n =head$dropWhile((>n).fst)(zip[1000,900,500,400,100,90,50,40,10,9,5,4,1]["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"])

-- Hilfsfunktion, 16 Zeichen
g :: Int -> Int
g n =n-(fst$f n)

-- die eigentliche Funktion, 26 Zeichen
r :: Int -> String
r 0=""
r n=(snd.f)n++(r.g)n

-- Beispiel-Ausgabe
main :: IO ()
main = do
	let xs = [1,2,3,4,5,6,14,15,19,42,1337,3999]
	let ys = map r xs
	let prettyPrint (a, b) = putStrLn $ concat [(show a), " -> ", b]
	mapM_ prettyPrint (zip xs ys)
