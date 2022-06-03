use anchor_lang::prelude::*;
//use spl_token_2022::solana_zk_token_sdk::encryption::{elgamal, pedersen};
use spl_token_2022::solana_zk_token_sdk::zk_token_elgamal::{ops::{add, subtract},pod::{ ElGamalPubkey, ElGamalCiphertext}};
use std::io::{self, Write};
use std::ops::Deref;
use bytemuck::{Zeroable, Pod};
use borsh::{BorshDeserialize, BorshSerialize};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod encrypt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, user_elgamal_key: ElGamalKey, bump: u8) -> Result<()> {
        let calc = &mut ctx.accounts.calc.load_init()?;
        calc.user_elgamal_key = user_elgamal_key;
        calc.bump = bump;
        //calc.user_result_ct = ElCipher::default();
        Ok(())
    }
    
    pub fn add_cts(ctx: Context<Add>, amount1_ct: ElCipher, amount2_ct: ElCipher) -> Result<()> {
        //homomorphic addition of two ciphertexts
        let ct_0 = &amount1_ct;
        let ct_1 = &amount2_ct;
        let sum = add(ct_0, ct_1).unwrap();
        let calc_state = &mut ctx.accounts.calc.load_mut()?;
        calc_state.user_result_ct = ElCipher::from(sum);
        msg!("Resulting sum of ciphertexts: {:?}", calc_state.user_result_ct);
        Ok(())   
    }
    pub fn subtract_cts(ctx: Context<Subtract>, amount1_ct: ElCipher, amount2_ct: ElCipher) -> Result<()> {
       //homomorphic subtraction of two ciphertexts
        let difference = subtract(&amount1_ct, &amount2_ct).unwrap();
        let calc_state = &mut ctx.accounts.calc.load_mut()?;
        calc_state.user_result_ct = ElCipher::from(difference);
        msg!("Resulting difference of ciphertexts: {:?}", calc_state.user_result_ct);
        Ok(())   
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(zero)]
    calc: AccountLoader<'info ,ZkCalc>,
    #[account(mut)]
    initializer: Signer<'info>,
    system_program: Program<'info, System>,
}
#[derive(Accounts)] 
pub struct Add<'info>{  
    #[account(mut)]
    calc: AccountLoader<'info, ZkCalc>,
}
#[derive(Accounts)] 
pub struct Subtract<'info>{  
    #[account(mut)]
    calc: AccountLoader<'info, ZkCalc>,
}

#[account(zero_copy)]
#[repr(packed)]
pub struct ZkCalc{
    user_elgamal_key: ElGamalKey,
    user_result_ct: ElCipher,
    bump: u8,
} 

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct ElGamalKey(pub ElGamalPubkey);

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct ElCipher(pub ElGamalCiphertext);

impl Deref for ElGamalKey{
    type Target = ElGamalPubkey;
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}
impl Deref for ElCipher{
    type Target = ElGamalCiphertext;
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}
impl From<ElGamalCiphertext> for ElCipher{
    fn from(ct: ElGamalCiphertext) -> Self{
        ElCipher(ct)
    }
}  

unsafe impl Zeroable for ElGamalKey {}
unsafe impl Pod for ElGamalKey {}
impl AnchorSerialize for ElGamalKey {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let buf = bytemuck::bytes_of(&self.0);
        writer.write_all(buf);
        Ok(())
    }
}


impl AnchorDeserialize for ElGamalKey {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let pubkey = *bytemuck::try_from_bytes(buf).unwrap();
        Ok(ElGamalKey(ElGamalPubkey(pubkey)))
    }
}

impl AnchorSerialize for ElCipher {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let buf = bytemuck::bytes_of(&self.0);
        writer.write_all(buf);
        Ok(())
    }
}
    
impl AnchorDeserialize for ElCipher {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let cipher = *bytemuck::try_from_bytes(buf).unwrap();
        Ok(ElCipher(ElGamalCiphertext(cipher)))
    }
}

/* 
impl AccountSerialize for ElGamalKey{
    fn try_serialize<W: Write>(&self, _writer: &mut W) -> Result<()> {
        let buf = bytemuck::bytes_of(&self.0);
        _writer.write_all(buf)?;
        Ok(())
    }
}
impl AccountDeserialize for ElGamalKey{
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        let key = bytemuck::try_from_bytes(buf).unwrap();
        Ok(ElGamalKey(ElGamalPubkey(*key)))
    }
}
impl AccountSerialize for ElCipher{
    fn try_serialize<W: Write>(&self, _writer: &mut W) -> Result<()> {
        let buf = bytemuck::bytes_of(&self.0);
        _writer.write_all(buf)?;
        Ok(())
    }
}
impl AccountDeserialize for ElCipher{
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        let cipher = bytemuck::try_from_bytes(buf).unwrap();
        Ok(ElCipher(ElGamalCiphertext(*cipher)))
    }
}
*/