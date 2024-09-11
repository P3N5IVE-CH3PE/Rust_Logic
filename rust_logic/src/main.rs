fn main() {
    	//let condition = false;

	//let number =  if condition {5} else {6};


	//println!("The value of number is:{number}");

	//	let x = 1;

	//  	let y = if x { 0 } else { 1 };
 
	//  	println!("{y}");
	

	//	loop {println!("Again!");}

			

	let a = [100,200,300,400,500];
	let mut index = 0;
	
	while index < 5 {
		println!("The value is: {}",a[index]);
		
		index += 1;
	
	}


	let a = [10,20,30,40,50];

	for element in a {
	
	println!("The value is: {element}");
	}


	for number in (1..4).rev(){
		println!("{number}")
	}
	println!("Take Off!");
}
