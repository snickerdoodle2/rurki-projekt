use gauss_quad::GaussLegendre;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use poloto::build;
// CONSTANTS
const N: usize = 9;

fn b(u_d: impl Fn(f64) -> f64, v_d: impl Fn(f64) -> f64, u: impl Fn(f64) -> f64, v: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let quad: GaussLegendre = GaussLegendre::init(4);
    quad.integrate(a, b, |x| u_d(x)*v_d(x)) - u(0.)*v(0.)
}

fn l(v: impl Fn(f64) -> f64) -> f64 {
	-20.0*v(0.0)
}

fn u_i(i_: usize) -> impl Fn(f64) -> f64{
    let n: f64 = N as f64;
    let i = i_ as f64;
    move |x| {
        if x > x_i(i - 1.) && x <= x_i(i) { 
            return n/2.0*x - i + 1.0
        }
        if x > x_i(i) && x < x_i(i+1.) { 
            return -n/2.0*x + i + 1.0
        };
        return 0.0;
    }
}

fn ud_i(i_: usize) -> impl Fn(f64) -> f64 {
    let n = N as f64;
    let i = i_ as f64;
    move |x| {
        if x > x_i(i-1.) && x <= x_i(i) { 
            return n/2.0;
        }
        if x > x_i(i) && x < x_i(i+1.) { 
            return -n/2.0;
        };
        return 0.0;
    }
}

fn x_i(i: f64) -> f64 {
    2.0*(i as f64)/(N as f64)
}

fn plot(x: Vec<f64>, y: Vec<f64>){
    let plots = poloto::plots!(
       build::plot("u(x)").line((0..x.len()).map(|i| [x[i], y[i]]))
    );

    poloto::data(poloto::plots!(build::origin(), plots))
        .build_and_label(("Równanie transportu ciepła", "x", "y"))
        .append_to(poloto::header().dark_theme())
        .render_stdout();
}

fn main() {
    let mut a: Array2<f64> = Array2::<f64>::zeros((N+1, N+1)); 
    let n = N as f64;
    for i in 0..N {
        for j in 0..=N {
            let s: f64;
            let e: f64;
            let diff = i.abs_diff(j); 
            if diff > 1 { continue; }
            if diff == 1 {
                s = 2. * f64::max(0., f64::min(i as f64, j as f64) / n);
                e = 2. * f64::min(1., f64::max(i as f64, j as f64) / n);
            } else {
                s = 2. * f64::max(0., (i as f64 - 1.) / n);
                e = 2. * f64::min(1., (i as f64 + 1.) / n);
            }

            a[[i, j]] = b(ud_i(j), ud_i(i), u_i(j), u_i(i), s, e);
        }
    }

    a[[N, N]] = 1.;


    let mut b: Array1<f64> = Array1::<f64>::zeros(N+1);

    for i in 0..N {
        b[i] = l(u_i(i));
    }
    b[N] = 0.;

    let res = a.solve_into(b).unwrap();
    
    let x: Vec<f64> = (0..=2000).map(|x| x as f64*0.001).collect::<Vec<f64>>();
    let mut y = vec![0f64; x.len()];

    for i in 0..x.len() {
        for j in 0..res.len() {
            let e = u_i(j);
            y[i] = y[i] + res[j] * e(x[i])
        }
    }

    plot(x, y);

}
