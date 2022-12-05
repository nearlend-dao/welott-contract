![Welott](https://i.imgur.com/2u1ruLU.png)

# Welott - NEAR based Lottery
Welott is a secure online lottery system, which employs smart contracts, NEAR VRF Solution and $NEAR to distribute tickets & prizes.

App is live: [https://welott.nearlenddao.com](https://welott.nearlenddao.com/)  

## Welott App Overview

### Inspiration
The lottery business has been established for a long time and accepted in many countries with different models and scopes. Even so, the organization of lottery games all has the same purpose: Meeting the entertainment needs of a part of the population, Creating jobs; Attracting investment capital; Promoting economic growth, and Contributing to social activities.

Lottery industry revenue in the Asia Pacific region has reached 21 billion USD. In Vietnam, for example, the revenue from the lottery business has steadily increased over the years. Budget reports for the past five years show that the total revenue from the lottery business is over $ 1.0 BILLION per year. _As of 2021, this number is around $3.5 billion; in the first half of 2022, this number reached $2.4 BILLION. Compared with other sources of revenue, such as crude oil in the first six months of 2022 is just more than 1.2 billion USD, the market size of the lottery business is remarkable.

However, business activities also exist many problems and barriers that cannot be solved, such as:

TRANSPARENCY: The issue of transparency from lottery companies and independent auditing firms in financial, business and investment activities has always been a controversial topic in society;

HIGH COST: The lottery industry is mainly operated by the traditional method, selling goods through many intermediaries, leading to high operating costs and child labor abuse;

LOTTERY FRAUD: Fraudulent activities to limit or prevent a player's ability to win; or Criminals using fake or altered lottery tickets to claim prizes;

PRIVACY: When players face problems receiving bonuses because they must go through complicated verification procedures or their identities cannot be kept secure, it leads to negative consequences later.

With the limitations mentioned above, the lottery industry is currently facing two options: one is to change to transform, and the other is to retreat. The only way to solve this problem is to apply "TECHNOLOGY." Experts also confirmed that traditional sales channels would gradually decrease soon. Instead, digital transformation and online ticketing will be an inevitable trend.

## Benefits for NEAR
 
UTILITY: Welott uses $NEAR to distribute prizes and tickets, increasing the utility of the $NEAR token and generating a significant amount of TVL for the Ecosystem in the long term.

OPEN-SOURCE: Welott is an open-source project built by developers from the Nearlend DAO project with proven qualifications and capabilities. The project will bring much value to the next developers who want to learn and build new applications on top of the NEAR Blockchain and the RUST language.

MISSING PUZZLE: Although a few lottery projects have been developed on the NEAR Protocol, they still need to be completed and offer a whole user experience. So, for now, the lottery is still a missing puzzle in the NEAR Ecosystem.


# How we built it

General Rules

Welott applies a basic lottery model with six digits. Players can choose the number they want instead of choosing predefined numbers from the publisher. To increase convenience, Welott also allows players to choose numbers at random.

The total value of the Prize Pool corresponds to the number of tickets purchased by the player and is distributed into corresponding frames. There are all 06 prize frames from 1 to 6 (or corresponding to 2% ;3%; 5%; 10%; 20%; 40%). In addition, 20% of the Pool's value will be provisioned in each round. Alternatively, at least 20% will be left for the next round in addition to the amount paid to the player.

Method of choosing winning numbers

Welott will integrate NEAR VRF (Verifiable Random Function) to choose the winning number.

https://docs.rs/near-sdk/latest/near_sdk/env/fn.random_seed.html#

Initially, NEAR VRF (Verifiable Random Function) is a provably fair and verifiable random number generator (RNG) that enables smart contracts to access random values without compromising security or usability.

Pick 10 digits random from the VRF number and modulo operator (https://en.wikipedia.org/wiki/Modulo_operation) by 9 to get the position random. Then using those positions to select digit by digit from the original VRF
The 10 numbers selected will combine into a string and convert to a number. We can call the name is fn1
The last step we will select the final number by using the math: 1,000,000 - (1,000,000 + (fn1 modulo 1,000,000))
For example:

The env:random_seed() number is: a = 9854264523

1) Ten positions random is [0,8,5,4,2,6,4,5,2,3]

2) A number generated will be: 9262542654. It's corresponding with:

a[0] = 9 ; a[8] = 2 ; a[5] = 6 ; a[4] = 2 ; a[2] = 5;

a[6] = 4 ; a[4] = 2 ; a[5] = 6 ; a[2] = 5 ; a[3] = 4

3) The final number is 5 4 2 6 5 4

It is calculated by the math: 1,000,000 - (1,000,000 + (9262542654 % 1,000,000))

## Add your files

- How to check out Welott:

```
cd your_path_folder
git clone https://gitlab.com/nearlend-protocol/welott.git
cd welott
```

## Unit test and simulation test

- Unit test:
```
cd welott/contract
cargo test -- --nocapture
```

- Simulation test:
```
cd welott/tests
cargo test -- --nocapture
```

## Run bash scripts

```
cd scripts
```

Deploy contract: 
```
sh ./0-deploy.sh
```

Update Welott contract:
```
sh ./1-config.sh
```

Start lottery, close lottery, draw final number:
```
sh ./2-round.sh
```

Views info lotteries
```
sh ./9-view.sh
```

## Others products of Nearlend DAO:

Testnet of Lending Protocol: https://www.testnet.nearlenddao.com/

Lang Biang NFT: https://nearlenddao.com/lang-biang-club

