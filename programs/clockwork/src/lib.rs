use {
    anchor_lang::prelude::*,
    clockwork_sdk::{
        self,
        state::{Thread, ThreadResponse, Trigger},
        ThreadProgram,
    },
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod contexts;
pub use contexts::*;

#[program]
pub mod clockwork {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.process()
    }
}
