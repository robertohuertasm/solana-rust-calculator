use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

#[program]
pub mod solana_calculator {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn op(ctx: Context<Operation>, op: Ops, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        let (result, remainder) = match op {
            Ops::Add => (num1 + num2, 0),
            Ops::Sub => (num1 - num2, 0),
            Ops::Mul => (num1 * num2, 0),
            Ops::Div => (num1 / num2, num1 % num2),
        };
        calculator.result = result;
        calculator.remainder = remainder;
        Ok(())
    }

    pub fn add(ctx: Context<Operation>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = i64::saturating_add(num1, num2);
        Ok(())
    }

    pub fn multiply(ctx: Context<Operation>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = i64::saturating_mul(num1, num2);
        Ok(())
    }

    pub fn subtract(ctx: Context<Operation>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = i64::saturating_sub(num1, num2);
        Ok(())
    }

    pub fn divide(ctx: Context<Operation>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Operation<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
