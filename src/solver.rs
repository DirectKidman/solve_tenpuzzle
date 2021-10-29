use num_rational::Ratio;
use permutohedron::LexicalPermutation as _;
const ANS: f32 = 10.0;

pub fn new_solver(mut v: Vec<Ratio<usize>>, goal: usize) -> Vec<String> {
    loop {
        if !v.next_permutation() {
            break;
        }
    }
    vec![]
}

fn _inner_solver(lhs: Ratio<usize>, rhs: Ratio<usize>) -> Vec<Ratio<usize>> {
    let mut res = vec![];
    res.push(lhs + rhs);
    res.push(lhs - rhs);
    res.push(lhs * rhs);
    if rhs != Ratio::from_integer(0) {
        res.push(lhs / rhs);
    }

    res
}

pub fn solve(v: &mut Vec<f32>) -> Vec<String> {
    let mut perm = crate::util::Permutation::new(4);
    let ops = make_op_vec();
    let mut res = Vec::new();

    loop {
        let x = perm.next();
        let id;

        match x {
            Some(x) => id = x,
            None => break,
        }

        let a: f32 = v[id[0]];
        let b: f32 = v[id[1]];
        let c: f32 = v[id[2]];
        let d: f32 = v[id[3]];

        for op in &ops {
            for i in 0..5 {
                if i == 0 {
                    let c1 = match calc(a, b, op[0]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let c2 = match calc(c1, c, op[1]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let ans = match calc(c2, d, op[2]) {
                        Some(x) => x,
                        None => continue,
                    };

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("(({}{}{}){}{}){}{}", a, &op[0], b, &op[1], c, &op[2], d);
                        res.push(s);
                    }
                } else if i == 1 {
                    let c1 = match calc(b, c, op[1]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let c2 = match calc(a, c1, op[0]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let ans = match calc(c2, d, op[2]) {
                        Some(x) => x,
                        None => continue,
                    };

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("({}{}({}{}{})){}{}", a, &op[0], b, &op[1], c, &op[2], d);
                        res.push(s);
                    }
                } else if i == 2 {
                    let c1 = match calc(c, d, op[2]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let c2 = match calc(b, c1, op[1]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let ans = match calc(a, c2, op[0]) {
                        Some(x) => x,
                        None => continue,
                    };

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("{}{}({}{}({}{}{}))", a, &op[0], b, &op[1], c, &op[2], d);
                        res.push(s);
                    }
                } else if i == 3 {
                    let c1 = match calc(b, c, op[1]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let c2 = match calc(c1, d, op[2]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let ans = match calc(a, c2, op[0]) {
                        Some(x) => x,
                        None => continue,
                    };

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("{}{}(({}{}{}){}{})", a, &op[0], b, &op[1], c, &op[2], d);
                        res.push(s);
                    }
                } else if i == 4 {
                    let c1 = match calc(a, b, op[0]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let c2 = match calc(c, d, op[2]) {
                        Some(x) => x,
                        None => continue,
                    };

                    let ans = match calc(c1, c2, op[1]) {
                        Some(x) => x,
                        None => continue,
                    };

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("({}{}{}){}({}{}{})", a, &op[0], b, &op[1], c, &op[2], d);
                        res.push(s);
                    }
                }
            }
        }
    }
    res
}

fn calc(x: f32, y: f32, op: char) -> Option<f32> {
    if op == '+' {
        Some(x + y)
    } else if op == '-' {
        Some(x - y)
    } else if op == '*' {
        Some(x * y)
    } else if y != 0.0 {
        Some(x / y)
    } else {
        None
    }
}

fn make_op_vec() -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    let op = vec!['+', '-', '*', '/'];

    for e in &op {
        for ee in &op {
            for eee in &op {
                let mut tmp: Vec<char> = Vec::new();
                tmp.push(e.clone());
                tmp.push(ee.clone());
                tmp.push(eee.clone());

                res.push(tmp);
            }
        }
    }

    res
}

pub fn solve_all(limit: usize) -> u32 {
    let mut ans: u32 = 0;
    let mut v: Vec<f32> = Vec::new();

    for i in 0..=limit {
        for j in i..=limit {
            for k in j..=limit {
                for l in k..=limit {
                    v.clear();
                    v.push(i as f32);
                    v.push(j as f32);
                    v.push(k as f32);
                    v.push(l as f32);

                    let res = solve(&mut v);
                    for e in &res {
                        println!("{}", e);
                    }
                    //println!("{:?}",v);

                    if res.len() >= 1 {
                        ans += 1;
                    }
                }
            }
        }
    }
    ans
}
