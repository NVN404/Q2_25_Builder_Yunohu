use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Manager {
    pub bump: u8,
}
