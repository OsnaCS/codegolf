#!/usr/bin/ruby
# @author Rene Helmke <rhelmke@uos.de>
# Codegolf Arabic to Roman Numbers Conversion in Ruby with 123 characters.

#|-------------------------------------------------------------------------------------------------------------------------| 123 Characters >:D
 f=->(n){[1000,900,500,400,100,90,50,40,10,9,5,4,1].zip(%w[M CM D CD C XC L XL X IX V IV I]).each{|v,l|print l*(n/v);n=n%v}}


# Call
(1..3999).each { |i| print i.to_s<<"->"; f[i]; puts "" }
