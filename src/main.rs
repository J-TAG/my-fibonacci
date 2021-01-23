fn main() {
    println!("Hello, world!");
    println!("Fibonacci 15: {}", fibonacci(15));
    println!("Fibonacci 2: {}", fibonacci(2));
}


/// `n` the rank used to compute the member of the sequence.
pub fn fibonacci(n: i32) -> u64 {
	if n < 0 {
		panic!("{} is negative!", n);
	} else if n == 0 {
		panic!("zero is not a right argument to fibonacci()!");
	} else if n == 1 {
		return 1;
	}

	let mut sum = 0;
	let mut last = 0;
	let mut curr = 1;
	for _i in 1..n {
		sum = last + curr;
		last = curr;
		curr = sum;
	}
	sum
}

#[test]
fn fib_15() {
    assert_eq!(fibonacci(15), 610);
}

#[test]
fn fib_2() {
    assert_eq!(fibonacci(2), 1);
}
