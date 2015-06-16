# `random_postorder`

Aufgabe ist es, einen Programm zu schreiben, was eine Postorder Traversierung eines zufälligen Suchbaums mit `n` Knoten ausgibt. Hierbei haben die Knoten die Werte `1` bis `n` Insgesamt soll, wie auch schon in den vorherigen Aufgaben ein "Funktionsobjekt" erstellt werden, was eine Zahl n als Argument nimmt. Diese Funktion soll dann die Postorder Traversierung in dieser Form ausgeben:

    1 3 2 6 5 4

Hierbei kann man sich die Bedeutung von "ausgeben" aussuchen: Entweder soll die Funktion einen String zurückgeben (dieser muss die Leerzeichen beinhalten) oder den String direkt auf stdout ausgeben. Es darf benutzt werden, was kürzer ist.
Die Traversierung oben entspricht diesem Suchbaum:

         4
        / \
       2   5
      / \   \
     1   3   6


Es soll ein zufälliger Baum erzeugt werden, wobei nicht jeder Baum gleich wahrscheinlich sein muss. Aber: Jeder mögliche Baum soll irgendwann mal erzeugt werden. Es gibt C_n viele unterschiedliche Suchbäume, die alle irgendwann erzeugt werden sollen (C ist hier die [Catalan Number](http://en.wikipedia.org/wiki/Catalan_number)).

Es dürfen Standardbibliotheken benutzt werden und auch Namen gekürzt werden. Diese `include`, `import`, `use` oder ähnliche Statements werden nicht mitgezählt.



## Bestenliste

* 84 -> Rene in Ruby
* 92 -> Lukas in Rust
