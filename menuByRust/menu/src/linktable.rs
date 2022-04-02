use std::rc::Rc;
use std::cell::{Ref, RefCell};

type Link<String> = Option<Rc<RefCell<Node<String>>>>;



#[derive(Debug)]
//链表节点定义
pub struct Node<String> {
	elem: String,
	next: Link<String>,
}

#[derive(Debug)]
//链表结构定义
pub struct LinkedList<String> {
	head: Link<String>,
	tail: Link<String>,
}


impl<String> LinkedList<String> {
	//初始化，首尾声明为None
	pub fn new() -> Self {
		LinkedList {
			head: None,
			tail: None,
		}
	}

	pub fn add(&mut self, elem : String) {
		//先建立一个新节点
		let new_point = Rc::new(RefCell::new(Node {
			elem,
			next: self.head.take()
		}));

		//
		if self.tail.is_none() {
			self.tail = Some(Rc::clone(&new_point));
		}
		self.head = Some(new_point);
	}
	
	pub fn delete(&mut self) -> Option<String> {
		//把head指向的值拿出来
		let first_link = self.head.take();
		//把head指向头结点的next
		let next_link = match first_link {
			Some(ref x) => x.borrow_mut().next.take(),
			None => None
		};
		self.head = next_link;
		//如果此时的head是空链接，需要将tail链接也要设置为空
		if self.head.is_none() {
			self.tail.take();
		}
		//返回值，使用.map可以将节点的值读取出来
		first_link.map(|r| Rc::try_unwrap(r).ok().unwrap().into_inner().elem)
	}

	//直接将首尾指针置空
	pub fn delete_linktable(&mut self)  {
		self.head.take();
		self.tail.take();
	}

	//获取首节点，通过list.peek().as_ref().unwrap().elem可以读出节点元素
	pub fn get_head(&self) -> Option<Ref<String>> {
		self.head.as_ref().map(|r| Ref::map(
			r.borrow(), |node| &node.elem
		))
	}


	pub fn into_iter(self) -> IntoIter<String> {
		IntoIter(self)
	}
}

pub struct IntoIter<String>(LinkedList<String>);

impl<String> Iterator for IntoIter<String> {
	type Item = String;
	fn next(&mut self) -> Option<Self::Item> {
		self.0.delete()
	}
}

