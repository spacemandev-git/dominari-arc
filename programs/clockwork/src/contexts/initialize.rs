use anchor_lang::solana_program::instruction::Instruction;

use {
    anchor_lang::prelude::*,
    clockwork_sdk::{
        self,
        state::{Thread, Trigger},
        ThreadProgram,
    },
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(address = Thread::pubkey(payer.key(),"dominari_thread".to_string()))]
    pub dominari_thread: SystemAccount<'info>,
    pub payer: Signer<'info>,
    #[account(address = ThreadProgram::id())]
    pub thread_program: Program<'info, ThreadProgram>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self {
            payer,
            thread_program,
            system_program,
            dominari_thread,
            ..
        } = self;

        let hello_world_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![],
            data: clockwork_sdk::utils::anchor_sighash("hello_world").into(),
        };

        clockwork_sdk::cpi::thread_create(
            CpiContext::new(
                thread_program.to_account_info(),
                clockwork_sdk::cpi::ThreadCreate {
                    authority: payer.to_account_info(),
                    payer: payer.to_account_info(),
                    system_program: system_program.to_account_info(),
                    thread: dominari_thread.to_account_info(),
                },
            ),
            "subscriber_thread".to_string(),
            hello_world_ix.into(),
            Trigger::Cron {
                schedule: "*/10 * * * * * *".to_string(),
                skippable: false,
            },
        )?;
        Ok(())
    }
}
