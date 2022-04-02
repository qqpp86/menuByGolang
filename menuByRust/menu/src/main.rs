mod linktable;
fn main() {
    let mut list: linktable::LinkedList<&str> = linktable::LinkedList::new();
	list.add("Help commend");
	list.add("Hello world!");
	list.add("Menu");
   
    // *list.peek_mut().unwrap() = "4";
	// println!("peek: {:?}\n", list.peek().unwrap());
    //list.delete_link();
	println!("delete: {:?}\n", list.delete());
	println!("head: {:?}\n", list.get_head());
	

	let mut iter = list.into_iter();
	println!("next: {:?}\n", iter.next());
	println!("next: {:?}\n", iter.next());
	println!("next: {:?}\n", iter.next());
}
