const N: usize = 10; // ilość kroków



// CAŁKI
use gauss_quad::GaussLegendre;

// ROZWIĄZANIE UKŁADU
use ndarray::prelude::*;
use ndarray_linalg::Solve;

// WYKRES
use egui::plot::{Line, Plot, PlotPoints};
use eframe::egui;

fn get_a(u_d: impl Fn(f64) -> f64, v_d: impl Fn(f64) -> f64,
	u: impl Fn(f64) -> f64, v: impl Fn(f64) -> f64,
	a: f64, b: f64) -> f64 {
		
    let quad: GaussLegendre = GaussLegendre::init(4);
    quad.integrate(a, b,
		|x| u_d(x)*v_d(x)) - u(0.)*v(0.)
}

// funkcja zwraca i-te L
fn get_l(v: impl Fn(f64) -> f64) -> f64 {
	-20.0*v(0.0)
}

// funkcja zwraca u_i
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


// funkcja zwraca (u_i)'
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

// funkcja zwraca i-tego x
fn x_i(i: f64) -> f64 {
    2.0*(i as f64)/(N as f64)
}


// DO WYKRESU
struct App {
   x: Vec<f64>,
   y: Vec<f64>
}

impl Default for App {
    fn default() -> Self {
        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        (0..1000).for_each(|i| {
             x.push(i as f64 * 0.01);
             y.push((i as f64 * 0.01).sin());
        });


        Self {
            x,
            y
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
                let points: PlotPoints = (0..self.x.len()).map(|i| {
                    [self.x[i], self.y[i]]
                }).collect();
                let line = Line::new(points);
                Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
       });
    }
}



fn plot(x: Vec<f64>, y: Vec<f64>){
    let options = eframe::NativeOptions {
//        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        ..Default::default()
    };

    let app = App {
        x: x.clone(),
        y: y.clone()
    };


    eframe::run_native(
        "Wykres",
        options,
        Box::new(|_cc| Box::new(app))
    )

}
fn print_matrix(t: Array2<f64>, b: Array1<f64>) {
	let mut row_index = 0;
	for row in t.genrows() {
		for i in row.iter() {
			print!("{:5.1}\t", i);
		}
		print!("\t{:5.1}", b[row_index]);
		row_index += 1;
		println!("");
	}
}

fn print_1d(t: Array1<f64>) {
	for i in t.iter() {
		println!("{:5.1}", i);
	}
}


// KONIEC DO WYKRESU

fn main() {
	// Macierz wypełniona zerami
    let mut a: Array2<f64> = Array2::<f64>::zeros((N+1, N+1)); 

    let n = N as f64;
	// Wypełniamy macierz zgodnie z przykładem na zajęciach
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

            a[[i, j]] = get_a(ud_i(j), ud_i(i), u_i(j), u_i(i), s, e);
        }
    }

    a[[N, N]] = 1.;

	
	// Macierz B
    let mut b: Array1<f64> = Array1::<f64>::zeros(N+1);
	
    for i in 0..N {
		b[i] = get_l(u_i(i));
    }
    b[N] = 0.;
	
	// Rozwiązujemy układ macierzy
    let res = a.solve_into(b.clone()).unwrap();
    
	// tworzymy punkty
    let x: Vec<f64> = (0..=2000).map(|x| x as f64*0.001).collect::<Vec<f64>>();
    let mut y = vec![0f64; x.len()];

    for i in 0..x.len() {
        for j in 0..res.len() {
            let e = u_i(j);
            y[i] = y[i] + res[j] * e(x[i])
        }
    }

	println!();
	print_matrix(a, b);
	println!();
	println!();
	print_1d(res);
	println!();

	// rysujemy wykres
    plot(x, y);

}
