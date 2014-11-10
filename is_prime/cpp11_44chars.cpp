/**
 *  CodeGolf is_prime -> 44
 *  =======================
 *  A c++11 lambda returning true if input n is a prime number, false 
 *  otherwise. Lambda definition is 44 characters long.  
 * 
 * @author Lukas Kalbertodt
 */
#include <iostream>
using namespace std;


int main() {
  // The lambda definition is on the right hand side of the assignment.
  // Counted characters:
  //       |..........................................|   
	auto l = [](int n){int i=1;while(n%++i);return i==n;};

  // just output
	for(int i = 2; i < 20; ++i) {
		cout << i << " -> " << l(i) << endl;

	}
}



