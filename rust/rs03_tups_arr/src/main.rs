fn main() {
	let tup1: (i32, f64, u8) = (500, 6.4, 1);
	// Type annotation not needed, but tup2 will have a different types!
	let tup2 = (500, 6.4, 1);

	// Destructuing a tuple:

	//TODO: use a single-line assignment to assign the components of tup1
	// to variables `x`, `y`, and `z`, respectively

	//<Your code here>

	println!("x: {}, y: {}, z: {}", x, y, z);

	// Individual access:
	println!("tup2: ({}, {}, {})", tup2.0, tup2.1, tup2.2);

	// Handy way to check a type:
	// Attempt assignment to a known wrong type and check the compiler error
	//let i: char = z;
	//let j: char = tup2.2;

	// Arrays
	//TODO: create an array containing ints 1, 2, 3, 4, and 5 and assign it to `arr1`

	//<Your code here>

	//TODO: create an array containing 5 3's and assign it to `arr2`

	//<Your code here>

	println!("arr1[3]: {}", arr1[3]);
	println!("arr2[0]: {}, arr2[4]: {}", arr2[0], arr2[4]);

	// Out-of-bounds access will panic at runtime
	// By default, compiler will try to warn of this when it can
	//println!("arr2[5]: {}", arr2[5]);
}
