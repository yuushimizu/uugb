use super::{Mbc, MbcContext};
use crate::util::bits::Bits;
use chrono::prelude::*;
use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Mbc3 {
    rom_bank_number: u8,
    ram_and_rtc_enabled: bool,
    ram_bank_number_or_rtc_register_select: u8,
    latched_datetime: DateTime<Local>,
    ready_to_latch: bool,
    day_counter: u64,
    rtc_halt: bool,
    day_counter_carry: bool,
}

impl Default for Mbc3 {
    fn default() -> Self {
        Self {
            rom_bank_number: 1,
            ram_and_rtc_enabled: false,
            ram_bank_number_or_rtc_register_select: 0,
            latched_datetime: Local::now(),
            ready_to_latch: false,
            day_counter: 0,
            rtc_halt: false,
            day_counter_carry: false,
        }
    }
}

impl Mbc3 {
    fn set_datetime(&mut self, value: Option<DateTime<Local>>) {
        if let Some(value) = value {
            self.latched_datetime = value;
        }
    }
}

impl Mbc for Mbc3 {
    fn read_rom(&self, context: &dyn MbcContext, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => context.get_from_rom_bank(0, address),
            0x4000..=0x7FFF => context.get_from_rom_bank(self.rom_bank_number, address - 0x4000),
            _ => unreachable!(),
        }
    }

    fn write_rom(&mut self, _: &mut dyn MbcContext, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_and_rtc_enabled = value == 0xA;
            }
            0x2000..=0x3FFF => {
                self.rom_bank_number = max(1, value & 0b0111_1111);
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number_or_rtc_register_select = value;
            }
            0x6000..=0x7FFF => {
                if self.ready_to_latch && value == 0x01 {
                    let now = Local::now();
                    let days = self
                        .latched_datetime
                        .date()
                        .signed_duration_since(now.date())
                        .num_days();
                    if days > 0 {
                        self.day_counter = self.day_counter.wrapping_add(days as u64);
                        if self.day_counter >= 512 {
                            self.day_counter_carry = true;
                            self.day_counter %= 512;
                        }
                    }
                    self.latched_datetime = now;
                }
                self.ready_to_latch = value == 0x00;
            }
            _ => unreachable!(),
        }
    }

    fn read_ram(&self, context: &dyn MbcContext, address: u16) -> u8 {
        if !self.ram_and_rtc_enabled {
            return 0xFF;
        }
        match self.ram_bank_number_or_rtc_register_select {
            0x00..=0x03 => {
                context.get_from_ram_bank(self.ram_bank_number_or_rtc_register_select, address)
            }
            0x08 => self.latched_datetime.second() as u8,
            0x09 => self.latched_datetime.minute() as u8,
            0x0A => self.latched_datetime.hour() as u8,
            0x0B => (self.day_counter & 0xFF) as u8,
            0x0C => {
                ((self.day_counter >> 8) & 0b1) as u8
                    | (self.rtc_halt as u8) << 6
                    | (self.day_counter_carry as u8) << 7
            }
            _ => 0xFF,
        }
    }

    fn write_ram(&mut self, context: &mut dyn MbcContext, address: u16, value: u8) {
        if !self.ram_and_rtc_enabled {
            return;
        }
        match self.ram_bank_number_or_rtc_register_select {
            0x00..=0x03 => {
                context.set_to_ram_bank(self.ram_bank_number_or_rtc_register_select, address, value)
            }
            0x08 => {
                self.set_datetime(self.latched_datetime.with_second(value as u32));
            }
            0x09 => {
                self.set_datetime(self.latched_datetime.with_minute(value as u32));
            }
            0x0A => {
                self.set_datetime(self.latched_datetime.with_hour(value as u32));
            }
            0x0B => {
                self.day_counter = (self.day_counter & !0xFF) | value as u64;
            }
            0x0C => {
                self.day_counter = (value as u64 & 0b1) << 8 | self.day_counter & 0xFF;
                self.rtc_halt = value.bit(6);
                self.day_counter_carry = value.bit(7);
            }
            _ => {}
        }
    }
}
