use std::collections::VecDeque;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.lines().next().unwrap().split(',').map(|n| n.parse::<u8>().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn run_80_days(individuals: &[u8]) -> u64 {
    const DAYS: usize = 80;
    let mut population = LanternFishPopulation::from(individuals);
    population.simulate_days(DAYS);
    population.get_total_population()
}

#[aoc(day6, part2)]
pub fn run_256_days(individuals: &[u8]) -> u64 {
    const DAYS: usize = 256;
    let mut population = LanternFishPopulation::from(individuals);
    population.simulate_days(DAYS);
    population.get_total_population()
}

#[derive(Debug)]
pub struct LanternFishPopulation {
    day: usize,
    population_counters: VecDeque<u64>,
}

impl From<&[u8]> for LanternFishPopulation {
    fn from(individuals: &[u8]) -> Self {
        let mut pop: [u64; 9] = [0; 9];
        for i in individuals {
            pop[*i as usize] += 1;
        }
        let population_counters = VecDeque::from(pop);
        LanternFishPopulation { day: 0, population_counters }
    }
}

impl LanternFishPopulation {
    fn simulate_days(&mut self, days: usize) {
        for _day in 0..days {
            self.simulate_day_progression();
        }
    }

    fn simulate_day_progression(&mut self) {
        let new_parents = self.population_counters.pop_front().unwrap();
        self.population_counters[6] += new_parents;
        self.population_counters.push_back(new_parents);
        self.day += 1;
    }

    fn get_total_population(&self) -> u64 {
        self.population_counters.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2021/day6.txt");

    #[test]
    fn input_known_answer() {
        let result = run_80_days(&input_generator(INPUT));

        assert_eq!(result, 350917);
    }

    #[test]
    fn input_known_answer_p2() {
        let result = run_256_days(&input_generator(INPUT));

        assert_eq!(result, 1592918715629);}

    const EXAMPLE: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn example_has_26_fish_after_18_days() {
        let mut population = LanternFishPopulation::from(EXAMPLE.as_slice());
        population.simulate_days(18);

        let count = population.get_total_population();

        assert_eq!(count, 26);
    }

    #[test]
    fn example_has_5934_fish_after_80_days() {
        let mut population = LanternFishPopulation::from(EXAMPLE.as_slice());
        population.simulate_days(80);

        let count = population.get_total_population();

        assert_eq!(count, 5934);
    }

    #[test]
    fn example_has_26984457539_fish_after_256_days() {
        let mut population = LanternFishPopulation::from(EXAMPLE.as_slice());
        population.simulate_days(256);

        let count = population.get_total_population();

        assert_eq!(count, 26984457539);
    }
}
