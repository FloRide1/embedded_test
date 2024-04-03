#![no_main]
#![no_std]
#![allow(dead_code)]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::gpio;
use stm32f4xx_hal::gpio::Output;
use stm32f4xx_hal::gpio::PinState;
use stm32f4xx_hal::gpio::PushPull;
use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::qei::Qei;

// LEDs
enum LedsEnum {
    LedUser1,
    LedUser2,
    LedUser3,

    LedGreen1,
    LedGreen2,

    LedRed1,
    LedRed2,

    LedOrange1,
    LedOrange2,

    LedBlue1,
    LedBlue2,

    LedWhite1,
    LedWhite2,
    LedWhite3,
}

type LedUser1 = gpio::PB0<Output<PushPull>>;
type LedUser2 = gpio::PB7<Output<PushPull>>;
type LedUser3 = gpio::PB14<Output<PushPull>>;

type LedGreen1 = gpio::PF6<Output<PushPull>>;
type LedGreen2 = gpio::PF0<Output<PushPull>>;

type LedRed1 = gpio::PF7<Output<PushPull>>;
type LedRed2 = gpio::PF1<Output<PushPull>>;

type LedOrange1 = gpio::PF8<Output<PushPull>>;
type LedOrange2 = gpio::PF2<Output<PushPull>>;

type LedBlue1 = gpio::PF9<Output<PushPull>>;
type LedBlue2 = gpio::PF3<Output<PushPull>>;

type LedWhite1 = gpio::PF10<Output<PushPull>>;
type LedWhite2 = gpio::PF4<Output<PushPull>>;
type LedWhite3 = gpio::PF5<Output<PushPull>>;

static G_LED_USER_1: Mutex<RefCell<Option<LedUser1>>> = Mutex::new(RefCell::new(None));
static G_LED_USER_2: Mutex<RefCell<Option<LedUser2>>> = Mutex::new(RefCell::new(None));
static G_LED_USER_3: Mutex<RefCell<Option<LedUser3>>> = Mutex::new(RefCell::new(None));

static G_LED_GREEN_1: Mutex<RefCell<Option<LedGreen1>>> = Mutex::new(RefCell::new(None));
static G_LED_GREEN_2: Mutex<RefCell<Option<LedGreen2>>> = Mutex::new(RefCell::new(None));

static G_LED_RED_1: Mutex<RefCell<Option<LedRed1>>> = Mutex::new(RefCell::new(None));
static G_LED_RED_2: Mutex<RefCell<Option<LedRed2>>> = Mutex::new(RefCell::new(None));

static G_LED_ORANGE_1: Mutex<RefCell<Option<LedOrange1>>> = Mutex::new(RefCell::new(None));
static G_LED_ORANGE_2: Mutex<RefCell<Option<LedOrange2>>> = Mutex::new(RefCell::new(None));

static G_LED_BLUE_1: Mutex<RefCell<Option<LedBlue1>>> = Mutex::new(RefCell::new(None));
static G_LED_BLUE_2: Mutex<RefCell<Option<LedBlue2>>> = Mutex::new(RefCell::new(None));

static G_LED_WHITE_1: Mutex<RefCell<Option<LedWhite1>>> = Mutex::new(RefCell::new(None));
static G_LED_WHITE_2: Mutex<RefCell<Option<LedWhite2>>> = Mutex::new(RefCell::new(None));
static G_LED_WHITE_3: Mutex<RefCell<Option<LedWhite3>>> = Mutex::new(RefCell::new(None));
//
//
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .require_pll48clk()
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    let gpiof = dp.GPIOF.split();

    // ENCODERs

    /*
    let rotary_encoder_1_pins = (gpioa.pa15, gpiob.pb3);
    let rotary_encoder_1_timer = dp.TIM2;
    let rotary_encoder_1 = Qei::new(rotary_encoder_1_timer, rotary_encoder_1_pins);
    */

    let rotary_encoder_2_pins = (gpioa.pa6, gpioc.pc7);
    let rotary_encoder_2_timer = dp.TIM3;
    let rotary_encoder_2 = Qei::new(rotary_encoder_2_timer, rotary_encoder_2_pins);

    let rotary_encoder_3_pins = (gpiod.pd12, gpiod.pd13);
    let rotary_encoder_3_timer = dp.TIM4;
    let rotary_encoder_3 = Qei::new(rotary_encoder_3_timer, rotary_encoder_3_pins);

    let rotary_encoder_4_pins = (gpioa.pa0, gpioa.pa1);
    let rotary_encoder_4_timer = dp.TIM5;
    let rotary_encoder_4 = Qei::new(rotary_encoder_4_timer, rotary_encoder_4_pins);

    // LEDs
    let led_user_1 = gpiob.pb0.into_push_pull_output();
    let led_user_2 = gpiob.pb7.into_push_pull_output();
    let led_user_3 = gpiob.pb14.into_push_pull_output();

    let led_green_1 = gpiof.pf6.into_push_pull_output();
    let led_green_2 = gpiof.pf0.into_push_pull_output();

    let led_red_1 = gpiof.pf7.into_push_pull_output();
    let led_red_2 = gpiof.pf1.into_push_pull_output();

    let led_orange_1 = gpiof.pf8.into_push_pull_output();
    let led_orange_2 = gpiof.pf2.into_push_pull_output();

    let led_blue_1 = gpiof.pf9.into_push_pull_output();
    let led_blue_2 = gpiof.pf3.into_push_pull_output();

    let led_white_1 = gpiof.pf10.into_push_pull_output();
    let led_white_2 = gpiof.pf4.into_push_pull_output();
    let led_white_3 = gpiof.pf5.into_push_pull_output();

    cortex_m::interrupt::free(|cs| {
        G_LED_USER_1.borrow(cs).replace(Some(led_user_1));
        G_LED_USER_2.borrow(cs).replace(Some(led_user_2));
        G_LED_USER_3.borrow(cs).replace(Some(led_user_3));

        G_LED_GREEN_1.borrow(cs).replace(Some(led_green_1));
        G_LED_GREEN_2.borrow(cs).replace(Some(led_green_2));

        G_LED_RED_1.borrow(cs).replace(Some(led_red_1));
        G_LED_RED_2.borrow(cs).replace(Some(led_red_2));

        G_LED_ORANGE_1.borrow(cs).replace(Some(led_orange_1));
        G_LED_ORANGE_2.borrow(cs).replace(Some(led_orange_2));

        G_LED_BLUE_1.borrow(cs).replace(Some(led_blue_1));
        G_LED_BLUE_2.borrow(cs).replace(Some(led_blue_2));

        G_LED_WHITE_1.borrow(cs).replace(Some(led_white_1));
        G_LED_WHITE_2.borrow(cs).replace(Some(led_white_2));
        G_LED_WHITE_3.borrow(cs).replace(Some(led_white_3));
    });
    //

    let mut delay = dp.TIM1.delay_ms(&clocks);

    loop {
        set_led_state(LedsEnum::LedUser1, PinState::Low);
        set_led_state(LedsEnum::LedUser2, PinState::High);
        set_led_state(LedsEnum::LedUser3, PinState::Low);

        set_led_state(LedsEnum::LedGreen1, PinState::High);
        set_led_state(LedsEnum::LedGreen2, PinState::Low);

        set_led_state(LedsEnum::LedRed1, PinState::High);
        set_led_state(LedsEnum::LedRed2, PinState::Low);

        set_led_state(LedsEnum::LedOrange1, PinState::High);
        set_led_state(LedsEnum::LedOrange2, PinState::Low);

        set_led_state(LedsEnum::LedBlue1, PinState::High);
        set_led_state(LedsEnum::LedBlue2, PinState::Low);

        set_led_state(LedsEnum::LedWhite1, PinState::High);
        set_led_state(LedsEnum::LedWhite2, PinState::Low);
        set_led_state(LedsEnum::LedWhite3, PinState::High);

        delay.delay_ms(1000_u32);

        set_led_state(LedsEnum::LedUser1, PinState::High);
        set_led_state(LedsEnum::LedUser2, PinState::Low);
        set_led_state(LedsEnum::LedUser3, PinState::High);

        set_led_state(LedsEnum::LedGreen1, PinState::Low);
        set_led_state(LedsEnum::LedGreen2, PinState::High);

        set_led_state(LedsEnum::LedRed1, PinState::Low);
        set_led_state(LedsEnum::LedRed2, PinState::High);

        set_led_state(LedsEnum::LedOrange1, PinState::Low);
        set_led_state(LedsEnum::LedOrange2, PinState::High);

        set_led_state(LedsEnum::LedBlue1, PinState::Low);
        set_led_state(LedsEnum::LedBlue2, PinState::High);

        set_led_state(LedsEnum::LedWhite1, PinState::Low);
        set_led_state(LedsEnum::LedWhite2, PinState::High);
        set_led_state(LedsEnum::LedWhite3, PinState::Low);

        delay.delay_ms(1000_u32);
    }
}

fn set_led_state(led: LedsEnum, state: PinState) {
    cortex_m::interrupt::free(|cs| match led {
        LedsEnum::LedUser1 => G_LED_USER_1
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedUser2 => G_LED_USER_2
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedUser3 => G_LED_USER_3
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),

        LedsEnum::LedGreen1 => G_LED_GREEN_1
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedGreen2 => G_LED_GREEN_2
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),

        LedsEnum::LedRed1 => G_LED_RED_1
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedRed2 => G_LED_RED_2
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),

        LedsEnum::LedOrange1 => G_LED_ORANGE_1
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedOrange2 => G_LED_ORANGE_2
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),

        LedsEnum::LedBlue1 => G_LED_BLUE_1
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedBlue2 => G_LED_BLUE_2
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),

        LedsEnum::LedWhite1 => G_LED_WHITE_1
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedWhite2 => G_LED_WHITE_2
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
        LedsEnum::LedWhite3 => G_LED_WHITE_3
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_state(state),
    })
}
