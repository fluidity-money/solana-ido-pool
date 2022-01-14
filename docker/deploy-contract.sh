#!/bin/sh -e

if [ -f "deployed" ]; then
  echo "it looks like the contracts have already been deployed!"
  echo "redeploying now will change addresses"
  echo "bring down your volumes with 'docker-compose down' then bring them back up to reset chain state and redeploy"
  exit 0
fi

echo "Waiting for RPC connction..."
until [ "$(curl $FLU_IDO_RPC_ADDR/health)" = "ok" ]; do
  echo "."
  sleep 1;
done

echo "Creating USDc mint (don't do this in prod)"
USDC_MINT=$(spl-token --url $FLU_IDO_RPC_ADDR create-token --mint-authority $FLU_IDO_AUTH_PATH --decimals 6 docker/usdc_id.json | \
  grep "Creating token" | cut -d" " -f3)
echo "USDc mint address $USDC_MINT"
echo

echo "Creating gov token mint"
GOV_MINT=$(spl-token --url $FLU_IDO_RPC_ADDR create-token --mint-authority $FLU_IDO_AUTH_PATH --decimals 9 docker/gov_id.json| \
  grep "Creating token" | cut -d" " -f3)
echo "Gov token mint address $GOV_MINT"
echo

echo "Creating gov token supply account"
GOV_ACCOUNT=$(spl-token --url $FLU_IDO_RPC_ADDR create-account --owner $FLU_IDO_AUTH_PATH $GOV_MINT | \
  grep "Creating account" | cut -d" " -f3)
echo "Gov token account $GOV_ACCOUNT"
echo

echo "Minting into gov token account"
spl-token --url $FLU_IDO_RPC_ADDR mint --mint-authority $FLU_IDO_AUTH_PATH $GOV_MINT $FLU_GOV_AMOUNT $GOV_ACCOUNT
echo
echo

cd scripts &&
  FLU_IDO_USDC_MINT=$USDC_MINT \
  FLU_IDO_WATERMELON_MINT=$GOV_MINT \
  FLU_IDO_WATERMELON_SOURCE=$GOV_ACCOUNT \
  FLU_IDO_WATERMELON_AMOUNT=$FLU_GOV_AMOUNT \
  ANCHOR_WALLET=../docker/test_signer.json \
  FLU_IDO_NAME="test_ido3" \
  FLU_IDO_START=$(expr $(date +%s) + $FLU_IDO_START_TIME) \
  FLU_IDO_END_DEPOSITS=$(expr $(date +%s) + $FLU_IDO_DEPOSIT_TIME) \
  FLU_IDO_END_IDO=$(expr $(date +%s) + $FLU_IDO_IDO_TIME) \
  FLU_IDO_END_ESCROW=$(expr $(date +%s) + $FLU_IDO_ESCROW_TIME) \
  npx ts-node deploy.ts

touch ../deployed
