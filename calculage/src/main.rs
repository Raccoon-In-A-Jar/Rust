//calculage
//use std::ops::Add;
use std::io;

fn main() {

	let nb: u8 = 65;
	println!("{:?}", nb as char);




	println!("Entrez un nombre et découvrez le caractère ASCII associé (ou pas) : ");
		
	let mut lecode = String::new();

	io::stdin().read_line(&mut lecode).expect("Marche pas !");

	println!("{}", lecode);

	let _lecode: u8 = lecode.parse().expect("Pas un nombre ! ^^");

	println!("Vous avez entré : {}", _lecode as char);

}
