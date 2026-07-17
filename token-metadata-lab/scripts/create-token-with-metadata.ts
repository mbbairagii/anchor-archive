import {
  createFungible,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createTokenIfMissing,
  findAssociatedTokenPda,
  mintTokensTo,
  mplToolbox,
} from "@metaplex-foundation/mpl-toolbox";
import {
  createSignerFromKeypair,
  generateSigner,
  percentAmount,
  signerIdentity,
  some,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"; //umi is a ts lib that helps u turn a secret key into a keypair and create a signer that can sign transactions. 
import { readFileSync } from "fs";

const RPC_URL = "https://api.devnet.solana.com";
const SECRET_KEY_PATH = "/Users/kvh/.config/solana/id.json";

const TOKEN_NAME = "wiz";
const TOKEN_SYMBOL = "W";
const TOKEN_DECIMALS = 9;
const INITIAL_SUPPLY_HUMAN = 1_000_000;
const METADATA_URI =
  "https://raw.githubusercontent.com/mbbairagii/anchor-archive/main/token-metadata-lab/assets/wiz-token-metadata.json";

function toBaseUnits(amount: number, decimals: number): bigint {
  return BigInt(amount) * BigInt(10 ** decimals);
}

async function main() {
  const secretKey = JSON.parse(readFileSync(SECRET_KEY_PATH, "utf-8"));

  const umi = createUmi(RPC_URL).use(mplTokenMetadata()).use(mplToolbox());

  const keypair = umi.eddsa.createKeypairFromSecretKey(
    new Uint8Array(secretKey)
  );
  const signer = createSignerFromKeypair(umi, keypair);
  umi.use(signerIdentity(signer));

  const mint = generateSigner(umi);

  console.log("Creating fungible token mint...");
  await createFungible(umi, { //createFungible is from metaplex token metadata
    mint,
    authority: umi.identity,
    name: TOKEN_NAME,
    symbol: TOKEN_SYMBOL,
    uri: METADATA_URI,
    sellerFeeBasisPoints: percentAmount(0),
    decimals: some(TOKEN_DECIMALS),
  }).sendAndConfirm(umi);

  console.log("Mint created:", mint.publicKey);

  const ownerAta = findAssociatedTokenPda(umi, {
    mint: mint.publicKey,
    owner: umi.identity.publicKey,
  });

  const initialSupplyBaseUnits = toBaseUnits(
    INITIAL_SUPPLY_HUMAN,
    TOKEN_DECIMALS
  );

  console.log("Creating ATA if missing and minting initial supply...");
  await createTokenIfMissing(umi, {
    mint: mint.publicKey,
    owner: umi.identity.publicKey,
  })
    .add(
      mintTokensTo(umi, {
        mint: mint.publicKey,
        token: ownerAta,
        amount: initialSupplyBaseUnits,
      })
    )
    .sendAndConfirm(umi);

  console.log("Owner wallet:", umi.identity.publicKey);
  console.log("Owner ATA:", ownerAta);
  console.log("Initial supply minted:", INITIAL_SUPPLY_HUMAN, TOKEN_SYMBOL);
  console.log("Metadata URI:", METADATA_URI);
}

main().catch((err) => {
  console.error("Error creating token:", err);
});







//pipeline:
/*
Your script:
-Loads your wallet’s secret key.
-Uses Umi to turn that into a signer and set it as your identity.
-Calls createFungible to make a new token (mint) with metadata (name, symbol, uri).
-Finds your token account for that token.
-Creates the token account if needed and mints 1,000,000 tokens into it.
-Phantom reads your wallet on devnet and shows the 1M tokens because your wallet owns them, not because you connected to a website.

Umi is just a helper library that makes talking to Solana and using Metaplex tools easier. You don’t need to know every line of TypeScript; you need to understand the concepts and the flow, and you can learn the rest gradually as you build more.
*/