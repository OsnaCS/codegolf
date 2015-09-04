#CodeGolf is_prime -> 17
#A anonymous function, which returns true, if n is a prime. If not, then false.
#The function has just 17 characters.
#Version from Joris Clement
#E-mail: JorisClement@posteo.eu

#      |...............|
f(n) = sum(n%[2:n].<1)<2

#Output
for j = 2:100
	println("$j: $(f(j))")
end
