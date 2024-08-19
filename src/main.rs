/*
 * Complete the 'activityNotifications' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY expenditure
 *  2. INTEGER d
 */

fn activityNotifications(expenditure: &[i32], d: i32) -> i32 {
    let mut notif_count = 0;
    let rest = &expenditure[d as usize..];
    for (idx, x) in rest.iter().enumerate() {
        let mut hist = expenditure[idx..d as usize + idx].to_owned();
        hist.sort_unstable();
        let hlen = hist.len();
        let median = if hlen % 2 == 0 {
            hist[hlen / 2 - 1..(hlen / 2 + 1)].iter().sum::<i32>() as f64 / 2_f64
        } else {
            hist[hlen / 2] as f64
        };
        println!(
            "({}, {}) | ({}, {}) | elem: {} >= median*2: {} = {} | {}",
            idx,
            d as usize + idx,
            hist[hlen / 2 - 1],
            hist[hlen / 2],
            x,
            median * 2_f64,
            *x as f64 >= median * 2_f64,
            if hlen % 2 == 0 { "even" } else { "odd" }
        );
        if *x as f64 >= median * 2_f64 {
            notif_count += 1;
        }
    }
    notif_count
}

fn main() {
    let contents = std::fs::read_to_string("./src/file.txt").expect("Failed to read file");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let fline = lines[0];
    let sline = lines[1];
    let fwords = fline.split(" ").collect::<Vec<&str>>();
    let count = fwords[0];
    let exp: i32 = fwords[1].parse().unwrap();
    let hist = sline.split(' ').collect::<Vec<&str>>();
    let hist_vec: Vec<i32> = hist.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let notif_count = activityNotifications(&hist_vec, exp);
    println!("{}", notif_count);
}
