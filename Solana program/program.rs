use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer,Mint};
use solana_program::system_instruction;
use std::str::FromStr;

declare_id!("GJYXo5SKM1yX6f4QV8dWY1FVuTVbQyMdfvXdcxuRf25A");
const ADMIN_KEY: &str = "7RKq7LXmNwBX4M3E4TnxbtsGBM2k99795mycB35tWqSy";
#[program]
mod new_website {
    use super::*;
    pub fn add_website(ctx: Context<Initialize>, url: String, probability: u8,whitelist: bool,url_reasons: String,domain_age_reasons: String,javascript_code_reasons: String,site_content_reasons: String) -> Result<()> {
        let website = &mut ctx.accounts.website;
        website.url = url;
        website.probability = probability;
        website.whitelist = whitelist;
        website.url_reasons = url_reasons;
        website.domain_age_reasons = domain_age_reasons;
        website.javascript_code_reasons = javascript_code_reasons;
        website.site_content_reasons = site_content_reasons;
       Ok(())
    }
    pub fn create_vote(ctx: Context<InitializeVote>,amount: u64,v: bool) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let website = &mut ctx.accounts.website;
        let base: u64 = 2;
        let minimum: u64
        if website.whitelist == true {
            let minimum: u64 = base.pow(website.probability.into()) * 1000;
        }else{
            let minimum: u64 = base.pow(1-website.probability.into()) * 1000;
        }
        let minimum: u64 = base.pow(website.probability.into()) * 1000;
        require!(amount>=minimum,MyError::MinimumNotMet);
        let transfer_instruction;
        if v {
        transfer_instruction = Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.for_account.to_account_info(),
                authority: ctx.accounts.token_account_owner_pda.to_account_info(),
            };
        } else{
        transfer_instruction = Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.against_account.to_account_info(),
                authority: ctx.accounts.token_account_owner_pda.to_account_info(),
            };
        }
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
    pub fn add_vote(ctx: Context<InitializeVote>,amount: u64,v: bool){
        let vote = &mut ctx.accounts.vote;
        let website = &mut ctx.accounts.website;
        let base: u64 = 2;
        let minimum: u64 = base.pow(website.probability.into()) * 1000;
        require!(amount >= minimum, MyError::MinimumNotMet);

        let transfer_instruction;
        if v {
            transfer_instruction = Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.for_account.to_account_info(),
                authority: ctx.accounts.token_account_owner_pda.to_account_info(),
            };
        } else {
            transfer_instruction = Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.against_account.to_account_info(),
                authority: ctx.accounts.token_account_owner_pda.to_account_info(),
            };
        }

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        Ok(())
        }
    pub fn finalize_vote(ctx: Context<FinalizeVote>) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let website = &mut ctx.accounts.website;
        let for_account = &ctx.accounts.for_account;
        let against_account = &ctx.accounts.against_account;
        let admin_account = &ctx.accounts.admin_account;

        // Check if the vote is still active
        require!(
            vote.status == VoteStatus::Active,
            MyError::VoteNotActive
        );

        // Check if the current time is after the end time
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= vote.end_time,
            MyError::VoteNotEnded
        );

        // Calculate the vote result
        let for_balance = for_account.amount;
        let against_balance = against_account.amount;

        let (result, winning_side, losing_side) = if for_balance > against_balance {
            website.whitelist = true;
            (
                VoteResult::WhitelistSuccessful,
                for_account.to_account_info(),
                against_account.to_account_info(),
            )
        } else {
            website.whitelist = false;
            (
                VoteResult::BlacklistSuccessful,
                against_account.to_account_info(),
                for_account.to_account_info(),
            )
        };

        // Set the vote status to inactive
        vote.status = VoteStatus::Inactive;

        // Transfer tokens
        let winning_side_data = TokenAccount::unpack(&winning_side.try_borrow_data()?)?;
        let losing_side_data = TokenAccount::unpack(&losing_side.try_borrow_data()?)?;

        let total_tokens = winning_side_data.amount + losing_side_data.amount;
        let burn_amount = total_tokens / 10; // 10% of total tokens burned
        let admin_amount = total_tokens / 20; // 5% of total tokens to admin
        let winning_side_amount = total_tokens - burn_amount - admin_amount;

        // Burn tokens
        let burn_instruction = Burn {
            mint: winning_side.mint.to_account_info(),
            source: winning_side.to_account_info(),
            authority: ctx.accounts.token_account_owner_pda.to_account_info(),
            amount: burn_amount,
        };
        let cpi_ctx_burn = CpiContext::new(ctx.accounts.token_program.to_account_info(), burn_instruction);
        anchor_spl::token::burn(cpi_ctx_burn, burn_amount)?;

        // Transfer tokens to admin
        let transfer_to_admin_instruction = Transfer {
            from: winning_side.to_account_info(),
            to: admin_account.to_account_info(),
            authority: ctx.accounts.token_account_owner_pda.to_account_info(),
        };
        let cpi_ctx_admin = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_admin_instruction);
        anchor_spl::token::transfer(cpi_ctx_admin, admin_amount)?;

        // Transfer tokens to winning side
        transfer_tokens_to_voters(
            &ctx.accounts.token_program,
            &winning_side,
            &winning_side_data,
            &ctx.accounts.token_account_owner_pda,
            winning_side_amount,
        )?;

        emit!(VoteFinalized {
            result,
            for_balance,
            against_balance,
            burn_amount,
            admin_amount,
            winning_side_amount,
        });

        Ok(())
    }

fn transfer_tokens_to_voters(
    token_program: &Program<'_, Token>,
    winning_side_account: &AccountInfo,
    winning_side_data: &TokenAccount,
    token_account_owner_pda: &AccountInfo,
    amount: u64,
) -> Result<()> {
    let mut voters = winning_side_data.token_amount_total_minted.iter();

    while let Some((voter_address, _)) = voters.next() {
        let voter_token_account = get_associated_token_address(&voter_address, &winning_side_account.mint);
        let transfer_instruction = Transfer {
            from: winning_side_account.to_account_info(),
            to: voter_token_account,
            authority: token_account_owner_pda.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_instruction);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
    }

    Ok(())
}

#[event]
pub struct VoteFinalized {
    result: VoteResult,
    for_balance: u64,
    against_balance: u64,
    burn_amount: u64,
    admin_amount: u64,
    winning_side_amount: u64,
}

}

#[derive(Accounts)]
#[instruction(url: String,reason: String)]
pub struct Initialize<'info> {
    #[account(init_if_needed,payer = signer,space = 1000,seeds = [url.as_bytes().as_ref()],bump)]
    pub website: Account<'info, Website>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}
#[derive(Accounts)]
#[instruction(url: String,reason: String)]
pub struct InitializeVote<'info> {
    #[account(init_if_needed,payer = signer,space = 1000,seeds = [url.as_bytes().as_ref(),b"vote"],bump)]
    pub vote: Account<'info,Vote>,
    #[account(init_if_needed,payer = signer,space = 1000,seeds = [url.as_bytes().as_ref()],bump)]
    pub website: Account<'info, Website>,
    #[account(init_if_needed,payer = signer,seeds=[b"token_account_owner_pda",url.as_bytes().as_ref()],bump,space = 8)]
    token_account_owner_pda: AccountInfo<'info>,
    #[account(init_if_needed,payer = signer,seeds=[b"for", mint_of_token_being_sent.key().as_ref()],token::mint=mint_of_token_being_sent,token::authority=token_account_owner_pda,bump)]
    pub for_account: Account<'info, TokenAccount>,
    #[account(init_if_needed,payer = signer,seeds=[b"against", mint_of_token_being_sent.key().as_ref()],token::mint=mint_of_token_being_sent,token::authority=token_account_owner_pda,bump)]
    pub against_account: Account<'info, TokenAccount>,
    #[account(mut)]
    sender_token_account: Account<'info, TokenAccount>,
    pub mint_of_token_being_sent: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}
#[account]
pub struct Website {
    pub url: String,
    pub reason: String,
    pub probability: u8,
    pub whitelist: bool,    
    pub url_reasons: String,
    pub domain_age_reasons: String,
    pub javascript_code_reasons: String,
    pub site_content_reasons: String
}

#[account]
pub struct Vote {
    pub startTime: i64,
    pub endTime: i64,
    pub initial_balance: u64,
    pub status: VoteStatus
}



#[error_code]
pub enum MyError {
    #[msg("Unauthorized admin operation attempt.")]
    UnauthorizedAdmin,
    #[msg("Minimum not met")]
    MinimumNotMet
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum VoteType {
    BlacklistToWhitelist,
    WhiteListToBlacklist
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum VoteStatus {
    Active,
    Inactive,
}