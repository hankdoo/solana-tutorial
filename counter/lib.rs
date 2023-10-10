use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2sqNTobLRXDtMZxemNY3HXWanz1aZAGnmQH2f35saPei");

#[program]
mod hello_world {
    use super::*;

    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        let counter = &mut _ctx.accounts.counter;
        counter.count += 1;
        msg!("Hello World - Greeting {}!", counter.count);
        Ok(())
    }

    pub fn initialze_counter(_ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut _ctx.accounts.counter;
        counter.count = 0;
        msg!("Initialize new count. Current value: {}!", counter.count);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct SayHello<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    count: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=signer,space=8+8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

