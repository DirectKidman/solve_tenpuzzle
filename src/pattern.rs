use Symbol::{Num, Op};
type Expr = Vec<Symbol>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Num,
    Op,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pattern {
    expr: Expr,
    num: usize,
}

impl Pattern {
    #[inline]
    fn can_push(&self, sym: Symbol) -> bool {
        match sym {
            Op if self.num >= 2 => true,
            Op => false,
            _ => true,
        }
    }

    fn push(&self, sym: Symbol) -> Self {
        let mut expr = self.expr.clone();
        expr.push(sym);
        let num = match sym {
            Num => self.num + 1,
            _ => self.num - 1,
        };
        Self {
            expr: expr,
            num: num,
        }
    }
}

pub struct Patterns {
    expressions: Vec<Vec<Expr>>,
}

impl Patterns {
    pub fn new(num: usize) -> Self {
        let mut all_patterns: Vec<Vec<Pattern>> = vec![vec![]; 2 * num];
        all_patterns[1].push(Pattern {
            expr: vec![Num],
            num: 1,
        });
        for i in 1..2 * num - 1 {
            let mut tmp_vec: Vec<Pattern> = vec![];
            for expr in &all_patterns[i] {
                tmp_vec.push(expr.push(Num));
                if expr.can_push(Op) {
                    tmp_vec.push(expr.push(Op));
                }
            }

            all_patterns[i + 1] = tmp_vec;
        }

        let mut expressions: Vec<Vec<Expr>> = vec![vec![]; 2 * num];
        for i in 0..2 * num {
            expressions[i] = all_patterns[i]
                .iter()
                .cloned()
                .filter(|x| x.num == 1)
                .map(|x| x.expr)
                .collect::<Vec<_>>();
            expressions[i].sort();
        }
        Self { expressions }
    }

    #[inline]
    pub fn get(&self, id: usize) -> &Vec<Expr> {
        &self.expressions[2 * id - 1]
    }
}

#[test]
fn test_expressions() {
    let e = Patterns::new(4);
    // println!("{:?}", e.expressions);

    let vec = vec![
        vec![Num, Num, Num, Num, Op, Op, Op],
        vec![Num, Num, Num, Op, Num, Op, Op],
        vec![Num, Num, Num, Op, Op, Num, Op],
        vec![Num, Num, Op, Num, Num, Op, Op],
        vec![Num, Num, Op, Num, Op, Num, Op],
    ];
    assert_eq!(&vec, e.get(4));
}
