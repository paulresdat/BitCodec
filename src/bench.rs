pub fn bench<F>(f: F) -> BenchDuration
    where F: Fn() {
    
    let mut bench = BenchDuration { nano: 0, micro: 0f64, milli: 0f64 };
    let mut times: Vec<u64> = Vec::new();

    println!("Warming up");
    // warmup
    for _ in 0..10 {
        for _ in 0..1000 {
            //get_bits(&v, 0, 1);
            f();
        }
    }
    
    for _ in 0..10 {
        let before = Instant::now();
        for _ in 0..1000 {
            //get_bits(&v, 0, 1);
            f();
        }
        let duration = before.elapsed().as_nanos() / 1000;
        times.push(duration as u64);
    }

    let mut tot: u128 = 0;
    let ln = times.len();
    for s in times {
        tot += s as u128;
    }

    let total_nano = tot / (ln as u128);
    let micro = (total_nano as f64) / 1000f64;

    bench.nano = total_nano;
    bench.micro = micro;
    if micro > 1000f64 {
        let milli = micro / 1000f64;
        bench.milli = milli;
    }

    bench
}

pub struct BenchDuration {
    nano: u128,
    micro: f64,
    milli: f64,
}

impl fmt::Display for BenchDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.milli != 0f64 {
            write!(f, "Milliseconds: {}", self.milli)
        }
        else if self.micro != 0f64 {
            write!(f, "Nano seconds: {}", self.nano)
        }
        else if self.nano != 0 {
            write!(f, "Nano seconds: {}", self.nano)
        }
        else {
            write!(f, "{}", 0)
        }
    }
}

