fn main() {
    let op = Op::Add;
    let expr = make_expressions(1, 2, 3, 4, (Op::Add, Op::Mul, Op::Add))
    // let expr2 = Expr::BinOp((op , Box::new(Expr::Num(3)), Box::new(Expr::Num(4))));
    // let expr3 = Expr::BinOp((op, Box::new(expr1), Box::new(expr2)));
    for e in expr {
        println!("{}", e.eval());
    }
    
}

fn make_ops() -> Vec<(Op,Op,Op)> {
    let mut operators = vec![];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let op1 = determine_op(i);
                let op2 = determine_op(j);
                let op3 = determine_op(k);
                operators.push((op1,op2,op3));
            }
        }
    }
    operators
}

fn boxize_u32(n: u32) -> Box<Option<u32>> {
    Box::new(Some(n))
}

fn make_expressions(a: u32, b: u32, c:u32 , d:u32, op: (Op,Op,Op)) -> Vec<Expr> {
    use Expr::{Num, BinOp};
    let (op1, op2, op3) = op;
    let (a,b,c,d) = (boxize_u32(a), boxize_u32(b), boxize_u32(c), boxize_u32(d));
    vec![
        BinOp(op3, BinOp(op2,BinOp(op1, a, b) , c), d),
        BinOp(op3, BinOp(op1, a, BinOp(op2, b, c)), d),
        
        BinOp(op1, a, BinOp(op3, BinOP(op2, b, c), d)),
        BinOp(op1, a, BinOP(op2, b, BinOp(op3, c, d)),
        BinOp(op2, BinOp(op1, a,b), BinOp(op3, c, d)),
    ]
}

fn determine_op(i: usize) -> Op {
    match i{
        0 => Op::Add,
        1 => Op::Sub,
        2 => Op::Mul,
        _ => Op::Div,
    }
}

trait OptionCalc {
    fn add(&self, other: &Option<u32>) -> Option<u32>;
    fn sub(&self, other: &Option<u32>) -> Option<u32>;
    fn mul(&self, other: &Option<u32>) -> Option<u32>;
    fn div(&self, other: &Option<u32>) -> Option<u32>;
}

impl OptionCalc for Option<u32> {
    fn add(&self, other: &Option<u32>) -> Option<u32> {
        self.and_then(|x| other.map(|y| x + y))
    }
    fn sub(&self, other: &Option<u32>) -> Option<u32> {
        self.and_then(|x| other.map(|y| x - y))
    }
    fn mul(&self, other: &Option<u32>) -> Option<u32> {
        self.and_then(|x| other.map(|y| x * y))
    }
    fn div(&self, other: &Option<u32>) -> Option<u32> {
        self.and_then(|x| {
            match other {
                Some(0) | None => None,
                Some(y) => Some(x / y),
            }
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, PartialEq, Eq)]
enum Expr {
    Num(Option<u32>),
    BinOp((Op, Box<Expr>, Box<Expr>)),
}

impl Expr {
    fn eval(&self) -> Option<u32> {
        match self {
            Expr::Num(x) => *x,
            Expr::BinOp((op, expr1, expr2)) => {
                match op {
                    Op::Add => Option::<u32>::add(&expr1.eval(), &expr2.eval()),
                    Op::Sub => Option::<u32>::sub(&expr1.eval(), &expr2.eval()),
                    Op::Mul => Option::<u32>::mul(&expr1.eval(), &expr2.eval()),
                    Op::Div => Option::<u32>::div(&expr1.eval(), &expr2.eval()),
                }
            },
        }
    }
}

