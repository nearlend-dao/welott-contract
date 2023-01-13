set -e
NETWORK=near
MASTER_ACC=welott.$NETWORK
CONTRACT_ACC=main.$MASTER_ACC
OWNER=nearlend-dao.sputnik-dao.near
TREASURY_ACC=nearlend-dao.sputnik-dao.near
INJECTOR_ACC=nearlend-dao.sputnik-dao.near
OPERATOR_ACC=operator.$MASTER_ACC

export NEAR_ENV=mainnet


# echo "################ DELETE THE ACCOUNT #########################"
# near delete $CONTRACT_ACC $MASTER_ACC

# echo "################ CREATE NEW ACCOUNT #########################"
# near create-account $CONTRACT_ACC --masterAccount $MASTER_ACC --initialBalance 5
# near create-account $OPERATOR_ACC --masterAccount $MASTER_ACC --initialBalance 2

# echo "################ BUILD CONTRACT #########################"
../build.sh

# echo "################ DEPLOY CONTRACT #########################"
near deploy $CONTRACT_ACC ../out/nearlott.wasm 

# echo "####################### GET CONFIG LOTTERY #########################"
# near view $CONTRACT_ACC view_config_lottery ''


# # echo "################# INIT CONTRACT #########################"
# near call $CONTRACT_ACC --accountId=$MASTER_ACC new '{
#     "owner_id":"'$OWNER'",
#     "injector_address":"'$INJECTOR_ACC'",
#     "operator_address": "'$OPERATOR_ACC'",
#     "treasury_address": "'$TREASURY_ACC'",
#      "config_lottery": {
#         "time_run_lottery": 43200000000000,
#         "price_ticket_in_near": "250000000000000000000000",
#         "discount_divisor": "0",
#         "rewards_breakdown": [125, 375, 750, 1250, 2500, 5000],
#         "reserve_fee": "2200",
#         "operate_fee": "500"
#      }
# }'

# echo "####################### GET CONFIG CONTRACT #########################"

near view $CONTRACT_ACC get_config ''

# near call $CONTRACT_ACC --accountId=$OWNER set_config_lottery '{
#     "_config_lottery": {
#     "time_run_lottery": 36000000000000,
#     "price_ticket_in_near": "250000000000000000000000",
#     "discount_divisor": "0",
#     "rewards_breakdown": [125, 375, 750, 1250, 2500, 5000],
#     "reserve_fee": "2200",
#     "operate_fee": "500"
#     }
# }'

#  near view $CONTRACT_ACC view_config_lottery ''

# echo "####################### GET CONFIG CONTRACT #########################"
# near view $CONTRACT_ACC --accountId=$CONTRACT_ACC view_random '' 

# echo "######################## GET LATEST ID ROUND #########################"
# near view $CONTRACT_ACC view_latest_lottery_id ''

# echo "######################## GET DETAIL CURRENT ROUND #########################"
# near view $CONTRACT_ACC view_current_lottery_running ''

# echo "################# DEPOSIT STORAGE #########################"
# near call $CONTRACT_ACC --accountId=$MASTER_ACC storage_deposit '{
#     "account_id":"'$OWNER'"
# }' --deposit=0.25


# near call $CONTRACT_ACC --accountId=$MASTER_ACC storage_deposit '{
#     "account_id":"'$OPERATOR_ACC'"
# }' --deposit=0.25