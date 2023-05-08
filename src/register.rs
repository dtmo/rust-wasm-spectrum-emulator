pub trait Register {
    fn get(&self) -> u8;
    fn load(&mut self, r_prime: u8);
}
