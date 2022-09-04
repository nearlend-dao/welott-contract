// Lottery query
pub const ERR1_NOT_EXISTING_LOTTERY: &str = "E1: Lottery does not exist";
pub const ERR2_NOT_EXISTING_TICKET: &str = "E2: Ticket does not exist";
pub const ERR3_NOT_EXISTING_BRACKET: &str = "E3: Bracket does not exist";

pub const ERR4_NOT_EXISTING_LOTTERIES_PER_USER: &str = "E4: No lotteries found";

pub const ERR6_MIN_DISCOUNT_DIVISOR: &str = "E6: Must be >=min_discount_divisor";
pub const ERR7_NUMBER_TICKETS_ZERO: &str = "E7: Number of tickets must be > 0";
pub const ERR8_MIN_PRICE_MAX_PRICE: &str = "E8: minPrice must be < maxPrice";
pub const ERR10_LOTTERY_NOT_TIME_TO_START: &str = "E10: Not time to start lottery";
pub const ERR11_LOTTERY_TIME_OUT_OF_RANGE: &str = "E11: Lottery length outside of range";
pub const ERR12_LOTTERY_PRICE_OUTSIDE_LIMIT: &str = "E12: Price ticket in near - Outside of limits";
pub const ERR13_LOTTERY_DISCOUNT_DIVISOR_TOO_LOW: &str = "E13: Discount divisor too low";
pub const ERR14_LOTTERY_OVER_RANGE_REWARDS: &str = "E14: Rewards must equal 10000";
pub const ERR15_LOTTERY_OVER_TREASURY_FEE: &str = "E15: Treasury fee too high";
pub const ERR16_ATTACHED_DEPOSIT_LESS_AMOUNT: &str = "E16: Attached deposit is less than amount";
pub const ERR17_LOTTERY_IS_NOT_OPEN: &str = "E17: Lottery not open";
pub const ERR18_LOTTERY_FINAL_NUMBER_NOT_DRAWN: &str = "E18: Numbers not drawn";
pub const ERR19_LOTTERY_NO_TICKERS_NUMBERS: &str = "E19: No number tickets per lottery_id";

pub const ERR20_LOTTERY_CLAIM_NOT_SAME_LENGTH: &str = "E20: Not same length";
pub const ERR21_TICKETS__LENGTH: &str = "E21: Length must be >0";
pub const ERR22_LOTTERY_CLAIM_TOO_MANY_TICKETS: &str = "E22: Too many tickets";
pub const ERR23_LOTTERY_CLAIM_TOO_MANY_TICKETS: &str = "E23: Lottery not claimable";
pub const ERR24_BRACKETS_OUT_RANGE: &str = "E24: Bracket out of range";
pub const ERR25_LOTTERY_CLAIM_TICKET_TOO_HIGH: &str = "E25: TicketId too high";
pub const ERR26_LOTTERY_CLAIM_TICKET_TOO_LOW: &str = "E26: TicketId too low";
pub const ERR27_LOTTERY_CLAIM_TICKET_NOT_OWNER: &str = "E27: Not the owner";
pub const ERR28_LOTTERY_CLAIM_NO_PRIZE: &str = "E28: No prize for this bracket";
pub const ERR29_LOTTERY_CLAIM_BRACKET_MUST_BE_HIGHER: &str = "E29: Bracket must be higher";
pub const ERR30_LOTTERY_IS_NOT_CLOSE: &str = "E30: Lottery not close";
pub const ERR31_LOTTERY_IS_OVER: &str = "E31: Lottery is over";
pub const ERR31_TICKET_NUMBER_RANGE: &str =
    "E31: The ticket number should be in a range 1000000 - 1999999";

pub const ERR32_INSUFFICIENT_STORAGE: &str = "E32: insufficient $NEAR storage deposit";
pub const ERR33_INSUFFICIENT_MINIMUM_REQUIRES: &str = "E33: Requires minimum deposit";
pub const ERR34_RANDOM_NUMBER_INVALID: &str = "E34: Invalid random number";

pub const ERR35_CONTRACT_PAUSED: &str = "E35: contract paused";
