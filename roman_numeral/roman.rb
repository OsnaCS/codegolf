#!/usr/bin/ruby
# @author Rene Helmke <rhelmke@uos.de>
# Codegolf Arabic to Roman Numbers Conversion in Ruby with 159 characters.

# |-------------------------------------------------------------------------------------------------------------------------------------------------------------| 159 Characters :3
f=->(n){return''if n==0;[1000,900,500,400,100,90,50,40,10,9,5,4,1].zip(%w[M CM D CD C XC L XL X IX V IV I]).each do|v,l|return(l*(n/v))+f[n%v]if v<=n end;f[n%v]}


# Output
(1..3999).each {|i| puts "%d -> %s" %[i, f[i].to_s]}
