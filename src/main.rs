use std;
import std::io;

fn main(args: [str]) {
	let argc = vec::len(args);
	let seq = [];
	if argc == 1u || argc == 2u {
		let count =  argc % 2u == 0u ? uint::from_str(args[1]) : 10u;
		seq = lucas::fib(count);
	} else if argc == 3u || argc == 4u {
		let n1 = int::from_str(args[1]),
			n2 = int::from_str(args[2]),
			count = argc % 2u == 0u ? uint::from_str(args[3]) : 10u;
		seq = lucas::seq(n1, n2, count);
	}
	
	for elt in seq {
		io::println(int::str(elt));
	}
			
}

