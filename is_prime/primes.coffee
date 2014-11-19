# Eine Lösung in Coffeescript.
# Zum Ausführen erst npm, dann Coffeescript installieren mit
#   npm install coffee-script
# dann
#   coffee primes.coffee

# 29    .............................
prime = (n)->[2...n].every (i)->n%i>0

# Ausgabe
console.log [2..100].filter(prime)
