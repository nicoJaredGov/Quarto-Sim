pub struct GeneticMinmaxConfig {
    pub search_depth: u8,
    pub max_generations: u8,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub initial_population_size: u16,
    pub max_population_size: u16,
}

impl Default for GeneticMinmaxConfig {
    fn default() -> Self {
        Self {
            search_depth: 3,
            max_generations: 2,
            crossover_rate: 0.4,
            mutation_rate: 0.8,
            initial_population_size: 3,
            max_population_size: 10,
        }
    }
}
