#CodeGolf is_happy -> 36
#A anonymous function, which returns true, if n is a happy number. If not, then false.
#The function has just 36 characters.
#On Ubuntu f.x. you can run it by:
#1. install julia
#   sudo apt-get install julia
#2. execute file
#   julia thisfilename.jl
#Version from Joris Clement
#E-mail: JorisClement@posteo.eu

f(n)=2<n!=4?f(sum(digits(n).^2)):n<2

#Output
for j = 1:100
	println("$j: $(f(j))")
end
