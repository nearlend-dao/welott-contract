set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott22.$MASTER_ACC
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

# near view $CONTRACT_ACC view_lotteries '{"_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 30, "_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_numbers_and_statuses_for_ticket_ids '{"_ticket_ids": [13875, 13876, 13877, 13878, 13879,
#     13880, 13881, 13882, 13883, 13884,
#     13885, 13886, 13887, 13888, 13889,
#     13890, 13891, 13892, 13893, 13894,
#     13895, 13896, 13897, 13898, 13899,
#     13900, 13901, 13902, 13903, 13904,
#     13905, 13906, 13907, 13908, 13909,
#     13910, 13911, 13912, 13913, 13914,
#     13915, 13916, 13917, 13918, 13919,
#     13920, 13921, 13922], "_lottery_id": 30}'

# near view $CONTRACT_ACC get_current_timestamp ''

# echo "################# DEPOSIT STORAGE #########################"
# near call $CONTRACT_ACC --accountId=$OWNER storage_deposit '{
#     "account_id":"lamns3.testnet"
# }' --deposit=0.25
# near view $CONTRACT_ACC storage_balance_of '{"account_id": "lamns1.testnet"}'
# # near view $CONTRACT_ACC storage_balance_of '{"account_id": "tn888.testnet"}'

# near view $CONTRACT_ACC view_number_tickets_per_lottery '{"_lottery_id": 2}'

# 250000000000000000000000
# 35470000000000000000000
# 8470000000000000000000

# 007750000000000000000000
# 250000000000000000000000

# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 1, "_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 2, "_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 1, "_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 4, "_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 5, "_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 6, "_cursor": 0, "_size": 50}'
# near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 8, "_cursor": 0, "_size": 50}'

near view $CONTRACT_ACC view_lottery '{"_lottery_id": 4}'
near view $CONTRACT_ACC view_lottery '{"_lottery_id": 5}'
near view $CONTRACT_ACC view_lottery '{"_lottery_id": 6}'