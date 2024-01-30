use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("Arctan of an angle")
        .version("0.1.0")
        .author("MrBiTs <mrbits@mrbits.com.br>")
        .about("Compute the arctan of an angle")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("distance")
                .short('d')
                .long("dist")
                .takes_value(true)
                .help("Distance between two objects"),
        )
        .arg(
            Arg::with_name("radius")
                .short('r')
                .long("rad")
                .takes_value(true)
                .help("Distance between two objects"),
        )
        .get_matches();

    let distance: f64 = matches.value_of("distance").unwrap().parse().unwrap();
    let radius: f64 = matches.value_of("radius").unwrap().parse().unwrap();
    let diameter = radius * 2_f64;

    let angle = diameter / distance;

    // Número de termos na série de Taylor
    let iteractions = 20;

    // Calcula o arco-tangente usando a série de Taylor
    let (arc, result) = taylor_series_arctan(angle, iteractions);
    let builtin_atan: f64 = angle.atan();

    // Exibe o resultado
    println!("Resultado usando método 1:  {}", result);
    println!("Resultado usando método 2:  {}", arc);
    println!("Resultado com ATAN builtin: {}", builtin_atan.to_degrees());
}

// Função para calcular o arco-tangente usando a série de Taylor
fn taylor_series_arctan(x: f64, iteractions: usize) -> (f64, f64) {
    let pi = 3.14159265358979323846264338327950288419716939937510f64;
    let mut result = 0.0;
    let mut sign = 1.0;
    let mut arc = 0.0;

    for n in 0..iteractions {
        let term = sign * x.powi((2 * n + 1) as i32) / (2 * n + 1) as f64;
        result += term;
        sign = -sign;
    }

    for i in (1..=iteractions).step_by(4) {
        let num = i as f64;
        let nm2 = num + 2.0;
        arc = arc + (x.powf(num) / num) - (x.powf(nm2) / nm2);
    }

    result = result * 180_f64 / pi;
    arc = arc * 180_f64 / pi;

    (arc, result)
}
