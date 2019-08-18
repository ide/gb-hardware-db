use regex::{Captures, Regex, RegexBuilder};
use std::str::FromStr;

pub use self::accelerometer::{parse_accelerometer, Accelerometer};
pub use self::cgb_cpu::{parse_cgb_cpu, CgbCpu};
pub use self::cgb_reg::{parse_cgb_reg, CgbReg};
pub use self::cgb_stamp::{parse_cgb_stamp, CgbStamp};
pub use self::cic::{parse_cic, Cic};
pub use self::coil::{parse_coil, Coil};
pub use self::crystal::{parse_crystal, Crystal};
pub use self::dmg_amp::{parse_dmg_amp, DmgAmp};
pub use self::dmg_cpu::{parse_dmg_cpu, DmgCpu, DmgCpuKind};
pub use self::dmg_reg::{parse_dmg_reg, DmgReg};
pub use self::dmg_stamp::{parse_dmg_stamp, DmgStamp};
pub use self::eeprom::{parse_eeprom, Eeprom};
pub use self::flash::{parse_flash, Flash};
pub use self::icd2::{parse_icd2, Icd2};
pub use self::lcd_chip::{parse_lcd_chip, LcdChip};
pub use self::lcd_screen::{parse_lcd_screen, LcdScreen};
pub use self::line_decoder::{parse_line_decoder, LineDecoder};
pub use self::mapper::{
    parse_mapper, Huc1Version, Mapper, MapperType, Mbc1Version, Mbc2Version, Mbc3Version,
};
pub use self::mask_rom::{parse_mask_rom, MaskRom};
pub use self::mgb_amp::{parse_mgb_amp, MgbAmp};
pub use self::mgb_cpu::{parse_mgb_cpu, MgbCpu};
pub use self::ram::{parse_ram, Ram};
pub use self::ram_backup::{parse_ram_backup, RamBackup};
pub use self::sgb2_cpu::{parse_sgb2_cpu, Sgb2Cpu};
pub use self::sgb_cpu::{parse_sgb_cpu, SgbCpu};
pub use self::sgb_rom::{parse_sgb_rom, SgbRom};
pub use self::tama::{parse_tama, TamaType};
pub use self::transformer::{parse_transformer, Transformer};

mod accelerometer;
mod cgb_cpu;
mod cgb_reg;
mod cgb_stamp;
mod cic;
mod coil;
mod crystal;
mod dmg_amp;
mod dmg_cpu;
mod dmg_reg;
mod dmg_stamp;
mod eeprom;
mod flash;
mod icd2;
mod lcd_chip;
mod lcd_screen;
mod line_decoder;
mod mapper;
mod mask_rom;
mod mgb_amp;
mod mgb_cpu;
mod ram;
mod ram_backup;
mod sgb2_cpu;
mod sgb_cpu;
mod sgb_rom;
mod tama;
mod transformer;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Manufacturer {
    Analog,
    AtT,
    Bsi,
    Crosslink,
    Fujitsu,
    Hudson,
    Hyundai,
    Kds,
    Lgs,
    LsiLogic,
    Kinseki,
    Macronix,
    Mitsubishi,
    Mitsumi,
    MoselVitelic,
    Motorola,
    Nec,
    Oki,
    Rohm,
    Samsung,
    Sanyo,
    Sharp,
    Smsc,
    Tdk,
    TexasInstruments,
    Toshiba,
    Victronix,
    Winbond,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Year {
    Full(u16),
    Partial(u8),
}

pub fn year1(text: &str) -> Result<Year, String> {
    match u8::from_str(text) {
        Ok(value) => Ok(Year::Partial(value)),
        _ => Err(format!("Invalid 1-digit year: {}", text)),
    }
}

pub fn year2(text: &str) -> Result<Year, String> {
    if text == "AL" {
        return Ok(Year::Full(2000));
    }
    match u16::from_str(text) {
        Ok(value @ 0..=87) => Ok(Year::Full(value + 2000)),
        Ok(value @ 88..=99) => Ok(Year::Full(value + 1900)),
        _ => Err(format!("Invalid 2-digit year: {}", text)),
    }
}

pub fn year2_u16(text: &str) -> Result<u16, String> {
    match u16::from_str(text) {
        Ok(value @ 0..=87) => Ok(value + 2000),
        Ok(value @ 88..=99) => Ok(value + 1900),
        _ => Err(format!("Invalid 2-digit year: {}", text)),
    }
}

pub fn week2(text: &str) -> Result<u8, String> {
    match u8::from_str(text) {
        Ok(value @ 1..=53) => Ok(value),
        _ => Err(format!("Invalid 2-digit week: {}", text)),
    }
}

pub fn month2(text: &str) -> Result<u8, String> {
    match u8::from_str(text) {
        Ok(value @ 1..=12) => Ok(value),
        _ => Err(format!("Invalid 2-digit month: {}", text)),
    }
}

pub struct Matcher<T> {
    regex: Regex,
    f: Box<dyn Fn(Captures) -> Result<T, String> + Sync>,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(Captures) -> Result<T, String> + Sync + 'static>(
        regex: &'static str,
        f: F,
    ) -> Matcher<T> {
        let regex = RegexBuilder::new(regex)
            .ignore_whitespace(true)
            .build()
            .unwrap();
        Matcher {
            regex,
            f: Box::new(f),
        }
    }
    pub fn apply(&self, text: &str) -> Option<T> {
        self.regex.captures(text).map(|c| (self.f)(c).unwrap())
    }
}
