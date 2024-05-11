use rand::{rngs::StdRng, Rng, SeedableRng};

type Gene = f64;
type Individual = Vec<Gene>;
type Evaluation = Vec<f64>;

#[derive(Debug)]
pub struct EAServer {
    individuals: Vec<Individual>,
    evaluations: Vec<Evaluation>,
    rng: StdRng, // random generator
    cnt_no_update: usize,
}

impl EAServer {
    pub fn new(indis: Vec<Individual>, evals: Vec<Evaluation>, rng: StdRng) -> Self {
        EAServer {
            individuals: indis,
            evaluations: evals,
            rng,
            cnt_no_update: 0,
        }
    }
    pub fn new_with_random(indi_num: usize, indi_len: usize, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let individuals = (0..indi_num)
            .map(|_| (0..indi_len).map(|_| rng.gen()).collect())
            .collect();

        EAServer {
            individuals: individuals,
            evaluations: vec![Vec::default(); indi_num],
            rng,
            cnt_no_update: 0,
        }
    }
    pub fn get_cnt_no_update(&self) -> usize {
        self.cnt_no_update
    }
    pub fn get_individuals(&self) -> &Vec<Individual> {
        &self.individuals
    }
    pub fn set_individual(&mut self, target_i: usize, indi: Individual) {
        self.individuals[target_i] = indi;
    }
    pub fn get_evaluations(&self) -> &Vec<Individual> {
        &self.evaluations
    }
    pub fn set_evaluations(&mut self, target_i: usize, eval: Evaluation) {
        self.evaluations[target_i] = eval;
    }
    pub fn get_gene_len(&self) -> usize {
        self.individuals[0].len()
    }
    pub fn get_best(&self) -> (&Individual, &Evaluation) {
        let (i, eval) = self
            .get_evaluations()
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.partial_cmp(y.1).unwrap())
            .unwrap();
        (&self.get_individuals()[i], eval)
    }

    /// mutate and cross
    pub fn de_rand_x_bin(
        &mut self,
        target_i: usize,
        vector_num: usize,
        f_scale: f64,
        own_ratio: f64,
    ) -> Individual {
        debug_assert!(0.0 <= own_ratio && own_ratio < 1.0);

        let factor_num = vector_num * 2 + 1;
        debug_assert!(factor_num < self.get_individuals().len());
        let mut factor_indexes = Vec::with_capacity(factor_num);
        let mut chosen_list = vec![false; self.individuals.len()];
        let mut cnt = 0;
        while cnt != factor_num {
            let i = self.rng.gen_range(0..self.individuals.len());
            if chosen_list[i] {
                continue;
            }
            chosen_list[i] = true;
            cnt += 1;
            factor_indexes.push(i);
        }

        let indi_len = self.get_gene_len();
        let mut indi = Vec::with_capacity(indi_len);

        for i in 0..indi_len {
            let mut gene = self.get_individuals()[factor_indexes[0]][i];
            for j in 0..vector_num {
                let gene1 = self.get_individuals()[factor_indexes[1 + 2 * j]][i];
                let gene2 = self.get_individuals()[factor_indexes[1 + 2 * j + 1]][i];

                gene += f_scale * (gene1 - gene2);
            }
            // [0.0, 1.0]がパラメータの範囲なため、超えていた場合は範囲内に収まるように修正する
            // かつ、多様性を失わないように
            if gene > 1.0 {
                gene = 0.8 + self.rng.gen::<f64>() * 0.2;
            } else if gene < 0.0 {
                gene = self.rng.gen::<f64>() * 0.2;
            }
            indi.push(gene);
        }

        let must_choose_another = self.rng.gen_range(0..self.get_gene_len());

        for i in 0..self.get_gene_len() {
            if self.rng.gen::<f64>() < own_ratio || i == must_choose_another {
                // another
            } else {
                // own
                indi[i] = self.get_individuals()[target_i][i];
            }
        }

        indi
    }

    /// If new_eval > old_eval, update and return true.
    pub fn may_update(&mut self, target_i: usize, indi: Individual, new_eval: Evaluation) -> bool {
        let old_eval = &self.get_evaluations()[target_i];
        if new_eval > *old_eval || old_eval.len() == 0 {
            self.set_individual(target_i, indi);
            self.set_evaluations(target_i, new_eval);
            self.cnt_no_update = 0;
            true
        } else {
            self.cnt_no_update += 1;
            false
        }
    }
}
