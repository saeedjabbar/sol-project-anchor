//import tools from anchor
use anchor_lang::prelude::*;

//program id
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

//create calls + inheritance via marco
#[program]

//create module/class to be inherited
pub mod myepicproject {
  use super::*;
  //function 
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    Ok(())
  }
}

//another macro
#[derive(Accounts)]
pub struct StartStuffOff {}