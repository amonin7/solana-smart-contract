use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod contract {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
        Ok(())
    }

    pub fn another_simple_function(ctx: Context<AnotherSimpleFunction>) -> Result <()> {
        println!("hello there!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff {}

#[derive(Accounts)]
pub struct AnotherSimpleFunction {}