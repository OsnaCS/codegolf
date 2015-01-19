
#![allow(unstable)]

use std::rand::random as r;
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

fn g(a:u32,z:u32){if a<=z{let s=r::<u32>()%(z-a+1)+a;g(a,s-1);g(s+1,z);print!("{} ", s)}}


fn main() {
  let random_postorder = |&:n|g(1,n);

  for i in 1..10 {
    print!("{} -> ", i);
    random_postorder(i);
    println!("");
  }
  


  // let p = |&:n:i32|(1..n+1);
  // let d : Vec<i32> = (1..9).collect();
  // f(d.as_slice());
  // println!("{}", d);
}