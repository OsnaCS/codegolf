// random_postorder (Codegolf Aufgabe)
// Gelößt mit 77 + 15 = 92 Zeichen
 


// Alpha-Warnungen unterdrücken 
#![allow(unstable)]

// Diese alias Teile werden nicht mitgezählt!
use std::rand::random as r;
type I = usize;

// Hauptfunktion: Die ganze nächste Reihe wird gezählt. => 77
fn g(a:I,z:I){if a<z{let s=r::<I>()%(z-a)+a;g(a,s);g(s+1,z);print!("{} ",s)}}


fn main() {
  // Lambda zum Wrappen: Nur Definition zählt.
  //                     |.............|  => 15
  let random_postorder = |&:n:I|g(1,n+1);

  // random_postorder(9);

  for i in 1..10 {
    print!("{} -> ", i);
    random_postorder(i);
    println!("");
  }
}