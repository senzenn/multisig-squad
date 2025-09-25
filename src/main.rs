use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer, EncodableKey},
    transaction::Transaction,
    instruction::Instruction,
};
use std::str::FromStr;
use anchor_lang::{InstructionData, ToAccountMetas};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// RPC URL
    #[arg(long, default_value = "http://localhost:8899")]
    rpc_url: String,

    /// Keypair path
    #[arg(long, default_value = "~/.config/solana/id.json")]
    keypair_path: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello
    Hello,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let rpc_client = RpcClient::new_with_commitment(
        cli.rpc_url.clone(),
        CommitmentConfig::confirmed(),
    );

    // Load keypair
    let keypair_path = shellexpand::tilde(&cli.keypair_path).into_owned();
    let keypair = Keypair::read_from_file(keypair_path)?;

    match cli.command {
        Commands::Hello => {
            say_hello(&rpc_client, &keypair).await?;
        }
    }

    Ok(())
}

async fn say_hello(
    rpc_client: &RpcClient,
    keypair: &Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFp1Jg")?;

    // Create instruction
    let accounts = multisig_squad::accounts::Hello {
        user: keypair.pubkey(),
    };

    let instruction = Instruction {
        program_id,
        accounts: accounts.to_account_metas(Some(true)),
        data: multisig_squad::instruction::Hello {}.data(),
    };

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Hello executed successfully!");
    println!("📋 Signature: {}", signature);
    println!("👤 Called by: {}", keypair.pubkey());

    Ok(())
}