use union_find::unionfind::*;

#[derive(Debug)]
pub struct FlatUnion {
	id: Vec<usize>
}

///
/// Quick Union implementation of union find task.
///
impl UnionFind for FlatUnion {
	fn union(&mut self, p: usize, q: usize) {
		self.id[p] = self.id[q];
	}
	fn connected(&mut self, p: usize, q: usize) -> bool {
		return self.find(p) == self.find(q);
	}
	fn find(&mut self, p: usize) -> usize {
		let mut i = p;
		while i != self.id[i] {
			self.id[i] = self.id[self.id[i]];
			i = self.id[i];
		}
		return i;
	}
}

impl FlatUnion {
	pub fn new(n: usize) -> FlatUnion {
		let mut s: Vec<usize> = Vec::new();
		for x in 0..n {
			s.push(x);
		}
		FlatUnion {
			id: s
		}
	}
}