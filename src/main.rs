use solve_tenpuzzle::new_solver;
use solve_tenpuzzle::pattern;
use solve_tenpuzzle::solver;
use std::env;
// use solve_tenpuzzle::algorithm::Permutation;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        let guard = pprof::ProfilerGuard::new(300).unwrap();
        let lim = 9;
        let clock = std::time::Instant::now();
        let ans = solver::solve_all(lim);
        let t = clock.elapsed();
        eprintln!("t1 = {:?}", t);
        let clock = std::time::Instant::now();
        let _ans = new_solver::solve_all(10, 4, lim as isize);
        let t = clock.elapsed();
        eprintln!("t2 = {:?}", t);
        // println!(
        //     "How many numbers can be solved?? : {}  : ({} puzzle)",
        //     ans, 10
        // );
        if let Ok(report) = guard.report().build() {
            let file = std::fs::File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
        };
    } else {
        for i in 1..args.len() {
            let tmp = &args[i].clone();
            let mut num_vec: Vec<f32> = tmp
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
            let nums: Vec<isize> = tmp
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();

            // let mut ans = solver::solve(&mut num_vec);
            // ans.sort();
            // ans.dedup();
            // for e in &ans {
            //     println!("{}", e);
            // }
            let e = pattern::Patterns::new(nums.len());
            println!("------------------");
            let res = new_solver::solve(&nums, 10, true, true, &e.get(nums.len()));
            eprintln!("res = {:?}", res);
        }
    }
}
