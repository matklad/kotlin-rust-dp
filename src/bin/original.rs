extern crate clap;      // Command line parsing
extern crate csv;       // CSV loading

use std::cmp::PartialEq;


// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---
// Time series trait
// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---

struct TimeSerie {
    class: String,
    data: Vec<f64>,
    ptr: *const f64
}

impl TimeSerie {
    #[inline(always)]
    fn length(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    fn at(&self, idx: isize) -> f64 { unsafe { *self.ptr.offset(idx) } }

    #[inline(always)]
    fn square_dist(&self, self_idx: isize, other_idx: isize, other: &Self) -> f64 {
        let dif = self.at(self_idx) - other.at(other_idx);
        dif * dif
    }

    fn compute_dtw(&self, other: &Self, m: *mut f64) -> f64 {
        let dim = self.length() as isize;
        unsafe {
            // --- Init 0,0
            *m = self.square_dist(0, 0, other);

            // --- Init the two "axis"
            // --- --- first line, along columns (0, ..): self
            // --- --- first column, along lines (.., 0): other
            for x in 1..dim as isize {
                *m.offset(x) = *m.offset(x - 1) + self.square_dist(0, x, other);               // First line
                *m.offset(dim * x) = *m.offset(dim * (x - 1)) + self.square_dist(x, 0, other);     // First col
            }

            // --- Compute DTW
            for idx_col in 1..dim as isize {
                for idx_line in 1..dim as isize {
                    *m.offset(dim * idx_line + idx_col) = {
                        // Compute ancestors
                        let d11 = *m.offset(dim * (idx_line - 1) + idx_col - 1);
                        let d01 = *m.offset(dim * (idx_line) + idx_col - 1);
                        let d10 = *m.offset(dim * (idx_line - 1) + idx_col);
                        // Take the smallest ancestor and add the current distance
                        (if d11 < d01 { if d11 < d10 { d11 } else { d10 } } else { if d01 < d10 { d01 } else { d10 } }) + self.square_dist(idx_line, idx_col, other)
                        // The next line actually call cmath
                        // d11.min(d01).min(d10) + self.square_dist(idx_line, idx_col, other)
                    };
                }
            }
            let last = dim - 1 as isize;
            (*m.offset(dim * last + last)) //.sqrt()
        }
    }

    // --- --- --- static functions
    fn new(class: String, data: Vec<f64>) -> TimeSerie { TimeSerie { class: class, ptr: data.as_ptr(), data: data } }
}


// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---
// Command line building
// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---
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


// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---
// Main app
// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---
fn main() {
    // --- 0: Get the command line arguments
    let matches = check_cli();
    let file = matches.value_of("INPUT FILE").unwrap();

    // --- 1: Load the CSV
    let mut rdr = csv::Reader::from_file(file).unwrap();
    let rows = rdr.records().map(|r| r.unwrap());
    let mut vec: Vec<TimeSerie> = Vec::new();
    for row in rows {
        let mut iter = row.into_iter();
        let class: String = iter.next().unwrap();
        let data: Vec<f64> = iter.map(|s| s.parse().unwrap()).collect();
        vec.push(TimeSerie::new(class, data));
    }

    // --- 2: Compute sum of DTW
    let mut total_e: f64 = 0.0;
    let ts_size = vec[0].length();
    let working_area = vec![0 as f64; ts_size * ts_size].as_mut_ptr();

    let now = std::time::SystemTime::now();
    for (id, vi) in vec.iter().enumerate() {
        for vj in vec.iter().skip(id) {
            total_e += vi.compute_dtw(vj, working_area);
        }
    }
    match now.elapsed() {
        Ok(elapsed) => { println!("{} s", elapsed.as_secs()); }
        Err(_) => { () }
    }

    println!("Total error: {}", total_e);
}
