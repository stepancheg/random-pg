extern crate pcg_rand;
extern crate rand;

use rand::Rng;
use rand::SeedableRng;

use pcg_rand::Pcg32;

fn rand_el<A, I>(iter: I) -> Option<A>
    where
        I : Iterator<Item=A>,
        A : Default + Copy,
{
    let seed: [u64; 2] = rand::thread_rng().gen();
    let mut rng = Pcg32::from_seed(seed);

    const BUF_SIZE: usize = 8 << 10;

    let mut buf: [A; BUF_SIZE] = [Default::default(); BUF_SIZE];

    let mut r = None;

    let mut size = 0;
    for (i, v) in iter.enumerate() {
        buf[i % BUF_SIZE] = v;
        if i > 0 && i % BUF_SIZE == 0 {
            let j: usize = rng.gen::<usize>() % i;
            if j < BUF_SIZE {
                r = Some(buf[j]);
            }
        }
        size = i + 1;
    }

    if size % BUF_SIZE != 0 {
        let j: usize = rng.gen_range(0, size);
        if j < size % BUF_SIZE {
            return Some(buf[j]);
        }
    }

    r
}

fn main() {
    let pg: u64 = 1_000_000_000;
    println!("{:?}", rand_el(0..pg));
}
