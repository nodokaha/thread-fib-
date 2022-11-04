use std::thread;
use std::env;
use gmp::mpz::Mpz;

fn main() {
    // let n = 10;
    let mut f1: Mpz = From::from(0);
    let mut f2: Mpz = From::from(1);
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let fib_thread = thread::spawn(move || {
	for n in 0..n{
	    // let s = f1.clone();
	    f1 = f1.clone()+f2.clone();
	    // println!("{} {} {} {}", n, f1, f2, f1+f2);
	    f2 = f1.clone()+f2.clone();
	    println!("{} {} {}", n, f1, f2);
	}
	    
    });
    fib_thread.join().unwrap();
}
