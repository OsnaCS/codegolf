/**
 *	author: Christian Heiden
 *
 *	This program checks if a number n is prime or not.
 *	# of chars: 61
 */
public class IsPrim{
  
	static boolean p(int n){int c=n;while(n%--c!=0);return c==1;}

	public static void main (String[]papperlapapp){
		for (int i = 2; i < 100; i++){
			System.out.printf("%4d: %10b\n", i, p(i));
		}
	}
}
