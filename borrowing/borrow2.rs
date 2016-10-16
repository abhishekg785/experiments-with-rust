// vectors uses heaps under the hood
// basically we have a stack having => size, capacity, pointer to the heap
// use heap for expandable memory

fn main(){
	let free_coloring_book = vec![
	    "mercury",
	    "venus",
	    "earth",
	    "mars",
	    "jupiter",
	    "saturn",
	    "uranus",
	    "neptune"
	];

	let coloring_book = free_coloring_book;  // claiming ownership

	println!("{:?}", coloring_book);
	
	// println!("Free coloring book looks like:\n {:?}", free_coloring_book);
	// for i in &free_coloring_book{
	// 	println!("{}", i);
	// }
}
