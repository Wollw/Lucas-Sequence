/*
	Lucas Sequence Generator

	This crate implements a Lucas sequence generator.  It provides
	a generalized function for creating sequences from a user defined
	seed and also provides a convenience function for the Fibonacci
	sequence.  Both functions returns uint vectors.

*/

fn seq(n1: int, n2: int, count: uint) -> [int] {
/*
		Postcondition:
			A vectorized sequence of lucas numbers is returned
			using n1 and n2 as the starting numbers and containing
			the number of digits specified by count.
*/
	let seq = [], i = 0u;
	while i < count {
		if i == 0u {
			seq += [n1]
		} else if i == 1u {
			seq += [n2]
		}
		else {
			seq += [seq[i-1u] + seq[i-2u]]
		}
		i = i + 1u
	}
	ret seq
}




fn fib(count: uint) -> [int] {
/*
		Postcondition:
			A vectorized sequence of Fibonacci numbers of length defined
			by count is returned.

*/
	ret seq(0, 1, count);
}
