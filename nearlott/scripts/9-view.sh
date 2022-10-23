set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott9.$MASTER_ACC
OWNER=$MASTER_ACC

export NEAR_ENV=$NETWORK

# near view $CONTRACT_ACC view_latest_lottery_id ''
# near view $CONTRACT_ACC view_lottery '{"_lottery_id": 1}'
# near view $CONTRACT_ACC view_lottery '{"_lottery_id": 5}'

# echo "######################## GET DETAIL CURRENT ROUND #########################"
# near view $CONTRACT_ACC view_current_lottery_running ''

# echo "####################### GET CONFIG CONTRACT #########################"
# near view $CONTRACT_ACC get_config ''

# near view $CONTRACT_ACC view_number_tickets_per_lottery '{"_lottery_id": 1}'

near view $CONTRACT_ACC view_lotteries '{"_cursor": 0, "_size": 50}'


near view $CONTRACT_ACC get_current_timestamp ''