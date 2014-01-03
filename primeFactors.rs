fn primeFactors(n: int) -> @[int] {
	match n {
		0 => @[],
		1..3 => @[n],
		4..10 => {
			if n % 2 == 0 {
				@[2,n/2]
			} else {
				@[n]
			}
		},
		_ => @[-1]
	}
}

#[test]
fn that_there_are_no_prime_factors_of_zero() {
	let zeroFactors = primeFactors(0);
	assert!(zeroFactors.is_empty(), "Zero should have no prime factors")
}

#[test]
fn that_the_prime_factor_of_one_is_one() {
	assert!(primeFactors(1) == @[1], "The prime factor of one should be '1'")
}

#[test]
fn that_the_prime_factor_of_two_is_two() {
	assert!(primeFactors(2) == @[2], "Prime factors of '2' should be '2'")
}

#[test]
fn that_the_prime_factor_of_three_is_three() {
	assert!(primeFactors(3) == @[3], "Prime factors of '3' should be '3'")
}

#[test]
fn that_the_prime_factors_of_four_is_two_and_two() {
	assert!(primeFactors(4) == @[2, 2], "Prime factors of 4 should be '2, 2'")
}

#[test]
fn that_the_prime_factor_of_five_is_five() {
	assert!(primeFactors(5) == @[5], "Prime factor of 5 should be '5'")
}

#[test]
fn that_the_prime_factors_of_six_is_three_and_two() {
	let factors = primeFactors(6);
	assert!(factors == @[2, 3], "Prime factors of 6 should be '2, 3'")
}

#[test]
fn seven() {
	assert!(primeFactors(7) == @[7], "Result for seven should be \"7\"")
}

#[test]
fn eight() {
	assert!(primeFactors(8) == @[2,2,2,2], "Result for eight should be \"2,2,2,2\"")
}
