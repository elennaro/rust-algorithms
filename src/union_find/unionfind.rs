pub trait UnionFind {
	fn union(&mut self, usize, usize);
	fn connected(&mut self, usize, usize) -> bool;
	fn find(&mut self, usize) -> usize;
}

