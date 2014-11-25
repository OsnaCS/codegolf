#!/bin/python

# Happy number calculation in Python
# Author: Lars Kiesow <lkiesow@uos.de>
#
# echo -n 'def f(x,n=0):
#  while x:n+=(x%10)**2;x/=10
#  return n==1 or(n>4 and f(n))' | wc -c
#
# -> 71

def f(x,n=0):
	while x:n+=(x%10)**2;x/=10
	return n==1 or(n>4 and f(n))

# Print happy numbers in {n \in \N | n \leq 100}:
print [i+1 for i in range(100) if f(i+1)]

