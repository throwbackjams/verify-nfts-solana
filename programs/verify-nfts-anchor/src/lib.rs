use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};
use mpl_token_metadata;
use mpl_token_metadata::state::{Metadata, PREFIX, EDITION};
use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod verify_nfts_anchor {
    use super::*;

    pub fn initialize(ctx: Context<VerifyNFT>) -> Result<()> {
        
        let nft_token_account = &ctx.accounts.nft_token_account;
        let user = &ctx.accounts.user;
        let nft_mint_account = &ctx.accounts.nft_mint;

        assert_eq!(nft_token_account.owner, user.key());
        assert_eq!(nft_token_account.mint, nft_mint_account.key());
        assert_eq!(nft_token_account.amount, 1);

        //We expect a Metaplex Master Edition so we derive it given mint as seeds
        //Then compare to the Mint account passed into the program

        let master_edition_seed = &[
            PREFIX.as_bytes(),
            ctx.accounts.token_metadata_program.key.as_ref(),
            nft_token_account.mint.as_ref(),
            EDITION.as_bytes()
        ];

        let (master_edition_key, _master_edition_seed) =
            Pubkey::find_program_address(master_edition_seed, ctx.accounts.token_metadata_program.key);
        
        assert_eq!(master_edition_key, ctx.accounts.nft_mint.key());

        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let nft_mint_account_pubkey = &ctx.accounts.nft_mint.key();

        let metadata_seed = &[
            "metadata".as_bytes(),
            ctx.accounts.token_metadata_program.key.as_ref(),
            nft_mint_account_pubkey.as_ref(),
        ];

        let (metadata_derived_key, _bump_seed) =
            Pubkey::find_program_address(
                metadata_seed,
                ctx.accounts.token_metadata_program.key
            );
        //check that derived key is the current metadata account key
        assert_eq!(metadata_derived_key, nft_metadata_account.key());

        if ctx.accounts.nft_metadata_account.data_is_empty() {
            return Err(Errors::NotInitialized.into());
        };

        //Get the metadata account struct so we can access its values
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
        
        let full_metadata_clone = metadata_full_account.clone();

        let expected_creator =
            Pubkey::from_str("BuSmTfRJFB7ewseydjbC8DaRYYuhPBPLGyeK7cxNLx1k").unwrap();
            //solana_program::pubkey!("BuSmTfRJFB7ewseydjbC8DaRYYuhPBPLGyeK7cxNLx1k");
        
            
        //Verify creator is present in metadata
        //NOTE: The first address in 'creators' is the Candy Machine Address
        // Therefore, the expected_creator should be the Candy Machine Address here
        //NOTE: May want to use updateAuthority field if CMA is not known in advance?
        assert_eq!(
            full_metadata_clone.data.creators.as_ref().unwrap()[0].address,
            expected_creator
        );

        //check if creator is verified
        if !full_metadata_clone.data.creators.unwrap()[0].verified {
            //return error as creator is not verified 
            return Err(Errors::CreatorNotVerified.into());
        };
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyNFT<'info> {
    //owner of NFT
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    pub nft_token_account: Account<'info, TokenAccount>,
    pub nft_metadata_account: AccountInfo<'info>,
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,
}

#[error_code]
pub enum Errors {
    #[msg("Not Initialized")]
    NotInitialized,
    #[msg("Creator is not verified")]
    CreatorNotVerified,
}