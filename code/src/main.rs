 use gauss_quad::GaussLegendre;

// CONSTANTS
const N: usize = 5;
const A: f64 = 0.0;
const B: f64 = 2.0;

fn b(u_d: fn(f64) -> f64, v_d: fn(f64) -> f64, u: fn(f64) -> f64, v: fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let quad: GaussLegendre = GaussLegendre::init(4);
	quad.integrate(a, b, |x| u_d(x)*v_d(x)) - u(0.0)*v(0.0)
}

fn l(v: fn(f64) -> f64) -> f64 {
	-20.0*v(0.0)
}

fn u_i(i: usize) -> impl Fn(f64) -> f64{
    let n: f64 = N as f64;
    move |x| {
        if x > x_i(i-1) && x <= x_i(i) { 
            return n/2.0*x - i as f64 + 1.0
        }
        if x > x_i(i) && x < x_i(i+1) { 
            return -n/2.0*x + i as f64 + 1.0
        };
        return 0.0;
    }
}

fn ud_i(i: usize) -> impl Fn(f64) -> f64 {
    let n = N as f64;
    move |x| {
        if x > x_i(i-1) && x <= x_i(i) { 
            return n/2.0;
        }
        if x > x_i(i) && x < x_i(i+1) { 
            return -n/2.0;
        };
        return 0.0;
    }
}

fn x_i(i: usize) -> f64 {
    2.0*(i as f64)/(N as f64)
}


fn main() {
  println!("{}", u_i(1)(0.1)) 
}
