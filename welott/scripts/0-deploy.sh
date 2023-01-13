set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott52.$MASTER_ACC
OWNER=$MASTER_ACC
TREASURY_ACC=lamns1.$NETWORK
INJECTOR_ACC=lamns1.$NETWORK
OPERATOR_ACC=lamns1.$NETWORK


export NEAR_ENV=$NETWORK


# echo "################ DELETE THE ACCOUNT #########################"
# near delete $CONTRACT_ACC $MASTER_ACC

# echo "################ CREATE NEW ACCOUNT #########################"
# near create-account $CONTRACT_ACC --masterAccount $MASTER_ACC --initialBalance 10

# echo "################ BUILD CONTRACT #########################"
# ../build.sh

# echo "################ DEPLOY CONTRACT #########################"
# near deploy $CONTRACT_ACC ../out/nearlott.wasm

# echo "################# INIT CONTRACT #########################"
#  near call $CONTRACT_ACC --accountId=$OWNER new '{
#      "owner_id":"'$OWNER'",
#      "injector_address":"'$INJECTOR_ACC'",
#      "operator_address": "'$OPERATOR_ACC'",
#      "treasury_address": "'$TREASURY_ACC'",
#      "config_lottery": {
#         "time_run_lottery": 36000000000000,
#         "price_ticket_in_near": "250000000000000000000000",
#         "discount_divisor": "0",
#         "rewards_breakdown": [125, 375, 750, 1250, 2500, 5000],
#         "reserve_fee": "2200",
#         "operate_fee": "500"
#      }
#  }'

# echo "####################### GET CONFIG LOTTERY #########################"
# near view $CONTRACT_ACC view_config_lottery ''

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
# near call $CONTRACT_ACC --accountId=$OWNER storage_deposit '{
#     "account_id":"'$OWNER'"
# }' --deposit=0.25



near view $CONTRACT_ACC view_accounts '{"_cursor": 0, "_size": 50}'