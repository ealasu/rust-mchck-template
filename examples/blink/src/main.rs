#![feature(core_intrinsics,asm)]
#![no_main]
#![no_std]

extern crate board;

use core::intrinsics::volatile_store;

macro_rules! GPIO_CONFIG  {() => (0x40048038 as *mut u32);}
macro_rules! PORTB_PCR16  {() => (0x4004A040 as *mut u32);}
macro_rules! GPIOB_PDDR   {() => (0x400FF054 as *mut u32);}
macro_rules! GPIOB_PDOR   {() => (0x400FF040 as *mut u32);}
const LED_PIN: u32 = 1 << 16; // pin 16

#[no_mangle]
pub unsafe fn main() -> ! {
    // Enable system clock on all GPIO ports - page 254
    *GPIO_CONFIG!() = 0x00043F82; // 0b1000011111110000010
    // Configure the led pin
    *PORTB_PCR16!() = 0x00000143; // Enables GPIO | DSE | PULL_ENABLE | PULL_SELECT - page 227
    // Set the led pin to output
    *GPIOB_PDDR!() = LED_PIN;

    blink_led();
}

pub fn led_on() {
    unsafe {
        volatile_store(GPIOB_PDOR!(), LED_PIN);
    }
}

pub fn led_off() {
    unsafe {
        volatile_store(GPIOB_PDOR!(), 0x0);
    }
}

pub fn delay(ms: i32) {
    for _ in 0..ms * 1250 {
        unsafe {
            asm!("NOP");
        }
    }
}

pub fn blink_led() -> ! {
    loop {
        led_on();
        delay(1000);
        led_off();
        delay(1000);
    }
}
