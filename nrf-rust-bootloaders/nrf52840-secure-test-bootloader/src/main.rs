#![no_std]
#![no_main]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate nrf52840_hal;
extern crate panic_halt;

use core;
use cortex_m_rt::{entry, exception};
// #[allow(unused_imports)]
// use cortex_m_semihosting::hprintln;

#[allow(dead_code, warnings)]
mod bl_cc310;
mod cc310_ecdsa_verify;

pub fn cc310_enable(peripherals: &mut nrf52840_hal::target::Peripherals) {
    peripherals
        .CRYPTOCELL
        .enable
        .write(|w| w.enable().enabled());
}

pub fn cc310_disable(peripherals: &mut nrf52840_hal::target::Peripherals) {
    peripherals
        .CRYPTOCELL
        .enable
        .write(|w| w.enable().disabled());
}

pub fn ecdsa_verify_app_signature() -> bl_cc310::CRYSError_t {
    let pub_key = cc310_ecdsa_verify::get_pubkey();
    let signature = cc310_ecdsa_verify::get_signature(0x82d0);
    let hash = match cc310_ecdsa_verify::calc_sha256_hash(0x8000) {
        Ok(v)   => v as *const u8,
        Err(e)  => panic!("{}", e),
    };
    // let hash = cc310_ecdsa_verify::get_test_hash();
    // let signature  = cc310_ecdsa_verify::get_test_signature();
    let verified =
        match cc310_ecdsa_verify::cc310_ecdsa_verify(&pub_key, &signature, hash, 0x20){
            Ok(v)  => v,
            Err(e) => panic!("{}", e),
        };
    return verified
}

pub fn boot_from(scb: &mut cortex_m::peripheral::SCB, address: u32) {
    // hprintln!("Trying to boot from 0x{:x}", address).unwrap();
    unsafe {
        let stack_pointer = *(address as *const u32);
        let reset_vector = *((address + 4) as *const u32);
        let jump_vector: extern "C" fn() = core::mem::transmute(reset_vector);

        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        scb.vtor.write(address);
        cortex_m::register::msp::write(stack_pointer);
        jump_vector();
    }
}

#[entry]
fn main() -> ! {
    // hprintln!("Starting the bootloader").unwrap();
    let mut core_peripherals = nrf52840_hal::target::CorePeripherals::take().unwrap();
    let mut peripherals = nrf52840_hal::target::Peripherals::take().unwrap();
    cc310_enable(&mut peripherals);
    let cc310_initlization_status;
    unsafe {
        cc310_initlization_status = bl_cc310::nrf_cc310_bl_init();
    }
    if cc310_initlization_status == bl_cc310::CRYS_OK {
        let verified = ecdsa_verify_app_signature();
        cc310_disable(&mut peripherals);
        if verified == bl_cc310::CRYS_OK {
            boot_from(&mut core_peripherals.SCB, 0x8000);
            loop {}
        } else {
            panic!("Error in verifying App Firmware {:?}", verified);
        }
    } else {
        panic!("Error initializing cc310 peripheral");
    }
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
