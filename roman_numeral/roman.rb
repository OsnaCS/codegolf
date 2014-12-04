#!/usr/bin/ruby
# @author Rene Helmke <rhelmke@uos.de>
# Codegolf Arabic to Roman Numbers Conversion in Ruby with 159 characters.

#|--------------------------------------------------------------------------------------------------------------------------------------------------------------------| 166 Characters :3
 f=->(n){return''if n==0;[1000,900,500,400,100,90,50,40,10,9,5,4,1].zip(%w[M CM D CD C XC L XL X IX V IV I]).each do|v,l|o=l*(n/v);print o;return(o)+f[n%v]if v<=n end}

# Call
(1..3999).each { |i| print i.to_s<<"->"; f[i]; puts "" }

