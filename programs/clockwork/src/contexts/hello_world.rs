use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct HelloWorld {}

impl<'info> HelloWorld {
    pub fn process(&mut self) -> Result<()> {
        msg!("Hello World!");
        Ok(())
    }
}
