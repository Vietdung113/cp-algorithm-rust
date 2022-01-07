use std::collections::HashMap;
use std::io::stdin;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let n: usize = scan.next::<usize>();
    let mut v: Vec<i64> = Vec::with_capacity(n);
    let mut count = 1;
    let mut tmp;
    let mut scores = HashMap::new();
    for _ in 0..n{
        tmp= scan.next::<i64>();
        if !scores.contains_key(&tmp){
            scores.insert(tmp,1);
        }
        v.push(tmp);
    };
    v.sort();
    println!("{}",scores.len());
    let mut current_number = v[0];
    for i in 1..n {
        if v[i] == current_number {
            count += 1;
        } else {
            // scores.insert(current_number, count);
            println!("{} {}", current_number, count);
            current_number = v[i];
            count = 1;
        }
    }
    println!("{} {}", current_number, count);

    // scores.insert(current_number, count);
    // println!("{}", scores.keys().len());
    // for key in scores.keys().sort(){
    //     println!("{} {}", key,scores[key]);
    // }

}
