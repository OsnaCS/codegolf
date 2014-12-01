/** 
 *   Roman Numeral Converter
 * 
 * \author Lukas Kalbertodt
 */

#include <iostream>
using namespace std;

// This thing is the complete function..  197 chars
void f(int n,int o=0){auto p=[](int i){cout<<"IVXLCDM"[i];};if(n>9)f(n/10,o+2);n%=10;if((n-1)&4){p(o+1);n-=5;}p(o+0);switch(n){case 3:p(o+0);case 2:p(o+0);break;case 4:p(o+1);break;case 9:p(o+2);}}

void test(int num) {
  cout << num << " -> ";
  f(num); 
  cout << endl;
}

int main() {
  test(3);
  test(4);
  test(8);
  test(3999);
}