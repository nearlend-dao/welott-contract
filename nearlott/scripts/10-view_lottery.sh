set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott9.$MASTER_ACC
OWNER=$MASTER_ACC

export NEAR_ENV=$NETWORK

near view $CONTRACT_ACC view_current_lottery_running '{}'

near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "lamns1.testnet", "_lottery_id": 28, "_cursor": 0, "_size": 100}'
near view $CONTRACT_ACC view_numbers_and_statuses_for_ticket_ids '{"_ticket_ids": [13875, 13876, 13877, 13878, 13879,
    13880, 13881, 13882, 13883, 13884,
    13885, 13886, 13887, 13888, 13889,
    13890, 13891, 13892, 13893, 13894,
    13895, 13896, 13897, 13898, 13899,
    13900, 13901, 13902, 13903, 13904,
    13905, 13906, 13907, 13908, 13909,
    13910, 13911, 13912, 13913, 13914,
    13915, 13916, 13917, 13918, 13919,
    13920, 13921, 13922], "_lottery_id": 30}'