use std::env;
mod algorithm;
// use solve_tenpuzzle::algorithm::Permutation;
const ANS : f32 = 10.0;

fn main() {
    let args : Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        let ans = solve_all();
        println!("How many numbers can be solved?? : {}  : ({} puzzle)",ans, ANS);
    }else{
        for i in 1..args.len() {
            let tmp = &args[i].clone();
            let mut num_vec : Vec<f32> = tmp.chars()
                                        .map(|x| x.to_string().parse().unwrap())
                                        .collect();
            
            let mut ans = solve(&mut num_vec);
            ans.sort();
            ans.dedup();
            for e in &ans{
                println!("{}",e);
            }
        }
    }
}

fn solve(v : &mut Vec<f32>) -> Vec<String>{
    let mut perm = algorithm::Permutation::new(4);
    let ops = make_op_vec();
    let mut res = Vec::new();

    loop{
        let x = perm.next();
        let id;

        match x {
            Some(x) => id = x,
            None => break,
        }

        let a : f32 = v[id[0]];
        let b : f32 = v[id[1]];
        let c : f32 = v[id[2]];
        let d : f32 = v[id[3]];

        for op in &ops{
            for i in 0..5 {
                if i == 0{
                    let c1 = match calc(a ,b, op[0]){
                        Some(x) => x,
                        None => continue
                    };

                    let c2 = match calc(c1,c,op[1]){
                        Some(x) => x,
                        None => continue
                    };

                    let ans = match calc(c2,d,op[2]){
                        Some(x) => x,
                        None => continue
                    };

                    

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("(({}{}{}){}{}){}{}",a,&op[0],b,&op[1],c,&op[2],d);
                        res.push(s);
                    }
                }else if i == 1{
                    let c1 = match calc(b ,c, op[1]){
                        Some(x) => x,
                        None => continue
                    };

                    let c2 = match calc(a,c1,op[0]){
                        Some(x) => x,
                        None => continue
                    };

                    let ans = match calc(c2,d,op[2]){
                        Some(x) => x,
                        None => continue
                    };
                    

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("({}{}({}{}{})){}{}",a,&op[0],b,&op[1],c,&op[2],d);
                        res.push(s);
                    }
                }else if i == 2{
                    let c1 = match calc(c ,d, op[2]){
                        Some(x) => x,
                        None => continue
                    };
                    

                    let c2 = match calc(b,c1,op[1]){
                        Some(x) => x,
                        None => continue
                    };
                    

                    let ans = match calc(a,c2,op[0]){
                        Some(x) => x,
                        None => continue
                    };
                    

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("{}{}({}{}({}{}{}))",a,&op[0],b,&op[1],c,&op[2],d);
                        res.push(s);
                    }
                }else if i == 3{
                    let c1 = match calc(b ,c, op[1]){
                        Some(x) => x,
                        None => continue
                    };

                    let c2 = match calc(c1,d,op[2]){
                        Some(x) => x,
                        None => continue
                    };

                    let ans = match calc(a,c2,op[0]){
                        Some(x) => x,
                        None => continue
                    };
                    

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("{}{}(({}{}{}){}{})",a,&op[0],b,&op[1],c,&op[2],d);
                        res.push(s);
                    }
                }else if i == 4{
                    let c1 = match calc(a ,b, op[0]){
                        Some(x) => x,
                        None => continue
                    };

                    let c2 = match calc(c,d,op[2]){
                        Some(x) => x,
                        None => continue
                    };

                    let ans = match calc(c1,c2,op[1]){
                        Some(x) => x,
                        None => continue
                    };
                    
                    

                    if (ans - ANS).abs() <= 0.001 {
                        let s = format!("({}{}{}){}({}{}{})",a,&op[0],b,&op[1],c,&op[2],d);
                        res.push(s);
                    }
                }
            }
        }
        
    }
    res
}

fn calc(x: f32, y: f32, op: char) -> Option<f32>{
    if op == '+'{
        Some(x + y)
    }else if op == '-'{
        Some(x - y)
    }else if op == '*'{
        Some(x * y)
    }else if y != 0.0{
        Some(x / y)
    }else{
        None
    }
}

fn make_op_vec() -> Vec<Vec<char>>{
    let mut res : Vec<Vec<char>> = Vec::new();
    let op = vec!['+','-','*','/'];

    for e in &op {
        for ee in &op{
            for eee in &op{
                let mut tmp : Vec<char> = Vec::new();
                tmp.push(e.clone());
                tmp.push(ee.clone());
                tmp.push(eee.clone());

                res.push(tmp);
            }
        }
    }

    res
}



fn solve_all() -> u32 {
    let mut ans : u32 = 0;
    let mut v :Vec<f32> = Vec::new();

    for i in 0..10{
        for j in i..10 {
            for k in j..10 {
                for l in k..10 {
                    v.clear();
                    v.push(i as f32);
                    v.push(j as f32);
                    v.push(k as f32);
                    v.push(l as f32);

                    let res = solve(&mut v);
                    //println!("{:?}",v);

                    if res.len() >= 1 {ans += 1;}
                }
            }
        }
    }
    ans
}
