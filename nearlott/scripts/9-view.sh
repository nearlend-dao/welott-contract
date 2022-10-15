set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=nearlott.$MASTER_ACC
OWNER=$MASTER_ACC

export NEAR_ENV=$NETWORK

near view $CONTRACT_ACC view_latest_lottery_id ''
# near view $CONTRACT_ACC view_lottery '{"_lottery_id": 1}'
near view $CONTRACT_ACC view_random ''
near view $CONTRACT_ACC view_random_result ''
near view $CONTRACT_ACC get_current_timestamp ''
near view $CONTRACT_ACC get_contract_info ''
