use union_find::unionfind::*;

#[derive(Debug)]
pub struct QuickFind {
    id: Vec<usize>
}
///
/// Quick Find implementation of union find task.
///
impl UnionFind for QuickFind {
    fn union(&mut self, p: usize, q: usize) {
        let pid = self.id[p];
        let qid = self.id[q];
        for i in 0..self.id.len() {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }
    }
    fn connected(&self, p: usize, q: usize) -> bool {
        return self.id[p] == self.id[q];
    }
    fn find(&self, p: usize) -> usize {
        return self.id[p] as usize;
    }
}

impl QuickFind {
    pub fn new(n: usize) -> QuickFind {
        let mut s: Vec<usize> = Vec::new();
        for x in 0..n {
            s.push(x);
        }
        QuickFind {
            id: s
        }
    }
}