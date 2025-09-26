use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, Mint};
use spl_token::instruction::AuthorityType;

// This declares the program ID - a unique identifier for our program on Solana
// In production, you would generate this using: anchor keys list
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFp1Jg");

#[program]
pub mod multisig_squad {
    use super::*;

    /**
     * Initialize a new multisig vault
     * Creates a multisig account with specified owners and threshold
     */
    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        owners: Vec<Pubkey>,
        threshold: u64,
        nonce: u8,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        require!(owners.len() > 0, ErrorCode::InvalidOwnersLength);
        require!(threshold > 0 && threshold <= owners.len() as u64, ErrorCode::InvalidThreshold);
        require!(owners.len() <= 10, ErrorCode::TooManyOwners); // Reasonable limit

        multisig.owners = owners;
        multisig.threshold = threshold;
        multisig.nonce = nonce;
        multisig.owner_set_seqno = 0;

        msg!("Multisig created with {} owners, threshold: {}", owners.len(), threshold);
        Ok(())
    }

    /**
     * Create a new transaction proposal
     */
    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        accs: Vec<TransactionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        let transaction = &mut ctx.accounts.transaction;

        require!(multisig.owners.contains(&ctx.accounts.proposer.key()), ErrorCode::NotOwner);

        transaction.multisig = multisig.key();
        transaction.proposer = ctx.accounts.proposer.key();
        transaction.program_id = pid;
        transaction.accounts = accs;
        transaction.data = data;
        transaction.signers = vec![false; multisig.owners.len()];
        transaction.executed = false;
        transaction.owner_set_seqno = multisig.owner_set_seqno;

        msg!("Transaction proposal created by: {}", ctx.accounts.proposer.key());
        Ok(())
    }

    /**
     * Approve a transaction proposal
     */
    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        let transaction = &mut ctx.accounts.transaction;

        require!(multisig.owners.contains(&ctx.accounts.owner.key()), ErrorCode::NotOwner);
        require!(!transaction.executed, ErrorCode::AlreadyExecuted);

        let owner_index = multisig
            .owners
            .iter()
            .position(|&owner| owner == ctx.accounts.owner.key())
            .ok_or(ErrorCode::NotOwner)?;

        require!(!transaction.signers[owner_index], ErrorCode::AlreadySigned);

        transaction.signers[owner_index] = true;

        msg!("Transaction approved by: {}", ctx.accounts.owner.key());
        Ok(())
    }

    /**
     * Execute a transaction if threshold is met
     */
    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        let transaction = &ctx.accounts.transaction;

        require!(!transaction.executed, ErrorCode::AlreadyExecuted);
        require!(transaction.owner_set_seqno == multisig.owner_set_seqno, ErrorCode::StaleTransaction);

        let approved_count = transaction.signers.iter().filter(|&&signed| signed).count();
        require!(approved_count >= multisig.threshold as usize, ErrorCode::NotEnoughSigners);

        // Execute the transaction
        msg!("Executing transaction with {} approvals", approved_count);

        // Note: In a real implementation, you would invoke the target instruction here
        // For now, we'll just mark it as executed
        ctx.accounts.transaction.executed = true;

        Ok(())
    }

    /**
     * Change the multisig owners and/or threshold
     */
    pub fn change_owners(
        ctx: Context<ChangeOwners>,
        new_owners: Vec<Pubkey>,
        new_threshold: u64,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;

        require!(multisig.owners.contains(&ctx.accounts.proposer.key()), ErrorCode::NotOwner);
        require!(new_owners.len() > 0, ErrorCode::InvalidOwnersLength);
        require!(new_threshold > 0 && new_threshold <= new_owners.len() as u64, ErrorCode::InvalidThreshold);
        require!(new_owners.len() <= 10, ErrorCode::TooManyOwners);

        // Update the multisig
        multisig.owners = new_owners;
        multisig.threshold = new_threshold;
        multisig.owner_set_seqno += 1;

        msg!("Multisig owners updated, new threshold: {}", new_threshold);
        Ok(())
    }
}

// Account structures for multisig
#[derive(Accounts)]
#[instruction(owners: Vec<Pubkey>, threshold: u64, nonce: u8)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        seeds = [b"multisig", create_key.key().as_ref()],
        bump = nonce,
        payer = create_key,
        space = 8 + 32 + 4 + 8 + 1 + 1 + 200  // discriminator + multisig key + owners len + threshold + nonce + owner_set_seqno + owners array
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub create_key: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(pid: Pubkey, accs: Vec<TransactionAccount>, data: Vec<u8>)]
pub struct CreateTransaction<'info> {
    #[account(mut, has_one = proposer)]
    pub multisig: Account<'info, Multisig>,

    #[account(
        init,
        seeds = [b"transaction", multisig.key().as_ref(), &data],
        bump,
        payer = proposer,
        space = 8 + 32 + 32 + 32 + 4 + 8 + 8 + 1 + 200  // discriminator + multisig + proposer + program_id + accounts len + data len + signers len + executed + arrays
    )]
    pub transaction: Account<'info, Transaction>,

    pub proposer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(mut, has_one = multisig)]
    pub transaction: Account<'info, Transaction>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(mut, has_one = multisig)]
    pub transaction: Account<'info, Transaction>,

    pub executor: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(new_owners: Vec<Pubkey>, new_threshold: u64)]
pub struct ChangeOwners<'info> {
    #[account(mut, has_one = proposer)]
    pub multisig: Account<'info, Multisig>,

    pub proposer: Signer<'info>,
}

// Data structures
#[account]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub nonce: u8,
    pub owner_set_seqno: u64,
}

#[account]
pub struct Transaction {
    pub multisig: Pubkey,
    pub proposer: Pubkey,
    pub program_id: Pubkey,
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
    pub signers: Vec<bool>,
    pub executed: bool,
    pub owner_set_seqno: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid owners length")]
    InvalidOwnersLength,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Not enough signers")]
    NotEnoughSigners,
    #[msg("Already executed")]
    AlreadyExecuted,
    #[msg("Already signed")]
    AlreadySigned,
    #[msg("Not an owner")]
    NotOwner,
    #[msg("Stale transaction")]
    StaleTransaction,
    #[msg("Too many owners")]
    TooManyOwners,
}
