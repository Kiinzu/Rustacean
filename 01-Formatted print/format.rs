fn main(){
	// Just need a {} to print the value, the value exist after the coma
	println!("{} days",31);
	
	// Value can be add in an index-like format after the coma, starting at index 0
	println!("Hello {0},this is {1}. Hi {1}, I'm {0}", "Doggo", "Meowi");

	// We can set them as named arguments too
	println!("{subject} {verb} {object}",
		subject="Andy",
		verb="talks to",
		object="Janneth");

	// To make our code clean, we can right-justify it
	println!("{name:>15}",name="Judith");

	// Flipping the sign to < can left-justify it, also we can add 0 behing the space
	println!("{number:0<5}",number=34);

	// We can also add named arguments in format specifier using $
	println!("{number:0<width$}",number=69,width=5);

	// Rust also check missing argument, but you know, we are gonna comment this
	//println!("My name is {0}, {1} {0}", "Bond");
    	// FIXME ^ Add the missing argument: "James"
	// FIXED println!("My name is {0}, {1} {0}", "Bond","James");
  	#[allow(dead_code)] // disable `dead_code` which warn against unused module
    	struct Structure(i32);

    	// This will not compile because `Structure` does not implement
    	// fmt::Display.
    	// println!("This struct `{}` won't print...", Structure(3));
    	// TODO ^ Try uncommenting this line
	let age:usize = 21;
	let name = "Richard";
	let width:usize = 3;
	println!("Hello I'm {name} and I'm {age:0>width$} y.o., Nice to meet ya!");

}
