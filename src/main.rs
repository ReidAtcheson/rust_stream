use rayon::prelude::*;

pub fn stream_triad(a : &mut [f64], b : &[f64], c : &[f64], scalar : f64){
    let m = a.len();
    if m>10000{
        let (a0,a1) = a.split_at_mut(m/2);
        let (b0,b1) = b.split_at(m/2);
        let (c0,c1) = c.split_at(m/2);
        rayon::join(
            ||stream_triad(a0,b0,c0,scalar),
            ||stream_triad(a1,b1,c1,scalar));
    }
    else{
        for ((ai,bi),ci) in a.iter_mut().zip(b.iter()).zip(c.iter()){
            *ai = bi + scalar*ci;
        }
    }
}

pub fn percentiles(a : &Vec<f64>) -> (f64,f64,f64,f64,f64,f64) {
    let mut b = a.clone();
    b.sort_by(|x,y|x.partial_cmp(y).unwrap());
    let m = a.len();
    let i01 =  1*m / 100;
    let i05 =  5*m / 100;
    let i50 = 50*m / 100;
    let i95 = 95*m / 100;
    let i99 = 99*m / 100;
    (b[i01],b[i05],b[i50],b[i95],b[i99],*b.last().unwrap())
}

fn main() {
    use std::time::{Instant};
    use rand::distributions::{Distribution,Uniform};
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    let maxsize = 10000000;
    let nbytes = std::mem::size_of::<f64>() * maxsize;
    let nreads = 2 * nbytes;
    let nwrites = 1 * nbytes;
    let nsamples = 1000;
    let seed : u64 = 19140148;
    let mut times = Vec::<f64>::new();
    for _ in 0..nsamples{
        let vals_dist = Uniform::new(-2.0,2.0);
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let m = maxsize;
        let mut a : Vec<f64> = (0..m).map(|_x|vals_dist.sample(&mut rng)).collect();
        let b : Vec<f64> = (0..m).map(|_x|vals_dist.sample(&mut rng)).collect();
        let c : Vec<f64> = (0..m).map(|_x|vals_dist.sample(&mut rng)).collect();
        let scalar : f64 = vals_dist.sample(&mut rng);
        let start = Instant::now();
        stream_triad(&mut a,&b,&c,scalar);
        let nreads_gb = (nreads as f64) / (1024.0 * 1024.0 * 1024.0);
        let nwrites_gb = (nwrites as f64) / (1024.0 * 1024.0 * 1024.0);
        times.push(  (nreads_gb + nwrites_gb) / start.elapsed().as_secs_f64() );
    }    
    let (p01,p05,p50,p95,p99,max) = percentiles(&times);
    println!("p01: {}, p05: {}, p50: {}, p95: {}, p99: {}, max: {}",p01,p05,p50,p95,p99,max);
}


