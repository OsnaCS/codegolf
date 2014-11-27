/**
 * Happy number calculation in C
 * Author: Lars Kiesow <lkiesow@uos.de>
 *
 * gcc -o happyNumberC_76chars happyNumberC_76chars.c
 *
 * echo -n 'int h(int x){int n=0;while(x){n+=x%10*(x%10);x/=10;}return n==1||n>4&&h(n);}'|wc -c
 *
 * -> 76
 */

int h(int x){int n=0;while(x){n+=x%10*(x%10);x/=10;}return n==1||n>4&&h(n);}

/* Print happy numbers in {x \in \N | x<101} */
#include <stdio.h>

int main() {
	int i;
	for (i = 1; i <= 100; i++) {
		if (h(i)) {
			printf("%i ", i);
		}
	}
	printf("\n");
}
