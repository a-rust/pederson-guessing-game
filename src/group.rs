pub struct Group {
    pub elements: Vec<i128>,
    pub subscript: i128,
}

//Implement the theorem for checking if an element is a generator of Z_p^*
impl Group {
    pub fn find_divisors(&self) -> Vec<i128> {
        let mut known_divisors: Vec<i128> = Vec::new();
        for i in 2..self.subscript - 2 {
            if (self.subscript - 1) % i == 0 {
                known_divisors.push(i);
            }
        }
        // println!(
        //     "strictly smaller divisors of {}, {:?}",
        //     self.subscript - 1,
        //     known_divisors
        // );
        known_divisors
    }

    pub fn find_generator(&self, divisors: Vec<i128>) -> Vec<i128> {
        let mut known_generators: Vec<i128> = Vec::new();
        let mut counter = 0;
        for i in self.elements.clone() {
            for j in divisors.clone() {
                //println!("{},{},{}", i, j, (i.pow(j as u32)) % self.subscript);
                if i.pow(j as u32) % self.subscript == 1 {
                    counter += 1;
                }
            }
            if counter < 1 {
                known_generators.push(i)
            }
            counter = 0;
        }
        // println!("generators {:?}", known_generators);
        known_generators
    }
}
