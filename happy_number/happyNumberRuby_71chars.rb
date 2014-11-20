#!/usr/bin/ruby

# @author Rene Helmke <rhelmke@uos.de>
# Codegolf Happy Numbers in Ruby with 71 characters.

#   |---------------------------------------------------------------------|
f = ->(n){while n>1&&n!=4;a,n=n,0;while a>0;n+=(a%10)**2;a/=10;end;end;n<2}

####I think this version is prettier..unfortunately the rules dont allow it...
#f = ->(n){while n>1&&n!=4;n=n.to_s.chars.reduce(0){|s,i|s+i.to_i**2}end;n<2}


#output
(1..100).each {|i| puts "%d -> %s" %[i, f.call(i).to_s]}
