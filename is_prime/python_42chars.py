"""
 CodeGolf is_prime -> 42
 =======================
 A prime number test in Python 3.
 42(!) characters.
"""

__author__ = "Niels Meyering"
__license__ = "MIT"

#   |........................................|
f = lambda n:all((n%i!=0 for i in range(2,n)))

if __name__ == '__main__':
    for x in range(2, 20):
        print("{} -> {}".format(x, f(x)))
