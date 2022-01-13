#!/bin/bash

ANCHOR=${ANCHOR_CMD:-anchor}

echo "Building contracts..."
$ANCHOR build
echo

echo "Deploying contracts..."
$ANCHOR deploy
echo

echo "Generating TS idl file..."
echo "export const idl =" "$(cat $(dirname $0)/../target/idl/ido_pool.json)" "as const;" > $(dirname $0)/util/ido-pool-abi.ts
echo


