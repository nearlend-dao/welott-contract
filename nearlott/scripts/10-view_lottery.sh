set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott9.$MASTER_ACC
OWNER=$MASTER_ACC

export NEAR_ENV=$NETWORK

# near view $CONTRACT_ACC view_current_lottery_running '{}'

# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 12, "_cursor": 0, "_size": 1000}'
# near view $CONTRACT_ACC view_numbers_and_statuses_for_ticket_ids '{"_ticket_ids": [13515, 13516, 13517, 13518, 13519, 13520], "_lottery_id": 30}'

### Close a lottery
# near call $CONTRACT_ACC --accountId=$OWNER close_lottery '{
#     "_lottery_id": 4
# }' --depositYocto=1

## Draw a final number
# near call $CONTRACT_ACC --accountId=$OWNER draw_final_number_and_make_lottery_claimable '{
#     "_lottery_id": 4,
#     "_auto_injection": true
# }' --depositYocto=1


# near call $CONTRACT_ACC --accountId=$OWNER claim_tickets '{
#     "_ticket_ids": [13515, 13516, 13517, 13518, 13519, 13520],
#     "_lottery_id": 28,
#     "_brackets": [0,1,1,1,1,1]

# }' --depositYocto=1

# near call $CONTRACT_ACC --accountId=$OWNER claim_tickets '{
#     "_ticket_ids": [1908, 3289],
#     "_lottery_id": 13,
#     "_brackets": [5,5]

# }' --depositYocto=1


# near view $CONTRACT_ACC view_all_lotteries_by_user '{"_user": "lamns1.testnet", "_lottery_id": 28, "_cursor": 0, "_size": 1}'

near view $CONTRACT_ACC storage_balance_of '{"account_id": "lamns1.testnet"}'
near view $CONTRACT_ACC account_storage_usage '{"account_id": "tn888.testnet"}' --accountId=$OWNER 
near view $CONTRACT_ACC storage_available '{"_account_id": "tn888.testnet"}' --accountId=$OWNER 
