/** 
 *   Roman Numeral Converter
 * 
 * \author Lukas Kalbertodt
 */

#include <iostream>
using namespace std;
#define x case

// This thing is the complete function..  215 chars
void f(int n,int o=0){auto p=[](int i){cout<<"IVXLCDM"[i];};if(n>9)f(n/10,o+2);n%=10;s:if(n>0){if((n-1)&4){p(o+1);n-=5;goto s;}p(o+0);switch(n){case 3:p(o+0);case 2:p(o+0);break;case 4:p(o+1);break;case 9:p(o+2);}}}

// void f(int n,int o=0){auto p=[](int i){cout<<"IVXLCDM"[i];};if(n>9)f(n/10,o+2);n%=10;if((n-1)&4){p(o+1);n-=5;}p(o+0);switch(n){case 3:p(o+0);case 2:p(o+0);break;case 4:p(o+1);break;case 9:p(o+2);}}

void test(int num) {
  cout << num << " -> ";
  f(num); 
  cout << endl;
}

int main() {
  // for(int i = 1; i < 4000; i++) {
  //   test(i);
  // }
  test(5);
  test(10);
  test(15);
  test(3999);
}