use evolutionary_computing::{
    crossover::BinCrossover,
    evolution_builder::EvolutionBuilder,
    io::{read_config, Config},
    mutation::BinMutation,
    population::GeneCod,
    selection::RouletteSelection,
    utils::{convert_bin, within_range},
};
use fitness_max::MathFitnessMax;

mod fitness_max;
mod fitness_min;

const L: f64 = 16.;
const RANGE: (f64, f64) = (-2., 2.);
const C_MAX: f64 = 2.;
const C_MIN: f64 = -4.;

pub fn f(x: f64) -> f64 {
    (20. * x).cos() - (x.abs() / 2.) + (x.powf(3.) / 4.)
}

fn main() {
    let file_name = "examples/math_func/Config.toml";

    let Config {
        population_size,
        dimension,
        gene_cod,
        runs,
        ..
    } = read_config(file_name).expect("Failed to read config file");

    if let GeneCod::Bin = gene_cod {
        let max_fitness = MathFitnessMax;

        let mut evolution =
            EvolutionBuilder::new(population_size, dimension, gene_cod, ())
                .set_fitness(max_fitness)
                .set_selection(RouletteSelection::default())
                .set_crossover(BinCrossover::default())
                .set_mutation(BinMutation::default())
                .set_title("Math Function".to_string())
                .set_stop_condition(move |_, iterations| iterations >= runs as u32)
                .build()
                .unwrap();

        evolution.run();

        let best_found = evolution.current_best();

        println!("Best found: {:?}", best_found);
        let x = within_range(RANGE, L, convert_bin(&best_found.0));
        println!("x: {}", x);
        println!("f(x): {}", f(x));

        evolution.plot_chart().unwrap();
    }
}
