use anchor_lang::prelude::*;

#[account]
pub struct Data {
    pub udata: u128,
    pub idata: i128,
}

#[account]
#[derive(Default)]
pub struct DataU16 {
    pub data: u16,
}

#[account]
pub struct DataI8 {
    pub data: i8,
}

#[account]
pub struct DataI16 {
    pub data: i16,
}

#[account(zero_copy)]
#[derive(Default)]
pub struct DataZeroCopy {
    pub data: u16,
    pub bump: u8,
}
