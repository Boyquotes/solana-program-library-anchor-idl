// This code is only used to generate the IDL for the governance program.

use anchor_lang::prelude::*;

declare_id!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

#[program]
pub mod governance {
    use super::*;

    pub fn create_realm(ctx: Context<CreateRealm>, name: String, config_args: RealmConfigArgs) -> Result<()> {
        Ok(())
    }

    pub fn deposit_governing_tokens(ctx: Context<DepositGoverningTokens>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withraw_governing_tokens(ctx: Context<WithdrawGoverningTokens>) -> Result<()> {
        Ok(())
    }

    pub fn set_governance_delegate(ctx: Context<SetGovernanceDelegate>, new_governance_delegate: Option<Pubkey>) -> Result<()> {
        Ok(())
    }

    pub fn create_governance(ctx: Context<CreateGovernance>, config: GovernanceConfig) -> Result<()> {
        Ok(())
    }

    pub fn create_program_governance(ctx: Context<CreateProgramGovernance>, config: GovernanceConfig, transfer_upgrade_authority: bool) -> Result<()> {
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, name: String, description_link: String, vote_type: VoteType, options: Vec<String>, use_deny_option: bool) -> Result<()> {
        Ok(())
    }

    pub fn add_signatory(ctx: Context<AddSignatory>, signatory: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn remove_signatory(ctx: Context<RemoveSignatory>, signatory: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn insert_transaction(ctx: Context<InsertTransaction>, option_index: u8, index: u16, hold_up_time: u32, instructions: Vec<InstructionData>) -> Result<()> {
        Ok(())
    }

    pub fn remove_transaction(ctx: Context<RemoveTransaction>) -> Result<()> {
        Ok(())
    }

    pub fn cancel_proposal(ctx: Context<CancelProposal>) -> Result<()> {
        Ok(())
    }

    pub fn sign_off_proposal(ctx: Context<SignOffProposal>) -> Result<()> {
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote: Vote) -> Result<()> {
        Ok(())
    }

    pub fn finalize_vote(ctx: Context<FinalizeVote>) -> Result<()> {
        Ok(())
    }

    pub fn relinquish_vote(ctx: Context<RelinquishVote>) -> Result<()> {
        Ok(())
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        Ok(())
    }

    pub fn create_mint_governance(ctx: Context<CreateMintGovernance>, config: GovernanceConfig, transfer_mint_authorities: bool) -> Result<()> {
        Ok(())
    }

    pub fn create_token_governance(ctx: Context<CreateTokenGovernance>, config: GovernanceConfig, transfer_account_authorities: bool) -> Result<()> {
        Ok(())
    }

    pub fn set_governance_config(ctx: Context<SetGovernanceConfig>, config: GovernanceConfig) -> Result<()> {
        Ok(())
    }

    pub fn flag_transaction_error(ctx: Context<FlagTransactionError>) -> Result<()> {
        Ok(())
    }

    pub fn set_realm_authority(ctx: Context<SetRealmAuthority>, action: SetRealmAuthorityAction) -> Result<()> {
        Ok(())
    }

    pub fn set_realm_config(ctx: Context<SetRealmConfig>, config_args: RealmConfigArgs) -> Result<()> {
        Ok(())
    }

    pub fn create_token_owner_record(ctx: Context<CreateTokenOwnerRecord>) -> Result<()> {
        Ok(())
    }

    pub fn update_program_metadata(ctx: Context<UpdateProgramMetadata>) -> Result<()> {
        Ok(())
    }

    pub fn create_native_treasury(ctx: Context<CreateNativeTreasury>) -> Result<()> {
        Ok(())
    }
}
    

#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(mut)]
    governance_realm_account: AccountInfo<'info>,
    realm_authority: AccountInfo<'info>,
    community_token_mint: AccountInfo<'info>,
    #[account(mut)]
    community_token_holding_account: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
    /* OPTIONAL ACCOUNTS **THESE DO NOT APPEAR IN THE IDL**
    council_token_mint: AccountInfo<'info>,
    #[account(mut)]
    council_token_holding_account: AccountInfo<'info>,
    community_voter_weight_id: AccountInfo<'info>,
    max_community_voter_weight_id: AccountInfo<'info>,
    #[account(mut)]
    realm_config_account: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositGoverningTokens<'info> {
    governance_realm_account: AccountInfo<'info>,
    #[account(mut)]
    governing_token_holding_account: AccountInfo<'info>,
    #[account(mut)]
    governing_token_source_account: AccountInfo<'info>,
    #[account(signer)]
    governing_token_owner: AccountInfo<'info>,
    #[account(signer)]
    governing_token_transfer_auth: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawGoverningTokens<'info> {
    governance_realm_account: AccountInfo<'info>,
    #[account(mut)]
    governing_token_holding_account: AccountInfo<'info>,
    #[account(mut)]
    governing_token_destination_account: AccountInfo<'info>,
    #[account(signer)]
    governing_token_owner: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_account: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetGovernanceDelegate<'info> {
    #[account(signer)]
    governing_token_owner: AccountInfo<'info>,
    #[account(mut)]
    token_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateGovernance<'info> {
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    account_governance: AccountInfo<'info>,
    governed_account: AccountInfo<'info>,
    governing_token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    realm_config: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    voter_weight_record: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateProgramGovernance<'info> {
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    account_governance: AccountInfo<'info>,
    governed_account: AccountInfo<'info>,
    #[account(mut)]
    program_data: AccountInfo<'info>,
    #[account(signer)]
    upgrade_auth: AccountInfo<'info>,
    governing_token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    bpf_upgradeable_loader: AccountInfo<'info>,
    system: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    realm_config: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    voter_weight_record: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    #[account(mut)]
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_account: AccountInfo<'info>,
    governing_token_mint: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
    realm_config: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    voter_weight_record: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AddSignatory<'info> {
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(mut)]
    signatory: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveSignatory<'info> {
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(mut)]
    signatory: AccountInfo<'info>,
    #[account(mut)]
    beneficiary: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InsertTransaction<'info> {
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(mut)]
    proposal_tx: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveTransaction<'info> {
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(mut)]
    proposal_tx: AccountInfo<'info>,
    #[account(mut)]
    beneficiary: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelProposal<'info> {
    #[account(mut)]
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SignOffProposal<'info> {
    #[account(mut)]
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    #[account(signer)]
    signatory: AccountInfo<'info>,
    // REQUIRED ONLY WHEN THE OWNER SIGNS OFF THE PROPOSAL
    token_owner_record_account: AccountInfo<'info>,
    /* REQUIRED ONLY WHEN NON-OWNER SIGNS OFF THE PROPOSAL
    #[account(mut)]
    signatory_record_account: AccountInfo<'info>,
    */
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_proposal: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_voter: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(mut)]
    proposal_vote_record: AccountInfo<'info>,
    governing_token_mint: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
    realm_config: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    voter_weight_record: AccountInfo<'info>,
    max_voter_weight_record: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FinalizeVote<'info> {
    #[account(mut)]
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_proposal: AccountInfo<'info>,
    governing_token_mint: AccountInfo<'info>,
    realm_config: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    max_voter_weight_record: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RelinquishVote<'info> {
    realm_account: AccountInfo<'info>,
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_vote_record: AccountInfo<'info>,
    governing_token_mint: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(mut)]
    beneficiary: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    #[account(mut)]
    proposal_tx: AccountInfo<'info>,
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateMintGovernance<'info> {
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    mint_governance: AccountInfo<'info>,
    #[account(mut)]
    governed_mint: AccountInfo<'info>,
    #[account(signer)]
    mint_auth: AccountInfo<'info>,
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
    system: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    realm_config: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    voter_weight_record: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenGovernance<'info> {
    realm_account: AccountInfo<'info>,
    #[account(mut)]
    token_governance: AccountInfo<'info>,
    #[account(mut)]
    token_account: AccountInfo<'info>,
    #[account(signer)]
    token_auth: AccountInfo<'info>,
    governing_token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    spl_token: AccountInfo<'info>,
    system: AccountInfo<'info>,
    sysvar_rent: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    realm_config: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    voter_weight_record: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetGovernanceConfig<'info> {
    realm_account: AccountInfo<'info>,
    #[account(mut, signer)]
    governance_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FlagTransactionError<'info> {
    #[account(mut)]
    proposal_account: AccountInfo<'info>,
    token_owner_record_account: AccountInfo<'info>,
    #[account(signer)]
    governance_auth: AccountInfo<'info>,
    #[account(mut)]
    proposal_tx: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetRealmAuthority<'info> {
    #[account(mut)]
    realm_account: AccountInfo<'info>,
    #[account(signer)]
    realm_authority: AccountInfo<'info>,
    new_realm_auth: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetRealmConfig<'info> {
    #[account(mut)]
    realm_account: AccountInfo<'info>,
    #[account(signer)]
    realm_authority: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    council_token_mint: AccountInfo<'info>,
    #[account(mut)]
    council_token_holding_account: AccountInfo<'info>,
    */
    system: AccountInfo<'info>,
    #[account(mut)]
    realm_config_account: AccountInfo<'info>,
    /* OPTIONAL ACCOUNT **THESE DO NOT APPEAR IN THE IDL**
    community_voter_weight_id: AccountInfo<'info>,
    max_community_voter_weight_id: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    */
    remaining_accounts: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenOwnerRecord<'info> {
    realm_account: AccountInfo<'info>,
    governing_token_owner: AccountInfo<'info>,
    #[account(mut)]
    token_owner_record_account: AccountInfo<'info>,
    governing_token_mint: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateProgramMetadata<'info> {
    #[account(mut)]
    program_metadata: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateNativeTreasury<'info> {
    governance_account: AccountInfo<'info>,
    #[account(mut)]
    native_treasury: AccountInfo<'info>,
    #[account(signer)]
    payer: AccountInfo<'info>,
    system: AccountInfo<'info>,
}
