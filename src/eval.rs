use crate::pattern::Symbol;
use num_rational::Ratio;
use Priority::{Commute, NonCommute, Num};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Priority {
    Num(usize),
    Commute(usize),
    NonCommute(usize),
}

pub fn eval(nums: &Vec<Ratio<i32>>, ops: &Vec<char>, pattern: &Vec<Symbol>) -> Option<Ratio<i32>> {
    let n = nums.len();
    let mut n_st = Vec::with_capacity(n);
    let mut n_id = 0;
    let mut ops_id = 0;
    for sym in pattern {
        match sym {
            Symbol::Num => {
                n_st.push(nums[n_id]);
                n_id += 1;
            }
            _ => {
                let y = n_st.pop().unwrap();
                let x = n_st.pop().unwrap();
                match calc(x, y, ops[ops_id]) {
                    Some(x) => n_st.push(x),
                    _ => return None,
                }
                ops_id += 1;
            }
        }
    }

    let n_ans = n_st.pop().unwrap();
    Some(n_ans)
}

pub fn eval_str(nums: &Vec<Ratio<i32>>, ops: &Vec<char>, pattern: &Vec<Symbol>) -> Option<String> {
    let mut s_st = Vec::with_capacity(nums.len());
    let mut n_id = 0;
    let mut ops_id = 0;

    for sym in pattern {
        match sym {
            Symbol::Num => {
                s_st.push((format!("{}", nums[n_id]), Num(3)));
                n_id += 1;
            }
            _ => {
                let sy = s_st.pop().unwrap();
                let sx = s_st.pop().unwrap();

                let res = concat(sx, sy, ops[ops_id]);
                s_st.push(res);
                ops_id += 1;
            }
        }
    }
    Some(s_st.pop().unwrap().0)
}

#[inline]
fn calc(x: Ratio<i32>, y: Ratio<i32>, ops: char) -> Option<Ratio<i32>> {
    match ops {
        '+' => Some(x + y),
        '-' => Some(x - y),
        '*' => Some(x * y),
        '/' if y == 0.into() => None,
        '/' => Some(x / y),
        _ => panic!("Unknown binary operator is given"),
    }
}

fn concat(x: (String, Priority), y: (String, Priority), ops: char) -> (String, Priority) {
    let priority = match ops {
        '+' | '-' => 1,
        _ => 2,
    };
    let op_info = match ops {
        '+' => Commute(priority),
        '-' => NonCommute(priority),
        '*' => Commute(priority),
        '/' => NonCommute(priority),
        _ => unreachable!(),
    };
    let xp = match x.1 {
        Commute(x) => x,
        NonCommute(x) => x,
        Num(x) => x,
    };
    let yp = match y.1 {
        Commute(x) => x,
        NonCommute(x) => x,
        Num(x) => x,
    };

    if xp < priority && yp < priority {
        (format!("({}){}({})", x.0, ops, y.0), op_info)
    } else if xp < priority {
        (format!("({}){}{}", x.0, ops, y.0), op_info)
    } else if yp < priority || (op_info == NonCommute(priority) && y.1 != Num(yp)) {
        (format!("{}{}({})", x.0, ops, y.0), op_info)
    } else {
        (format!("{}{}{}", x.0, ops, y.0), op_info)
    }
}

#[test]
fn test_eval() {
    use crate::pattern::Patterns;
    let nums: Vec<Ratio<i32>> = vec![1.into(), 2.into(), 7.into(), 0.into()];
    let ops = vec!['+', '+', '+'];
    let e = Patterns::new(4);
    let ans = eval(&nums, &ops, &e.get(4)[0]);
    eprintln!("e.get(4)[0] = {:?}", &e.get(4)[0]);
    let acc = Some(10.into());

    for l in e.get(4) {
        let ans = eval(&nums, &ops, l);
        eprintln!("ans = {:?}", ans);
    }
    assert_eq!(acc, ans);
}
