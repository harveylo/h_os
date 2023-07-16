use x86_64::instructions::port::Port;


pub struct PIT {
    interval: u16,
    cmd_port: Port<u8>,
    channel0: Port<u8>,
}

impl PIT {
    pub fn new(int: u16) -> PIT{
        PIT{interval: int, cmd_port: Port::new(0x43), channel0: Port::new(0x40)}
    }

    pub unsafe fn init(&mut self){
       self.cmd_port.write(0x36);
       let div = 1193182 / (self.interval as u32);
       let low_bits = (div & 0xff) as u8;
       Port::<u8>::new(0x80).write(0x80);
       let high_bits = ((div >>8)&0xff) as u8;
       self.channel0.write(low_bits);
       self.channel0.write(high_bits);
    }
}