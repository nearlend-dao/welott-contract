set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=mitsori9.$NETWORK
CONTRACT_ACC=nearlott.$MASTER_ACC
OWNER=$MASTER_ACC
TREASURY_ACC=mitsori10.$NETWORK
INJECTOR_ACC=mitsori11.$NETWORK
OPERATOR_ACC=mitsori12.$NETWORK

export NEAR_ENV=$NETWORK

# #delete acc
echo "Delete $CONTRACT_ACC? are you sure? Ctrl-C to cancel"
# # read input
# near delete $CONTRACT_ACC $MASTER_ACC
near create-account $CONTRACT_ACC --masterAccount $MASTER_ACC --initialBalance 10
near deploy $CONTRACT_ACC ../out/nearlott.wasm 
near call $CONTRACT_ACC --accountId=$OWNER new '{
    "owner_id":"'$OWNER'",
    "injector_address":"'$INJECTOR_ACC'",
    "operator_address": "'$OPERATOR_ACC'",
    "treasury_address": "'$TREASURY_ACC'"
}'
