pub trait UnionFind {
	fn union(&mut self, usize, usize);
	fn connected(&self, usize, usize) -> bool;
	fn find(&self, usize) -> usize;
}

