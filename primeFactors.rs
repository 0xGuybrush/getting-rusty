fn primeFactors(n: int, currentResult: @[int]) -> @[int] {
	currentResult + match n {
		0 | 1 => @[],
		2..3 => @[n],
		_ => {
			if n % 2 == 0 {
				primeFactors(2, primeFactors(n/2, @[]))
			} else if n % 3 == 0 {
				primeFactors(3, primeFactors(n/3, @[]))	
			} else {
				@[n]
			}
		}
	}
}

fn assertResult(numberToCheck: int, expectedResult: @[int]) {
	let actualResult = primeFactors(numberToCheck, @[]);
	assert!(actualResult == expectedResult, 
		"The prime factor of \""+ numberToCheck.to_str() + "\" should be \"" + expectedResult.to_str() 
			+ "\", but was \"" + actualResult.to_str() + "\"");
}

#[test]
fn zero() {
	assertResult(0, @[])
}

#[test]
fn one() {	
	assertResult(1, @[])
}

#[test]
fn two() {
	assertResult(2, @[2])
}

#[test]
fn three() {
	assertResult(3, @[3])
}

#[test]
fn four() {
	assertResult(4, @[2,2])
}

#[test]
fn five() {
	assertResult(5, @[5])
}

#[test]
fn six(){
	assertResult(6, @[3, 2])
}

#[test]
fn seven() {
	assertResult(7, @[7])
}

#[test]
fn eight() {
	assertResult(8, @[2,2,2])
}

#[test]
fn nine() {
	assertResult(9, @[3,3])
}

#[test]
fn ten() {
	assertResult(10, @[5, 2])
}

#[test]
fn ninetyOne() {
	assertResult(91, @[13, 7])
}
