use std::env;
mod solver;
mod util;
// use solve_tenpuzzle::algorithm::Permutation;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        let ans = solver::solve_all(13);
        println!(
            "How many numbers can be solved?? : {}  : ({} puzzle)",
            ans, 10.0
        );
    } else {
        for i in 1..args.len() {
            let tmp = &args[i].clone();
            let mut num_vec: Vec<f32> = tmp
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();

            let mut ans = solver::solve(&mut num_vec);
            ans.sort();
            ans.dedup();
            for e in &ans {
                println!("{}", e);
            }
        }
    }
}
