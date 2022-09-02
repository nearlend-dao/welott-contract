use crate::*;

#[near_bindgen]
impl NearLott {
    /**
     * @notice Request randomness from a user-provided seed
     * @param _seed: seed provided by the NearLott lottery
     */
    #[private]
    pub fn get_random_number(&self, mut contractData: &ContractData) {
        self.assert_operator_calling();
        env::random_seed();

        // store last number
        /*  let rand_array = [*env::random_seed().get(0).unwrap(),*env::random_seed().get(2).unwrap(),*env::random_seed().get(3).unwrap()];
        let len:u8 = self.entries.len();
        let rand = rand_array[0] + rand_array[1] + rand_array[2];
        let winner = self.entries[(rand%len)];
        */
    }

    pub fn view_random_result(&self) -> u32 {
        0
    }
}
