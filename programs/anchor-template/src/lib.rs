use anchor_lang::prelude::*;

declare_id!("4UbeHWCbfqHE8fM4ykCZK33ZcFUYijrk1dCK5jjtNKMY");

#[program]
pub mod anchor_template {
    use super::*;
    pub fn initialize(ctx: Context<InitializeCounter>) -> ProgramResult {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = 0;
        Ok(())
    }

    pub fn add(ctx: Context<AddCounter>) -> ProgramResult {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = user, space = 9000)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddCounter<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[account]
pub struct CounterAccount {
    pub counter: u64,
}

/*
TEMPLATE TO CREATE AN "ENDPOINT"

...
    fn doSome(ctx: ...<DoSome>) {
        let cuenta = &mut ctx.accounts.cuenta
        
        ...
    }
...

#[derive(Accounts)]
pub struct DoSome<'info> {
    #[account(mut)]
    pub cuenta: Account<'info, Cuenta>,
}


*/
