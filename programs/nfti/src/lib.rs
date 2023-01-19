use anchor_lang::prelude::*;


mod account;
mod context;

use context::*;
use account::*;


declare_id!("2LSc752NepdABk6TtHLTLFjVcUuYXkMj4dtaViuN8koZ");

#[program]
pub mod nfti {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

/*
    Given a MINT ID
    Create a Switchboard Job

*/