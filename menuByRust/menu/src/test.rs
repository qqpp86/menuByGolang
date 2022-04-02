use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

#[derive(Debug)]
struct LinkedList<T> {
	head: Link<T>,
	tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
	elem: T,
	next: Link<T>,
}

impl<T> LinkedList<T> {
	fn new() -> Self {
		LinkedList {
			head: None,
			tail: None,
		}
	}

	fn push(&mut self, elem: T) {
		let new_point = Rc::new(RefCell::new(Node {
			elem,
			next: self.head.take()
		}));
		if self.tail.is_none() {
			self.tail = Some(Rc::clone(&new_point));
		}
		self.head = Some(new_point);
	}

	fn pop_link(&mut self) -> Link<T> {
		let first_link = self.head.take();
		let next_link = match first_link {
			Some(ref x) => x.borrow_mut().next.take(),
			None => None
		};
		self.head = next_link;
		if self.head.is_none() {
			self.tail.take();
		}
		first_link
	}

	fn pop(&mut self) -> Option<T> {
		let first_link = self.head.take();
		let next_link = match first_link {
			Some(ref x) => x.borrow_mut().next.take(),
			None => None
		};
		self.head = next_link;
		if self.head.is_none() {
			self.tail.take();
		}
		first_link.map(|r| Rc::try_unwrap(r).ok().unwrap().into_inner().elem)
	}

	fn peek_node(&self) -> Option<Ref<Node<T>>> {
		self.head.as_ref().map(|r| r.borrow())
	}

	fn peek(&self) -> Option<Ref<T>> {
		self.head.as_ref().map(|r| Ref::map(
			r.borrow(), |node| &node.elem
		))
	}

	fn peek_mut(&mut self) -> Option<RefMut<T>> {
		self.head.as_ref().map(|r| RefMut::map(
				r.borrow_mut(), |node| &mut node.elem
		))
	}

	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}

struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

fn main() {
	let mut list: LinkedList<u32> = LinkedList::new();
	list.push(1);
	list.push(2);
	list.push(3);
	list.push(4);

	*list.peek_mut().unwrap() = 5;
	println!("peek: {:?}\n", list.peek().unwrap());

	println!("pop: {:?}\n", list.pop());

	let mut iter = list.into_iter();
	println!("next: {:?}\n", iter.next());
    //println!("peek: {:?}\n", iter.peek().unwrap());
	println!("next: {:?}\n", iter.next());
	println!("next: {:?}\n", iter.next());
	println!("next: {:?}\n", iter.next());

    
}