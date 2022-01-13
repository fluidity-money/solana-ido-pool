#!/bin/bash

set -e

SOLANA=${SOLANA_CMD:-solana}
KEYGEN=${SOLANA_KEYGEN_CMD:-solana-keygen}
SPL=${SPL_CMD:-spl-token}

IDO_AUTH_PATH=${IDO_AUTH_PATH:-./ido_admin_id.json}

echo "Using keyfile $IDO_AUTH_PATH"
if [ ! -f "$IDO_AUTH_PATH" ]; then
  echo "Keyfile $IDO_AUTH_PATH not found, generating..."
  $KEYGEN new -o "$IDO_AUTH_PATH"
fi
echo

echo "Funding account..."
$SOLANA airdrop --keypair $IDO_AUTH_PATH 100
echo

echo "Creating USDc mint (don't do this in prod)"
USDC_MINT=$($SPL create-token --mint-authority $IDO_AUTH_PATH --decimals 6 | grep "Creating token" | cut -d" " -f3)
echo "USDc mint address $USDC_MINT"
echo

echo "Creating gov token mint"
GOV_MINT=$($SPL create-token --mint-authority $IDO_AUTH_PATH --decimals 9 | grep "Creating token" | cut -d" " -f3)
echo "Gov token mint address $GOV_MINT"
echo

echo "Creating gov token supply account"
GOV_ACCOUNT=$($SPL create-account --owner $IDO_AUTH_PATH $GOV_MINT | grep "Creating account" | cut -d" " -f3)
echo "Gov token account $GOV_ACCOUNT"
echo

echo "Minting into gov token account"
GOV_AMOUNT=2000
$SPL mint --mint-authority $IDO_AUTH_PATH $GOV_MINT $GOV_AMOUNT $GOV_ACCOUNT
echo
echo

echo "ANCHOR_WALLET=$IDO_AUTH_PATH"
echo "FLU_IDO_USDC_MINT=$USDC_MINT"
echo "FLU_IDO_WATERMELON_MINT=$GOV_MINT"
echo "FLU_IDO_WATERMELON_SOURCE=$GOV_ACCOUNT"
echo "FLU_IDO_WATERMELON_AMOUNT=$GOV_AMOUNT"
echo

echo "FLU_IDO_USDC_MINT=$USDC_MINT FLU_IDO_WATERMELON_MINT=$GOV_MINT FLU_IDO_WATERMELON_SOURCE=$GOV_ACCOUNT FLU_IDO_WATERMELON_AMOUNT=$GOV_AMOUNT ANCHOR_WALLET=$IDO_AUTH_PATH FLU_IDO_NAME="test_ido3" FLU_IDO_START=$(expr $(date +%s) + 120) FLU_IDO_END_DEPOSITS=$(expr $(date +%s) + 180) FLU_IDO_END_IDO=$(expr $(date +%s) + 240) FLU_IDO_END_ESCROW=$(expr $(date +%s) + 300) ts-node deploy.ts"
