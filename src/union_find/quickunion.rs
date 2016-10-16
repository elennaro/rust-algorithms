use union_find::unionfind::*;

#[derive(Debug)]
pub struct QuickUnion {
	id: Vec<usize>
}

///
/// Quick Union implementation of union find task.
///
impl UnionFind for QuickUnion {
	fn union(&mut self, p: usize, q: usize) {
		self.id[p] =  self.id[q];
	}
	fn connected(&self, p: usize, q: usize) -> bool {
		return self.find(p) == self.find(q);
	}
	fn find(&self, p: usize) -> usize {
		let mut i = p;
		while i != self.id[i] {
			i = self.id[i]
		}
		return i;
	}
}

impl QuickUnion {
	pub fn new(n: usize) -> QuickUnion {
		let mut s: Vec<usize> = Vec::new();
		for x in 0..n {
			s.push(x);
		}
		QuickUnion {
			id: s
		}
	}
}