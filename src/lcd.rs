use core::any::Any;
use esp32c3::io_mux::GPIO;
use hal::{clock::ClockControl, Delay, peripherals, peripherals::Peripherals, prelude::*};
use hal::gpio::{BankGpioRegisterAccess, GpioPin, InteruptStatusRegisterAccess, Output, PushPull};
use hal::peripherals::GPIO_SD;

struct  LCD{
    /*sbit CS=P2^2;		//片选
    sbit RES=P2^1;		//复位
    sbit RS=P2^4;		//数据/命令选择
    sbit RW=P2^5;*/
    //CS:GpioPin<Output<PushPull>,BankGpioRegisterAccess, InteruptStatusRegisterAccess, IRA, PINTYPE, SIG, const GPIONUM: u8>
    CS:GPIO, //片选
    RES:GPIO, //复位
    RS:GPIO, //数据/命令选择
    RW:GPIO,

}

impl LCD {
    //
    fn ILI9325_Initial(&self)
    {
        self.delayms(50);                     //根据不同晶振速度可以调整延时，保障稳定显示
        self.Write_Cmd_Data(0x0001, 0x0100);
        self.Write_Cmd_Data(0x0002, 0x0700);
        self.Write_Cmd_Data(0x0003, 0x1030);
        self.Write_Cmd_Data(0x0004, 0x0000);
        self.Write_Cmd_Data(0x0008, 0x0207);
        self.Write_Cmd_Data(0x0009, 0x0000);
        self.Write_Cmd_Data(0x000A, 0x0000);
        self.Write_Cmd_Data(0x000C, 0x0000);
        self.Write_Cmd_Data(0x000D, 0x0000);
        self.Write_Cmd_Data(0x000F, 0x0000);
//power on sequence VGHVGL
        self.Write_Cmd_Data(0x0010, 0x0000);
        self.Write_Cmd_Data(0x0011, 0x0007);
        self.Write_Cmd_Data(0x0012, 0x0000);
        self.Write_Cmd_Data(0x0013, 0x0000);
//vgh
        self.Write_Cmd_Data(0x0010, 0x1290);
        self.Write_Cmd_Data(0x0011, 0x0227);
//delayms(100);
//vregiout
        self.Write_Cmd_Data(0x0012, 0x001d); //0x001b
//delayms(100);
//vom amplitude
        self.Write_Cmd_Data(0x0013, 0x1500);
//delayms(100);
//vom H
        self.Write_Cmd_Data(0x0029, 0x0018);
        self.Write_Cmd_Data(0x002B, 0x000D);

//gamma
        self.Write_Cmd_Data(0x0030, 0x0004);
        self.Write_Cmd_Data(0x0031, 0x0307);
        self.Write_Cmd_Data(0x0032, 0x0002);// 0006
        self.Write_Cmd_Data(0x0035, 0x0206);
        self.Write_Cmd_Data(0x0036, 0x0408);
        self.Write_Cmd_Data(0x0037, 0x0507);
        self.Write_Cmd_Data(0x0038, 0x0204);//0200
        self.Write_Cmd_Data(0x0039, 0x0707);
        self.Write_Cmd_Data(0x003C, 0x0405);// 0504
        self.Write_Cmd_Data(0x003D, 0x0F02);
//ram
        self.Write_Cmd_Data(0x0050, 0x0000);
        self.Write_Cmd_Data(0x0051, 0x00EF);
        self.Write_Cmd_Data(0x0052, 0x0000);
        self.Write_Cmd_Data(0x0053, 0x013F);
        self.Write_Cmd_Data(0x0060, 0xA700);
        self.Write_Cmd_Data(0x0061, 0x0001);
        self.Write_Cmd_Data(0x006A, 0x0000);
//
        self.Write_Cmd_Data(0x0080, 0x0000);
        self.Write_Cmd_Data(0x0081, 0x0000);
        self.Write_Cmd_Data(0x0082, 0x0000);
        self.Write_Cmd_Data(0x0083, 0x0000);
        self.Write_Cmd_Data(0x0084, 0x0000);
        self.Write_Cmd_Data(0x0085, 0x0000);
//
        self.Write_Cmd_Data(0x0090, 0x0010);
        self.Write_Cmd_Data(0x0092, 0x0600);
        self.Write_Cmd_Data(0x0093, 0x0003);
        self.Write_Cmd_Data(0x0095, 0x0110);
        self.Write_Cmd_Data(0x0097, 0x0000);
        self.Write_Cmd_Data(0x0098, 0x0000);
        self.Write_Cmd_Data(0x0007, 0x0133);


//	Write_Cmd_Data(0x0022);//
    }



    fn Write_Cmd_Data(&self,x: u8, y: u16)
    {
        let m: u8;
        let n: u8;
        m = (y >> 8) as u8;
        n = y as u8;
        self.Write_Cmd(0x00, x);
        self. Write_Data(m, n);
    }
    fn Write_Cmd(&self, DH:u8, DL:u8)
    {
         self.CS.write(|w| unsafe { w.bits(0) })
        /*CS = 0;
        RS = 0;

        P0 = DH;
        RW = 0;
        RW = 1;

        P0 = DL;

        RW = 0;
        RW = 1;
        CS = 1;*/
    }
    fn Write_Data(&self, DH:u8, DL:u8)
    {

/*

    CS=0;

    RS=1;
    P0=DH;
    RW=0;
    RW=1;

    P0=DL;
    RW=0;
    RW=1;
    CS=1;*/
    }


    fn delayms(&self,count: u32)
    {
        let peripherals = Peripherals::take();
        let mut system = peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        let mut delay = Delay::new(&clocks);
        delay.delay_ms(count);
        /*  for i in 0..count {
        for j in 0..260 {

        }
    }*/
    }
}

