set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott40.$MASTER_ACC
OWNER=$MASTER_ACC
TREASURY_ACC=lamns1.$NETWORK
INJECTOR_ACC=lamns1.$NETWORK
OPERATOR_ACC=lamns1.$NETWORK
DECIMAL_20=00000000000000000000
DECIMAL_23=00000000000000000000000
DECIMAL_24=000000000000000000000000
ONE_YOCTO=0.000000000000000000000001


export NEAR_ENV=$NETWORK


# echo "########################### GET CURRENT TIMESTAMP #########################"
# near view $CONTRACT_ACC get_current_timestamp ''

# echo "######################## CLOSE ROUND #############################"
# near call $CONTRACT_ACC --accountId=$OWNER close_lottery '{
#     "_lottery_id": 3
# }' --depositYocto=1

# near call $CONTRACT_ACC --accountId=$OWNER draw_final_number_and_make_lottery_claimable '{
#     "_lottery_id": 2,
#     "_auto_injection": true
# }' --depositYocto=1

# near call $CONTRACT_ACC --accountId=$OWNER claim_tickets '{
#     "_lottery_id": 1,
#     "_ticket_ids": true
# }' --depositYocto=1

echo "########################### START ROUND #########################"
# end_time=$(($(date +%s) + 24*60*60))
end_time=$(($(date +%s) + 60))
near call $CONTRACT_ACC --accountId=$OWNER start_lottery '{
    "_end_time": '$end_time'000000000,
    "_price_ticket_in_near": "1'$DECIMAL_24'",
    "_discount_divisor": "0",
    "_rewards_breakdown": [125, 375, 750, 1250, 2500, 5000],
    "_reserve_fee": "2000",
    "_operate_fee": "500"

}' --depositYocto=1

# echo "######################## BUY TICKETS #################################"
# near call $CONTRACT_ACC --accountId=$OWNER buy_tickets '{
#     "_lottery_id": 4,
#     "_ticket_numbers":[1233145,1233146,1233141,1233142,1233143]
# }' --depositYocto=499000000000000000000



# echo "####################### GET CURRENT ROUND ID #########################"
# near view $CONTRACT_ACC view_latest_lottery_id ''

# echo "######################## GET DETAIL CURRENT ROUND #########################"
# near view $CONTRACT_ACC view_current_lottery_running ''

# echo "####################### GET CURRENT ROUND ID #########################"
# near view $CONTRACT_ACC view_lottery '{
#     "_lottery_id": 1
# }'


# echo "######################## CHECK PRICES FOR BUY TICKETS #################################"
# near view $CONTRACT_ACC calculate_total_price_for_bulk_tickets '{
#     "_lottery_id": 1,
#     "_number_tickets": 5
# }'

# echo "######################## BUY TICKETS #################################"
# near call $CONTRACT_ACC --accountId=$OWNER buy_tickets '{
#     "_lottery_id": 2,
#     "_ticket_numbers":[1233145,1233146,1233141,1233142,1233143]
# }' --deposit=4.99

# echo "######################## GET DETAIL CURRENT ROUND #########################"
# near view $CONTRACT_ACC view_number_tickets_per_lottery '{
#     "_lottery_id": 1
# }'

# echo "####################### VIEW TICKETS #################################"
# near view $CONTRACT_ACC  view_user_info_for_lottery_id '{
#     "_user": "'$OWNER'",
#     "_lottery_id": 1,
#     "_cursor": 0,
#     "_size": 100
# }' 


# echo "######################## CLOSE ROUND #############################"
# near call $CONTRACT_ACC --accountId=$OWNER close_lottery '{
#     "_lottery_id": 1
# }' --depositYocto=1