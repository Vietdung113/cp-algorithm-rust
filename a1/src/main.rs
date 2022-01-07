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
    let mut min_avg: f64 = std::f64::MAX;
    let mut max_avg: f64 = std::f64::MIN;
    let mut avg_sum;
    let mut current_sum: i64;
    let n = scan.next::<usize>();
    let v: Vec<f64> = (0..n).map(|_| scan.next()).collect();

    for i in 0..n - 1 {
        current_sum = v[i] as i64;
        for j in i + 1..n {
            current_sum += v[j] as i64;
            avg_sum = current_sum as f64 / ((j - i + 1) as f64);
            // println!(
            //     "i: {}\tj: {}\tv[i]: {}\t v[j]: {}\tcurrentSum: {}\tavgSum: {:.3}\tlength: {}",
            //     i,
            //     j,
            //     v[i],
            //     v[j],
            //     currentSum,
            //     avgSum,
            //     j - i + 1
            // );
            if avg_sum > max_avg {
                max_avg = avg_sum;
            }
            if avg_sum < min_avg {
                min_avg = avg_sum;
            }
        }
    }
    println!("{:.3} {:.3}", min_avg, max_avg);
}
