# Nearlott

This contract was initialized with Near-sdk version 4.0.0-pre.7. Made by @mitsori - 2022

Nettlet is inspried by Pancake Swap Lottery.

# Quick Start

If you haven't installed dependencies during setup:

    npm run deps-install

Build and deploy your contract to TestNet with a temporary dev account:

    npm run deploy

Test your contract:

    npm test

If you have a frontend, run `npm start`. This will run a dev server.

# Exploring The Code

1. The smart-contract code lives in the `/contract` folder. See the README there for
   more info. In blockchain apps the smart contract is the "backend" of your app.
2. The frontend code lives in the `/frontend` folder. `/frontend/index.html` is a great
   place to start exploring. Note that it loads in `/frontend/index.js`,
   this is your entrypoint to learn how the frontend connects to the NEAR blockchain.
3. Test your contract: `npm test`, this will run the tests in `integration-tests` directory.

# Deploy

Every smart contract in NEAR has its [own associated account][near accounts].
When you run `npm run deploy`, your smart contract gets deployed to the live NEAR TestNet with a temporary dev account.
When you're ready to make it permanent, here's how:

## Step 0: Install near-cli (optional)

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `npm install`, but for best ergonomics you may want to install it globally:

    npm install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

## Step 1: Create an account for the contract

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `near-blank-project.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `near-blank-project.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

   near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

   near create-account near-blank-project.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet

## Step 2: deploy the contract

Use the CLI to deploy the contract to TestNet with your account ID.
Replace `PATH_TO_WASM_FILE` with the `wasm` that was generated in `contract` build directory.

    near deploy --accountId near-blank-project.YOUR-NAME.testnet --wasmFile PATH_TO_WASM_FILE

## Step 3: set contract name in your frontend code

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'near-blank-project.YOUR-NAME.testnet'



Admin:
1. Deploy contract
 - cấu hình address: 
    owner_id: AccountId,
    injector_address: AccountId,
    operator_address: AccountId,
    treasury_address: AccountId,
2. Tạo round
    - start_lottery: bắt đầu 1 round mới
    - close_lottery: đóng round
    - draw_final_number_and_make_lottery_claimable: tính toán giải và tìm ra người thắng cuộc
    - 
3. 


User:
1. Add stogre 
2. Buy tickets
3. Check tickets (lấy tickets )
4. View:
    - calculate_total_price_for_bulk_tickets: lấy giá tickets
    - view_latest_lottery_id: lấy cái round gần nhất
    - view_lottery: view detail 1 round
    - view_rewards_for_ticket_id: kiểm tra 1 vé vs số round xem có trung hay ko. _bracket (từ giải 0 -  5)
    - view_user_info_for_lottery_id: kiểm tra 1 user 
    - view_numbers_and_statuses_for_ticket_ids: 
    - view_random: 
    - view_random_result: 
    - get_current_timestamp: 
    
    -> buy_tickets: mua tickets
    -> claim_tickets: nhận giải