set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott9.$MASTER_ACC
OWNER=$MASTER_ACC
TREASURY_ACC=lamns1.$NETWORK
INJECTOR_ACC=lamns1.$NETWORK
OPERATOR_ACC=lamns1.$NETWORK

export NEAR_ENV=$NETWORK


# echo "################ DELETE THE ACCOUNT #########################"
# near delete $CONTRACT_ACC $MASTER_ACC

# echo "################ CREATE NEW ACCOUNT #########################"
# near create-account $CONTRACT_ACC --masterAccount $MASTER_ACC --initialBalance 10

echo "################ BUILD CONTRACT #########################"
../build.sh

echo "################ DEPLOY CONTRACT #########################"
near deploy $CONTRACT_ACC ../out/nearlott.wasm 

# echo "################# INIT CONTRACT #########################"
# near call $CONTRACT_ACC --accountId=$OWNER new '{
#     "owner_id":"'$OWNER'",
#     "injector_address":"'$INJECTOR_ACC'",
#     "operator_address": "'$OPERATOR_ACC'",
#     "treasury_address": "'$TREASURY_ACC'"
# }'

echo "####################### GET CONFIG CONTRACT #########################"
near view $CONTRACT_ACC get_config ''

# echo "####################### GET CONFIG CONTRACT #########################"
# near view $CONTRACT_ACC --accountId=$CONTRACT_ACC view_random '' 

echo "######################## GET LATEST ID ROUND #########################"
near view $CONTRACT_ACC view_latest_lottery_id ''

# echo "######################## GET DETAIL CURRENT ROUND #########################"
# near view $CONTRACT_ACC view_current_lottery_running ''

echo "################# DEPOSIT STORAGE #########################"
near call $CONTRACT_ACC --accountId=$OWNER storage_deposit '{
    "account_id":"'$OWNER'"
}' --deposit=0.25




