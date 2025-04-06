use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelAmmInstruction {
    CreatePool = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreatePool {}

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Pod, Zeroable)]
// pub struct Add {
//     pub amount: [u8; 8]
// }

instruction!(SteelAmmInstruction, CreatePool);
// instruction!(SteelAmmInstruction, Add);
