pub struct Emu {
    cpu: Cpu,
    cart: Cart,
}

impl Emu {
    fn new(cart : Cart) -> Self {
        Self {
            cpu: Cpu {},
        }
    }
}