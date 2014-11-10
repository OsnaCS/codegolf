/**
 *  CodeGolf is_prime -> 27
 *  =======================
 *  A Lambda that returns true if n is a prime number, false otherwise. Lambda
 *  definition is only 27 characters long.
 * 
 * @author Lukas Kalbertodt
 */

fn main() {
  // The lambda definition is on the right hand side of the assignment.
  // Counted characters:
  //     |.........................|   
  let f= |n|range(2,n).all(|o|n%o>0);

  // just output
  for x in range(2i, 20) {
    println!("{} -> {}", x, f(x)) 
  }
}