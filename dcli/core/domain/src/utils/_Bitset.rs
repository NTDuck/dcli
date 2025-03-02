macro_rules! bitset {
    () => {
        
    };
}

pub struct Bitset<const N: usize> {
    bits: [u8; N],
}

impl<const N: usize> Bitset<N> {
    pub const fn new() -> Self {
        todo!()
    }
}
