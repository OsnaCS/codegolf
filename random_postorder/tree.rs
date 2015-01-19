
#![allow(unstable)]

use std::rand::random as r;
type I = usize;
// fn f(d: &[i32]) {
//   // print!("CALL: ");
//   // for e in d.iter() {
//   //   print!("{} ", e);
//   // }
//   // println!("");

//   if d.len()>0 {
//     let s = std::rand::random::<usize>()%(d.len());
//     // print!("len: {}, s: {}", d.len(), s);
//     // println!("");

//     f(&d[0..s]);
//     f(&d[s+1..d.len()]);

//     print!("{} ", d[s]);
//   }
// }

// fn f(d: &[i32]) {
//   if d.len()>0 {
//     let s = r::<usize>()%(d.len());
//     f(&d[0..s]);
//     f(&d[s+1..d.len()]);
//     print!("{} ", d[s]);
//   }
// }

// z inklusive
// a=3, z=7 (Will: 3 + rand()%5)
// a=3, z=4 -> JA ; a=3,z=3 -> JA ; a=3, z=2 -> NEIN   ===> a <= z
// fn g(a:I,z:I){if a<=z{let s=r::<I>()%(z-a+1)+a;g(a,s-1);g(s+1,z);print!("{} ",s)}}
// z exklusive
// a=3, z=7 (Will: 3 + rand()%4)
// a=3, z=4 -> JA ; a=3,z=3 -> NEIN ; a=3, z=2 -> NEIN  ====> a < z
// fn g(a:I,z:I){if a<z{let s=r::<I>()%(z-a)+a;g(a,s-2);g(s+1,z);print!("{} ",s)}}


// Hauptfunktion: Die ganze nächste Reihe wird gezählt. => 77
fn g(a:I,z:I){if a<z{let s=r::<I>()%(z-a)+a;g(a,s);g(s+1,z);print!("{} ",s)}}


// a <= z
// a-1 < z

fn main() {
  // Lambda zum Wrappen: Nur Definition zählt.
  //                     |.............|  => 15
  let random_postorder = |&:n:I|g(1,n+1);

  random_postorder(9);

  // for i in 1..10 {
  //   print!("{} -> ", i);
  //   random_postorder(i);
  //   println!("");
  // }
  


  // let p = |&:n:i32|(1..n+1);
  // let d : Vec<i32> = (1..9).collect();
  // f(d.as_slice());
  // println!("{}", d);
}