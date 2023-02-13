use std::{hint::black_box, time::Instant};

use pasta_curves::group::{ff::Field, Group, GroupEncoding};

const RUNS: usize = 100000;

fn mark<F: Fn()>(label: &str, f: F) {
  let prev = Instant::now();
  f();
  println!("{label}: {}ms", (Instant::now() - prev).as_millis());
}

fn bench<G: Group + GroupEncoding>() {
  // Scalar add
  mark("scalar_add", || {
    let two = G::Scalar::from(2u64);
    let mut sum = G::Scalar::zero();
    for _ in 0 .. (RUNS * 100) {
      sum += two;
    }
    black_box(sum);
  });

  // Scalar mul
  mark("scalar_mul", || {
    let two = G::Scalar::from(2u64);
    let mut sum = G::Scalar::zero();
    for _ in 0 .. (RUNS * 100) {
      sum += two;
    }
    black_box(sum);
  });

  // Point double
  mark("point_dbl", || {
    let mut sum = G::generator();
    for _ in 0 .. (RUNS * 100) {
      sum = sum.double();
    }
    black_box(sum);
  });

  // Point add
  mark("point_add", || {
    let mut sum = G::identity();
    for _ in 0 .. (RUNS * 100) {
      sum = sum + G::generator();
    }
    black_box(sum);
  });

  // Point mul
  mark("point_mul", || {
    let scalar = G::Scalar::zero() - G::Scalar::one();
    let mut sum = G::generator();
    for _ in 0 .. RUNS {
      sum *= scalar;
    }
    black_box(sum);
  });

  // Ser
  mark("point_ser", || {
    for _ in 0 .. RUNS {
      black_box(G::generator().to_bytes());
    }
  });

  // Deser
  mark("point_deser", || {
    let ser = G::generator().to_bytes();
    for _ in 0 .. RUNS {
      black_box(G::from_bytes(&ser)).unwrap();
    }
  });
}

fn main() {
  println!("Ed25519. Point doubling is implemented via addition. deser will check if it's prime order and accordingly be significantly slower.");
  bench::<dalek_ff_group::EdwardsPoint>();
  println!("\r\nRistretto");
  bench::<dalek_ff_group::RistrettoPoint>();
  println!("\r\nPallas");
  bench::<pasta_curves::pallas::Point>();
  println!("\r\nVesta");
  bench::<pasta_curves::vesta::Point>();
  println!("\r\nsecp256k1");
  bench::<k256::ProjectivePoint>();

}
