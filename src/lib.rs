mod programs;
#[cfg(test)] mod tests {
    use solana_sdk::{message::Message, signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
    use bs58;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey,system_instruction::transfer, system_program};
    use std::str::FromStr;
    use crate::programs::Turbin3_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]

    fn enroll() {
        // Create a Solana devnet connection 
        let rpc_client: RpcClient = RpcClient::new(RPC_URL);

        // Let's define our accounts 
        let signer: Keypair = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        // Define our instruction data 
        let args: CompleteArgs = CompleteArgs { github: b"crgold".to_vec() };

        // Get recent blockhash 
        let blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash");

        // Now we can invoke the "complete" function 
        let transaction: Transaction = WbaPrereqProgram::complete( &[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()), &[&signer], blockhash );

        // Send the transaction 
        let signature: solana_sdk::signature::Signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed to send transaction");

        // Print our transaction out 
        println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }

    #[test]
    fn keygen() {
        let kp: Keypair = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");

        println!("{:?}", kp.to_bytes());
    } #[test] fn airdrop() {
        // Import our keypair
        let keypair: Keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Connected to Solana Devnet RPC Client
        let client: RpcClient = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {

        Ok(s) => {
        println!("Success! Check out your TX here:");

        println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());

        },

        Err(e) => println!("Oops, something went wrong: {}", e.to_string()) };
    } #[test] fn transfer_sol() {
        // Import our keypair 
        let keypair: Keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 
        // Define our Turbin3 public key 
        let to_pubkey: Pubkey = Pubkey::from_str("CXQGd9n729Y2JjwvNfaEhvt1JXTqyr4JHCksNVUW8FpN").unwrap();
        // Create a Solana devnet connection 
        let rpc_client: RpcClient = RpcClient::new(RPC_URL);
        // Get recent blockhash 
        let recent_blockhash: solana_sdk::hash::Hash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash");

        // Get balance of dev wallet 
        let balance: u64 = rpc_client .get_balance(&keypair.pubkey()) .expect("Failed to get balance");

        // Create a test transaction to calculate fees 
        let message: Message = Message::new_with_blockhash(&[transfer( &keypair.pubkey(), &to_pubkey, balance, )], Some(&keypair.pubkey()), &recent_blockhash );

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees 
        let fee: u64 = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

        let transaction: Transaction = Transaction::new_signed_with_payer( &[transfer( &keypair.pubkey(), &to_pubkey, balance - fee )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash );

        // Send the transaction 
        let signature: solana_sdk::signature::Signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed to send transaction"); 
        // Print our transaction out 
        println!( "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:"); let stdin = io::stdin(); let
        wallet =

        stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').
        split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);
    }
}
