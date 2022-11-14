use std::collections::BTreeSet;

use crate::{
    eval::{eval, eval_str},
    pattern::{Patterns, Symbol},
};
use itertools::Itertools;
use num_rational::Ratio;
use std::time::Instant;

pub fn solve_all(target: i32, n_use: usize, limit: isize) -> usize {
    let mut count = 0;
    let patterns = Patterns::new(n_use);
    let mut lp = 0;
    let mut time = 0;
    let mut sols = BTreeSet::new();
    for nums in (0..=limit as i32).combinations_with_replacement(n_use) {
        let start = Instant::now();
        let c = solve(&nums, target, true, false, &patterns.get(n_use));
        let elapsed = start.elapsed();
        time += elapsed.as_millis();
        lp += 1;
        if c == 0 {
            count += 1;
            let n = nums
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &x)| x * 10i32.pow(i as u32))
                .fold(0, |acc, x| acc + x);
            sols.insert(n);
        }
    }
    for e in sols {
        println!("{:04}", e);
    }

    println!("The number of solved numbers: {}", count);
    eprintln!("time/lp = {:?}ms ({} loop)", time as f64 / lp as f64, lp);
    count
}

pub fn solve(
    nums: &Vec<i32>,
    target: i32,
    is_perm: bool,
    show_expr: bool,
    patterns: &Vec<Vec<Symbol>>,
) -> usize {
    let nums: Vec<Ratio<i32>> = nums.iter().map(|&x| x.into()).collect_vec();
    let target: Ratio<i32> = target.into();

    let ops_vec = vec!['+', '-', '*', '/'];
    let n_len = nums.len();

    let num_set = if is_perm {
        nums.into_iter()
            .permutations(n_len)
            .collect::<BTreeSet<_>>()
    } else {
        nums.into_iter()
            .permutations(n_len)
            .take(1)
            .collect::<BTreeSet<_>>()
    };

    let operators = std::iter::repeat(ops_vec)
        .take(n_len - 1)
        .multi_cartesian_product()
        .collect_vec();

    let mut exprs = BTreeSet::new();
    let mut sol_cnt = 0;
    for perm_num in num_set {
        for ops in &operators {
            for pattern in patterns {
                let res = eval(&perm_num, &ops, pattern);

                if let Some(res) = res {
                    if res == target {
                        sol_cnt += 1;
                        if show_expr {
                            exprs.insert(eval_str(&perm_num, &ops, pattern).unwrap());
                        }
                    }
                }
            }
        }
    }
    if show_expr {
        for expr in &exprs {
            println!("{}", expr);
        }
    }

    sol_cnt
}

#[test]
fn test_solve_all() {
    let count = solve_all(10, 4, 9);
    assert_eq!(552, count);
}
#[test]
fn test_solve() {
    let patterns = Patterns::new(4);
    let c = solve(&vec![1, 2, 3, 4], 10, true, false, &patterns.get(4));
    assert!(c > 0);
}
