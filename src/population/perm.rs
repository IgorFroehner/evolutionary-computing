use rand::{seq::SliceRandom, thread_rng};

use super::Individual;

#[derive(Debug, Clone)]
pub struct IntPerm(pub Vec<i64>, f64);

impl Individual for IntPerm {
    type Gene = i64;
    type RangeType = ();

    fn generate_member(dimension: u32, _: &Self::RangeType) -> Self {
        let mut rng = thread_rng();

        let mut member = (1..=dimension as i64).collect::<Vec<i64>>();
        member.shuffle(&mut rng);

        Self(member, 0.0)
    }

    fn get_vec(&self) -> &Vec<Self::Gene> {
        &self.0
    }

    fn set_fitness(&mut self, fitness: f64) {
        self.1 = fitness;
    }

    fn get_fitness(&self) -> f64 {
        self.1
    }
}
