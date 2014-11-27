/*!

  HappyNumberTest in Rust

\author Lukas Kalbertodt
*/


fn f(mut n:int)->bool{let mut c=0;while n>0{c+=(n%10)*(n%10);n/=10}c<2||c>4&&f(c)}

fn main() {
  // output
  for e in range(1, 20) {
    println!("{} -> {}", e, f(e));
  }
}