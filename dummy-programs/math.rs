// This code is only used to generate the IDL for the math library

use anchor_lang::prelude::*;

declare_id!("Math111111111111111111111111111111111111111");

#[program]
pub mod math {
    use super::*;

    pub fn precise_square_root(ctx: Context<PreciseSquareRoot>, radicand: u64) -> Result<()> {
        Ok(())
    }

    pub fn square_root_u64(ctx: Context<SquareRootU64>, radicand: u64) -> Result<()> {
        Ok(())
    }

    pub fn square_root_u128(ctx: Context<SquareRootU128>, radicand: u128) -> Result<()> {
        Ok(())
    }

    pub fn u64_multiply(ctx: Context<U64Multiply>, multiplicand: u64, multiplier: u64) -> Result<()> {
        Ok(())
    }

    pub fn u64_divide(ctx: Context<U64Divide>, dividend: u64, divisor: u64) -> Result<()> {
        Ok(())
    }

    pub fn f32_multiply(ctx: Context<F32Multiply>, multiplicand: f32, multiplier: f32) -> Result<()> {
        Ok(())
    }

    pub fn f32_divide(ctx: Context<F32Divide>, dividend: f32, divisor: f32) -> Result<()> {
        Ok(())
    }

    pub fn f32_exponentiate(ctx: Context<F32Exponentiate>, base: f32, exponent: f32) -> Result<()> {
        Ok(())
    }

    pub fn f32_natural_log(ctx: Context<F32NaturalLog>, argument: f32) -> Result<()> {
        Ok(())
    }

    pub fn f32_normal_cdf(ctx: Context<F32NormalCDF>, argument: f32) -> Result<()> {
        Ok(())
    }

    pub fn noop(ctx: Context<Noop>) -> Result<()> {
        Ok(())
    }

}


#[derive(Accounts)]
pub struct PreciseSquareRoot<'info> {}

#[derive(Accounts)]
pub struct SquareRootU64<'info> {}

#[derive(Accounts)]
pub struct SquareRootU128<'info> {}

#[derive(Accounts)]
pub struct U64Multiply<'info> {}

#[derive(Accounts)]
pub struct U64Divide<'info> {}

#[derive(Accounts)]
pub struct F32Multiply<'info> {}

#[derive(Accounts)]
pub struct F32Divide<'info> {}

#[derive(Accounts)]
pub struct F32Exponentiate<'info> {}

#[derive(Accounts)]
pub struct F32NaturalLog<'info> {}

#[derive(Accounts)]
pub struct F32NormalCDF<'info> {}

#[derive(Accounts)]
pub struct Noop<'info> {}
