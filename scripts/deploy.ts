import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Writable, mustEnv, pkFromString, bnFromString } from "./util/util";
import { idl as ido_pool } from './util/ido-pool-abi'

const ido_pool_idl = ido_pool as Writable<typeof ido_pool>;

// this gets accessed when we connect the provider but with a much worse error message
if (!process.env[`ANCHOR_WALLET`]) throw new Error(`Env ANCHOR_WALLET not found, set it to the path of your wallet!`);

// mint account for usdc
const idoUsdcMint = mustEnv(`FLU_IDO_USDC_MINT`, pkFromString);

// mint account for gov tokens
const idoWatermelonMint = mustEnv(`FLU_IDO_WATERMELON_MINT`, pkFromString);

// account that holds gov tokens we transfer from, must be owned by our wallet
const idoWatermelonSource = mustEnv(`FLU_IDO_WATERMELON_SOURCE`, pkFromString);

const idoName = process.env[`FLU_IDO_NAME`]!;
if (!idoName) throw new Error(`Env FLU_IDO_NAME not set!`);
if (idoName.length > 10) throw new Error(`Ido name ${idoName} too long, must be <= 10 chars!`);

const idoWatermelonAmount = mustEnv(`FLU_IDO_WATERMELON_AMOUNT`, bnFromString);

const startIdo = mustEnv(`FLU_IDO_START`, bnFromString);
const endDeposits = mustEnv(`FLU_IDO_END_DEPOSITS`, bnFromString);
const endIdo = mustEnv(`FLU_IDO_END_IDO`, bnFromString);
const endEscrow = mustEnv(`FLU_IDO_END_ESCROW`, bnFromString);

const initializePool = async () => {
  const provider = anchor.Provider.local(process.env.FLU_SOLANA_RPC_ADDR);
  const program = new anchor.Program(ido_pool_idl, ido_pool_idl.metadata.address, provider)

  const idoAuthority = provider.wallet.publicKey;

  const bumps = {
    idoAccount: 0,
    redeemableMint: 0,
    poolWatermelon: 0,
    poolUsdc: 0,
  };

  // program derived accounts

  // data account
  const [idoAccount, idoAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(idoName)],
    program.programId
  );
  bumps.idoAccount = idoAccountBump;

  // mint account for the option account
  const [redeemableMint, redeemableMintBump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(idoName), Buffer.from("redeemable_mint")],
    program.programId
  );
  bumps.redeemableMint = redeemableMintBump;

  // escrow account for gov tokens
  const [poolWatermelon, poolWatermelonBump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(idoName), Buffer.from("pool_watermelon")],
    program.programId
  );
  bumps.poolWatermelon = poolWatermelonBump;

  // escrow account for USDc
  const [poolUsdc, poolUsdcBump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(idoName), Buffer.from("pool_usdc")],
    program.programId
  );
  bumps.poolUsdc = poolUsdcBump;

  const idoTimes = {
    startIdo,
    endDeposits,
    endIdo,
    endEscrow,
  };

  const sig = await program.rpc.initializePool(
    idoName,
    bumps,
    idoWatermelonAmount,
    idoTimes,
    {
      accounts: {
        idoAuthority,
        idoAuthorityWatermelon: idoWatermelonSource,
        idoAccount,
        watermelonMint: idoWatermelonMint,
        usdcMint: idoUsdcMint,
        redeemableMint,
        poolWatermelon,
        poolUsdc,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
    },
  );

  console.log(`Initialized IDO pool in transaction sig ${sig}`);
};

initializePool()
  .then(() => console.log(`Done!`));

