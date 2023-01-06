use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct AccountBracketCounting {
    pub bracket_ticket_number_counting: HashMap<BracketTicketNumber, CountTicketValue>,
}

impl Default for AccountBracketCounting {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountBracketCounting {
    pub fn new() -> Self {
        Self {
            bracket_ticket_number_counting: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bracket_ticket_number_counting.keys().len() == 0
    }
}

impl AccountBracketCounting {
    pub fn internal_unwrap_bracket_ticket_number_counting(
        &self,
        _bracket_ticket_number: &BracketTicketNumber,
    ) -> CountTicketValue {
        self.internal_get_bracket_ticket_number_counting(_bracket_ticket_number)
            .expect(ERR19_LOTTERY_NO_TICKERS_NUMBERS)
    }

    pub fn internal_get_bracket_ticket_number_counting(
        &self,
        _bracket_ticket_number: &BracketTicketNumber,
    ) -> Option<CountTicketValue> {
        self.bracket_ticket_number_counting
            .get(_bracket_ticket_number)
            .copied()
    }

    pub fn internal_get_bracket_ticket_number_counting_or_default(
        &mut self,
        _bracket_ticket_number: &BracketTicketNumber,
    ) -> CountTicketValue {
        self.internal_get_bracket_ticket_number_counting(_bracket_ticket_number)
            .unwrap_or(0)
    }

    pub fn internal_set_bracket_ticket_number_counting(
        &mut self,
        _bracket_ticket_number: &BracketTicketNumber,
    ) {
        let mut ticket_num =
            self.internal_get_bracket_ticket_number_counting_or_default(_bracket_ticket_number);

        ticket_num += 1;
        self.bracket_ticket_number_counting
            .insert(*_bracket_ticket_number, ticket_num);
    }
}
