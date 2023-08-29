#[derive(Debug)]
#[repr(u8)]
pub enum Op {
    LInt(i16),
    LConst(u16),
    BAdd,
    BSub,
    BMul,
    BDiv,
    UMinus,
}

const _: () = {
    let _ = std::mem::transmute::<Op, u32>;
};
