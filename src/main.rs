#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Write;
use heapless::String;



use riscv_rt::entry;
use longan_nano::hal::{pac, prelude::*, pac::*, eclic::*, adc::*};
use longan_nano::hal::delay::McycleDelay;
use longan_nano::{lcd, lcd_pins};
use embedded_graphics::mono_font::{
    ascii::FONT_7X14,
    MonoTextStyleBuilder,
};

use gd32vf103xx_hal::pac::Interrupt;
use panic_halt as _;
use gd32vf103xx_hal::timer;
use gd32vf103xx_hal::timer::Timer;


use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Rectangle, PrimitiveStyle};
use embedded_graphics::text::Text;

static mut G_TIMER1: Option<Timer<TIMER1>> = None;
static mut G_ADC0: Option<Adc<ADC0>> = None;
static mut G_PMU: Option<gd32vf103xx_hal::pac::PMU> = None;

//shared data
static mut G_TEMP_VAL :i32 = 0;


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    
    
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    unsafe { G_PMU = Some(dp.PMU); };

    //===================================== ADC =====================================

    let adc= Adc::adc0(dp.ADC0, &mut rcu);
    unsafe {G_ADC0 = Some(adc)};    

    //============================= ECLIC & TIMER conf ==============================

    ECLIC::reset();
    ECLIC::set_threshold_level(Level::L0);
    ECLIC::set_level_priority_bits(LevelPriorityBits::L3P1);

    // init tiemer 1
    let mut timer =  Timer::timer1(dp.TIMER1, 1.hz(), &mut rcu);
    timer.listen(timer::Event::Update); //start the timer ?
    unsafe {G_TIMER1 = Some(timer)};


    // configure the timer1 interrupts in the ECLIC
    ECLIC::setup(
        Interrupt::TIMER1,
        TriggerType::RisingEdge,
        Level::L1,
        Priority::P1,
    );
    //unmask timer1 interrupt and enable global interrupts
    unsafe { 
        ECLIC::unmask(Interrupt::TIMER1);
        riscv::interrupt::enable();
    };
    
    //===================================== LCD =====================================
        let lcd_pins = lcd_pins!(gpioa, gpiob);
        let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
        let (width, height) = (lcd.size().width, lcd.size().height);
        // Clear screen
        Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut lcd)
            .unwrap();
    
    
        lcd_print(&mut lcd, 5, 10, "challenge 3 accepted !");
        lcd_print(&mut lcd, 5, 50, "TEMP: ");
        lcd_print(&mut lcd, 5, 70, "challenge 3 DONE !");
        
    //=========================== Displaying something  =============================

    let mut delay = McycleDelay::new(&rcu.clocks);
    let progress: [&str; 6] = [
        "[     ]",
        "[#    ]",
        "[##   ]",
        "[###  ]",
        "[#### ]",
        "[#####]" 
        ];
        
    let mut data = String::<20>::from("");
    
    //magic happens here !
    loop { 
        // dispay AWAKE status
        lcd_print(&mut lcd, 60, 30, "AWAKE ");
        
        // show progress bar animation then sleep untill an interrupt wakes CPU again
        for i in 0..6 {
            unsafe { let _=write!(data,"{}", G_TEMP_VAL); } //display shared temp val
            lcd_print(&mut lcd, 5, 30, progress[i]);
            lcd_print(&mut lcd, 55, 50, data.as_str());
            delay.delay_ms(20);
            data.clear();
        }
        
        //display ASLEEP status and put MCU in standby mode
        lcd_print(&mut lcd, 60, 30, "ASLEEP");
        standby(); //SLEEP

    }
}




#[allow(non_snake_case)]
#[no_mangle]
fn TIMER1() {
    unsafe {
        if let Some(timer1) = G_TIMER1.as_mut() {
            timer1.clear_update_interrupt_flag();
        }

        //reading tempreture value from ADC in a safe manner
        if let Some(adc) = G_ADC0.as_mut() {
            riscv::interrupt::free(|_|{
                G_TEMP_VAL = adc.read_temp();
            });
        }
    }
}






fn lcd_print(lcd: &mut lcd::Lcd, xpos : i32, ypos: i32, text: &str) {

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_7X14)
        .text_color(Rgb565::WHITE)
        .background_color(Rgb565::BLACK)
        .build();
    

    // Create a text at position (x, y) and draw it using style defined above
    Text::new(text, Point::new(xpos, ypos), style)
        .draw (lcd)
        .unwrap();

        
}

fn standby(){

    unsafe {
        if let Some(pmu) = G_PMU.as_mut() {
            //config power managment unit to enter standby when wfi is called
            pmu.ctl.write(|w| w.stbmod().set_bit());
            pmu.ctl.write(|w| w.wurst().set_bit());
            
            //put the device in sleep mode using riscv instruction
            riscv::asm::wfi(); 
        }
    }
}