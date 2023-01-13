set -e
NETWORK=near
MASTER_ACC=welott.$NETWORK
CONTRACT_ACC=main.$MASTER_ACC
OWNER=nearlend-dao.sputnik-dao.near
TREASURY_ACC=nearlend-dao.sputnik-dao.near
INJECTOR_ACC=nearlend-dao.sputnik-dao.near
OPERATOR_ACC=operator.$MASTER_ACC

export NEAR_ENV=mainnet



# echo "########################### GET CURRENT TIMESTAMP #########################"
# near view $CONTRACT_ACC get_current_timestamp ''

# # echo "######################## CLOSE ROUND #############################"
#  near call $CONTRACT_ACC --accountId=$OWNER close_lottery '{}' --depositYocto=1

# near call $CONTRACT_ACC --accountId=$OWNER draw_final_number_and_make_lottery_claimable '{
#     "_lottery_id": 2,
#     "_auto_injection": true
# }' --depositYocto=1

# near call $CONTRACT_ACC --accountId=$OWNER claim_tickets '{
#     "_lottery_id": 1,
#     "_ticket_ids": true
# }' --depositYocto=1

# echo "########################### START ROUND #########################"
# # end_time=$(($(date +%s) + 24*60*60))
# end_time=$(($(date +%s) + 1000*60))
near call $CONTRACT_ACC --accountId=$OPERATOR_ACC start_lottery '' --depositYocto=1

# echo "######################## BUY TICKETS #################################"
# near call $CONTRACT_ACC --accountId=$OWNER buy_tickets '{
#     "_lottery_id": 4,
#     "_ticket_numbers":[1233145,1233146,1233141,1233142,1233143]
# }' --depositYocto=499000000000000000000



# echo "####################### GET CURRENT ROUND ID #########################"
# near view $CONTRACT_ACC view_latest_lottery_id ''

echo "######################## GET DETAIL CURRENT ROUND #########################"
near view $CONTRACT_ACC view_current_lottery_running ''

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


# # echo "######################## CLOSE ROUND #############################"
# near call $CONTRACT_ACC --accountId=$OWNER close_lottery '{}' --depositYocto=1