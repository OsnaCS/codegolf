# `is_prime`: Primzahlentest

Aufgabe ist, einen Primzahlentest zu programmieren. Allgemein gesagt soll ein "Objekt" (meist Funktion) erstellt werden, dem man eine Zahl übergibt, wobei der Ausdruck dann zu true oder false evaluiert wird. `true`, wenn die die Zahl eine Primzahl ist, `false` sonst. Es muss nur für Zahlen `>= 2` funktioniere.

Beispiel Java-Code:

    bool isPrime(int n) {
      for(int i = 2; i < n; i++) {
        if(n%i == 0) {
          return false;
        }
      }
      return true;
    }

Diese Lösung ist 118 Zeichen lang. 

Whitespace-Zeichen werden natürlich mitgezählt. Es zählt NUR die Definition des "Objekts" (meist Funktion). Aufruf des Objekts oder Ausgabe zählen nicht. Im Falle, dass man ein Funktionen-Objekt erstellt und es einer Variable zuweist, wird die Zuweisung nicht mitgezählt. Also

    auto fun = [](){};

würde als 6 Zeichen (`[](){}`) zählen. Der Code ist übrigens C++.

## Bestenliste
* 25 -> Rust, Lukas Kalbertodt
* 29 -> Coffeescript, Niels Meyering
* 29 -> Haskell, Niels Meyering
* 42 -> Python, Niels Meyering
* 44 -> C++, Lukas Kalbertodt
* 45 -> Javascript, Christian Heiden
* 49 -> Prolog, Christian Heiden
* 61 -> Java, Christian Heiden
