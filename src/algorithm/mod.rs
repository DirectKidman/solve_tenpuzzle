pub struct Permutation{
    n : usize,
    v : Vec<usize>,
    first_next : bool,
}

impl Permutation {
    pub fn new(n : usize) -> Self{
        let mut tmp : Vec<usize> = Vec::new();
        for i in 0..n {
            tmp.push(i);
        }
        Permutation{n: n, v: tmp, first_next: true}
    }
}

impl Iterator for Permutation {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_next {
            let x = self.v.clone();
            self.first_next = false;

            Some(x)
        }else{
            let mut max_leq_id = 0;
            let mut f = false;
            for i in 0..(self.n-1) {
                if self.v[i] < self.v[i+1] {
                    max_leq_id = i;
                    f = true;
                }
            }

            if !f {
                return None;
            }

            let mut greater_id = self.n-1;
            for i in 0..(self.n-1) {
                if self.v[max_leq_id] < self.v[self.n - i - 1] {
                    greater_id = self.n - i - 1;
                    break;
                }
            }

            
            // std::mem::swap(&mut self.v[max_leq_id], &mut self.v[max_leq_id+1]);

            self.v.swap(max_leq_id,greater_id);

            for i in 0.. (self.n - max_leq_id)/2 {
                self.v.swap(i + max_leq_id + 1, self.n - i - 1);
            }

            let v_res = self.v.clone();
            Some(v_res)
        }

        
    }
}
