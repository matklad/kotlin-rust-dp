extern crate clap;
extern crate csv;


// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---
// Time series trait
// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---

fn square_dist(xs: &[f64], xidx: usize, ys: &[f64], yidx: usize) -> f64 {
    let dif = xs[xidx] - ys[yidx];
    dif * dif
}

fn min3(x: f64, y: f64, z: f64) -> f64 {
    if x < y { if x < z { x } else { z } } else { if y < z { y } else { z } }
}

fn compute_dtw(xs: &[f64], ys: &[f64]) -> f64 {
    assert_eq!(xs.len(), ys.len());
    let n = xs.len();
    let mut curr = vec![0f64; n];
    let mut prev = vec![0f64; n];

    curr[0] = square_dist(xs, 0, ys, 0);
    for i in 1..n {
        curr[i] = curr[i - 1] + square_dist(xs, 0, ys, i);
    }

    // --- Compute DTW
    for idx_line in 1..n {
        ::std::mem::swap(&mut curr, &mut prev);
        curr[0] = prev[0] + square_dist(xs, idx_line, ys, 0);
        for idx_col in 1..n {
            let d11 = prev[idx_col - 1];
            let d01 = curr[idx_col - 1];
            let d10 = prev[idx_col];
            curr[idx_col] = min3(d11, d01, d10) + square_dist(xs, idx_line, ys, idx_col);
        }
    }
    curr[n - 1]
}


fn check_cli<'a>() -> clap::ArgMatches<'a> {
    let matches = clap::App::new("ts")
        .version("0.0")
        .about("Working with time series")
        .arg(clap::Arg::with_name("INPUT FILE")
            .required(true)
            .index(1)
            .help("Input file, must be a csv")
        ).get_matches();
    return matches;
}

fn measure_time_millis<F: FnOnce()>(f: F) -> u64 {
    let start = std::time::Instant::now();
    f();
    let elapsed = ::std::time::Instant::now() - start;
    elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000
}

fn main() {
    // --- 0: Get the command line arguments
    let matches = check_cli();
    let file = matches.value_of("INPUT FILE").unwrap();

    // --- 1: Load the CSV
    let mut rdr = csv::Reader::from_file(file).unwrap();
    let rows = rdr.records().map(|r| r.unwrap());
    let mut vec: Vec<Vec<f64>> = Vec::new();
    for row in rows {
        let mut iter = row.into_iter();
        let _class: String = iter.next().unwrap();
        let data: Vec<f64> = iter.map(|s| s.parse().unwrap()).collect();
        vec.push(data);
    }

    // --- 2: Compute sum of DTW
    let mut total_e: f64 = 0.0;

    let elapsed = measure_time_millis(|| {
        for (id, vi) in vec.iter().enumerate() {
            for vj in vec.iter().skip(id) {
                total_e += compute_dtw(vi, vj);
            }
        }
    });
    println!("{} ms", elapsed);
    println!("Total error: {}", total_e);
}
