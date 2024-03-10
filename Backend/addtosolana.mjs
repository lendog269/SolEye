import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

// Define review inputs
const RESTAURANT_NAME = "Gay.co";
const RATING = 5;
const REVIEW_TEXT = "Gay";
const WHITELIST_FLAG = false;
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import fs from 'fs';
 // Assuming you have exported the IDL object in idl.js
 const idl = JSON.parse(fs.readFileSync('/Users/lennybreeds/Desktop/Computingprojects/SolEye/front-end/idl.json', 'utf8'));
// Instantiate the Program with the IDL object
// Specify the path to your wallet keypair file directly
const walletKeypairPath = '/Users/lennybreeds/Desktop/Computingprojects/SolEye/front-end/wallet-keypair.json';

// Load the wallet keypair
const keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(walletKeypairPath, 'utf-8')))
);

// Create a Wallet instance from the keypair
const wallet = new Wallet(keypair);

// Specify the provider URL directly
const providerUrl = 'http://127.0.0.1:8899';
const programId = new web3.PublicKey('EvWSp8yjbVF9vC7BrBWzejhS6GsqcyE3hELv54r614Yt'.trim());
// Create a connection to the Solana cluster
const connection = new web3.Connection(providerUrl, 'processed');

// Create the Anchor provider
const provider = new AnchorProvider(connection, wallet, {
  preflightCommitment: 'processed',
});
// Step 1: Configure the client to use the local cluster
anchor.setProvider(provider);
async function main(url, url_reasons, probability,whitelist,domain_age_reasons,javascript_code_reasons,site_content_reasons){
  const [reviewPda] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(url)],
    programId // Ensure you have the programId correctly defined or accessible here
  );
  
  
  // Step 3: Fetch Latest Blockhash
  const latestBlockhash = await provider.connection.getLatestBlockhash("finalized");
  
  // Step 4: Send and Confirm the Transaction
  // Adjust `addWebsite` method parameters according to your actual program's method signature
  const program = new anchor.Program(idl, programId, provider);
  const tx = await program.rpc.addWebsite(
    url, url_reasons, probability,whitelist,domain_age_reasons,javascript_code_reasons,site_content_reasons, // Repeat placeholders as needed
      {
        accounts: {
          signer: provider.wallet.publicKey,
          website: reviewPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        blockhash: latestBlockhash.blockhash,
        signers: [wallet.payer], // Include any signers if necessary
      }
  );
  
  // Confirm the transaction
  await provider.connection.confirmTransaction({
    signature: tx,
    blockhash: latestBlockhash.blockhash,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  });
}
// Assuming the first argument passed to the script is the JSON string
const args = process.argv[2];
const { url, url_reasons, probability, whitelist, domain_age_reasons, javascript_code_reasons, site_content_reasons } = JSON.parse(args);
main(url, url_reasons, probability,whitelist,domain_age_reasons,javascript_code_reasons,site_content_reasons);
// Use these variables in your script as needed




