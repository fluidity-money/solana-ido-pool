import * as anchor from '@project-serum/anchor';

// we use `as const` to get the literal type of our ABI
// but this also makes everything readonly, so we need to
// remove it to make it compatible with anchor's types
export type Writable<T> = {-readonly [k in keyof T]: Writable<T[k]>};

export function mustEnv<T>(env: string, parser: (input: string) => T): T {
    const val = process.env[env];
    if (!val) throw new Error(`Environment variable ${env} not set!`);
    return parser(val);
}

export function pkFromString(input: string): anchor.web3.PublicKey {
    return new anchor.web3.PublicKey(input);
}

export function bnFromString(input: string): anchor.BN {
    return new anchor.BN(input, 10);
}
