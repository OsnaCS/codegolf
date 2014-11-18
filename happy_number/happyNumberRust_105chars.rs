/*!

  HappyNumberTest in Rust

\author Lukas Kalbertodt
*/

fn main() {

  //! Version without lists or something: Just simple control structures...
  let func = |mut n:int|{while(n>1&&n!=4){n=(|mut s:int|{let mut f=0i;while(s>0){f+=(s%10)*(s%10);s/=10;}f})(n);};n<2};

  //! Older/longer versions: Most of them use lists in some way
  // let func = |n:int|iterate(n,|mut s|{let mut f=0i;while(s>0){f+=(s%10)*(s%10);s/=10;}f}).find(|m|*m==1||*m==4)==Some(1);
  // let func = |n:int|iterate(n,|s|iterate(s,|x|x/10).take_while(|m|*m>0).fold(0,|x,y|x+(y%10)*(y%10))).find(|m|*m==1||*m==4)==Some(1);
  // let func = |n:int|(iterate(n,|s|iterate(s,|x|x/10).take_while(|m|*m!=0).fold(0,|x,y|x+(y%10)*(y%10))).find(|m|*m==1||*m==4))==Some(1);
  // let func = |n:int|(iterate(n,|s|Unfold::new(s*10,|x|{*x/=10;if *x==0{None}else{Some(*x)}}).fold(0,|x,y|x+(y%10)*(y%10))).find(|m|*m==1||*m==4))==Some(1);

  // output
  for e in range(1, 20) {
    println!("{} -> {}", e, func(e));
  }
}