use crate::group::Group;
use rand::prelude::SliceRandom;
pub struct PedersonCommitmentScheme {
    pub group: Group,
}

impl PedersonCommitmentScheme {
    pub fn choose_random_generator(&self) -> i128 {
        let options = self.group.find_generator(self.group.find_divisors());
        let rand_gen = options.choose(&mut rand::thread_rng());
        *rand_gen.unwrap()
    }

    pub fn secret_key(&self) -> i128 {
        let options = self.group.elements.clone();
        let rand_key = options.choose(&mut rand::thread_rng());
        *rand_key.unwrap()
    }

    pub fn private_info(&self, key_one: i128, key_two: i128, my_number: i128) -> Vec<i128> {
        let mut private: Vec<i128> = Vec::new();
        private.push(key_one);
        private.push(key_two);
        private.push(my_number);
        private
    }

    pub fn public_info(
        &self,
        gen_one: i128,
        gen_two: i128,
        key_one: i128,
        key_two: i128,
        my_number: i128,
    ) -> Vec<i128> {
        let mut public_info: Vec<i128> = Vec::new();
        public_info.push(gen_one);
        public_info.push(gen_two);
        let c_1: i128 = gen_one.pow(key_one as u32) % self.group.subscript;
        let c_2 = gen_two.pow((key_two * my_number) as u32) % self.group.subscript;
        let c = (c_1 * c_2) % self.group.subscript;
        public_info.push(c);
        println!(
            "Here is the public information used in this
Pederson Commitment Scheme.
---
Public info: [g_1, g_2, c] = {:?}
---",
            public_info
        );
        public_info
    }
    pub fn verify_commitment(
        &self,
        public_info: Vec<i128>,
        private_info: Vec<i128>,
        ring_size: i128,
    ) {
        println!(
            "---
To verify I was honest:
{:?} was the public info [g_1, g_2, c].
{:?} was the private info [s_1, s_2, m],
where m was my number.",
            public_info, private_info
        );
        let gen_one = public_info[0];
        let gen_two = public_info[1];
        let known_commit = public_info[2];
        let key_one = private_info[0];
        let key_two = private_info[1];
        let my_number = private_info[2];
        let c_1 = gen_one.pow(key_one as u32) % ring_size;
        let c_2 = gen_two.pow((key_two * my_number) as u32) % ring_size;
        let calculated_commit = (c_1 * c_2) % ring_size;
        assert_eq!(
            known_commit, calculated_commit,
            "You got me! My commitment wasn't truthful"
        );
    }
}
