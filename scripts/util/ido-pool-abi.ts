export const idl = {
  "version": "0.1.0",
  "name": "ido_pool",
  "instructions": [
    {
      "name": "initializePool",
      "accounts": [
        {
          "name": "idoAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "idoAuthorityWatermelon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "watermelonMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "poolWatermelon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "idoName",
          "type": "string"
        },
        {
          "name": "bumps",
          "type": {
            "defined": "PoolBumps"
          }
        },
        {
          "name": "numIdoTokens",
          "type": "u64"
        },
        {
          "name": "idoTimes",
          "type": {
            "defined": "IdoTimes"
          }
        }
      ]
    },
    {
      "name": "initUserRedeemable",
      "accounts": [
        {
          "name": "userAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "exchangeUsdcForRedeemable",
      "accounts": [
        {
          "name": "userAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "userUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initEscrowUsdc",
      "accounts": [
        {
          "name": "userAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "escrowUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "exchangeRedeemableForUsdc",
      "accounts": [
        {
          "name": "userAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "escrowUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "watermelonMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "exchangeRedeemableForWatermelon",
      "accounts": [
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "userAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userWatermelon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userRedeemable",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "watermelonMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "redeemableMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolWatermelon",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawPoolUsdc",
      "accounts": [
        {
          "name": "idoAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "idoAuthorityUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "watermelonMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "poolUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "withdrawFromEscrow",
      "accounts": [
        {
          "name": "payer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "userAuthority",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowUsdc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "idoAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "usdcMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateExchangeRate",
      "accounts": [
        {
          "name": "idoAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "idoAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num",
          "type": "u64"
        },
        {
          "name": "denom",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "IdoAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "idoName",
            "type": {
              "array": [
                "u8",
                10
              ]
            }
          },
          {
            "name": "bumps",
            "type": {
              "defined": "PoolBumps"
            }
          },
          {
            "name": "idoAuthority",
            "type": "publicKey"
          },
          {
            "name": "usdcMint",
            "type": "publicKey"
          },
          {
            "name": "redeemableMint",
            "type": "publicKey"
          },
          {
            "name": "watermelonMint",
            "type": "publicKey"
          },
          {
            "name": "poolUsdc",
            "type": "publicKey"
          },
          {
            "name": "poolWatermelon",
            "type": "publicKey"
          },
          {
            "name": "exchangeNum",
            "type": "u64"
          },
          {
            "name": "exchangeDenom",
            "type": "u64"
          },
          {
            "name": "numOptions",
            "type": "u64"
          },
          {
            "name": "numIdoTokens",
            "type": "u64"
          },
          {
            "name": "idoTimes",
            "type": {
              "defined": "IdoTimes"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "IdoTimes",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "startIdo",
            "type": "i64"
          },
          {
            "name": "endDeposits",
            "type": "i64"
          },
          {
            "name": "endIdo",
            "type": "i64"
          },
          {
            "name": "endEscrow",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "PoolBumps",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "idoAccount",
            "type": "u8"
          },
          {
            "name": "redeemableMint",
            "type": "u8"
          },
          {
            "name": "poolWatermelon",
            "type": "u8"
          },
          {
            "name": "poolUsdc",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "IdoFuture",
      "msg": "IDO must start in the future"
    },
    {
      "code": 6001,
      "name": "SeqTimes",
      "msg": "IDO times are non-sequential"
    },
    {
      "code": 6002,
      "name": "StartIdoTime",
      "msg": "IDO has not started"
    },
    {
      "code": 6003,
      "name": "EndDepositsTime",
      "msg": "Deposits period has ended"
    },
    {
      "code": 6004,
      "name": "EndIdoTime",
      "msg": "IDO has ended"
    },
    {
      "code": 6005,
      "name": "IdoNotOver",
      "msg": "IDO has not finished yet"
    },
    {
      "code": 6006,
      "name": "EscrowNotOver",
      "msg": "Escrow period has not finished yet"
    },
    {
      "code": 6007,
      "name": "LowUsdc",
      "msg": "Insufficient USDC"
    },
    {
      "code": 6008,
      "name": "LowRedeemable",
      "msg": "Insufficient redeemable tokens"
    },
    {
      "code": 6009,
      "name": "UsdcNotEqRedeem",
      "msg": "USDC total and redeemable total don't match"
    },
    {
      "code": 6010,
      "name": "InvalidNonce",
      "msg": "Given nonce is invalid"
    }
  ],
  "metadata": {
    "address": "v8N37RZ3w484F5XXnxeQnKEoARLZaqtJ31ibYwxkCRM"
  }
} as const;
