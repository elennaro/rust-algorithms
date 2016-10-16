use union_find::unionfind::*;

#[derive(Debug)]
pub struct WeightedUnion {
	id: Vec<usize>,
	size: Vec<usize>
}

///
/// Quick Union implementation of union find task.
///
impl UnionFind for WeightedUnion {
	fn union(&mut self, p: usize, q: usize) {
		let root_p = self.find(p);
		let root_q = self.find(q);
		if root_p == root_q {
			return;
		}
		// make smaller root point to larger one
		if self.size[root_p] < self.size[root_q] {
			self.id[root_p] = root_q;
			self.size[root_q] += self.size[root_p];
		} else {
			self.id[root_q] = root_p;
			self.size[root_p] += self.size[root_q];
		}
		self.id[p] = self.id[q];
	}
	fn connected(&mut self, p: usize, q: usize) -> bool {
		return self.find(p) == self.find(q);
	}
	fn find(&mut self, p: usize) -> usize {
		let mut i = p;
		while i != self.id[i] {
			i = self.id[i];
		}
		return i;
	}
}

impl WeightedUnion {
	pub fn new(n: usize) -> WeightedUnion {
		let mut id: Vec<usize> = Vec::new();
		let size: Vec<usize> = vec![1; n-1];
		for x in 0..n {
			id.push(x);
		}
		WeightedUnion {
			id: id,
			size: size
		}
	}
}