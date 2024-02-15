#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog-to-Digital Converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "sample time register 1"]
    #[inline(always)]
    pub const fn smpr1(self) -> crate::common::Reg<regs::Smpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "sample time register 2"]
    #[inline(always)]
    pub const fn smpr2(self) -> crate::common::Reg<regs::Smpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "injected channel data offset register 1"]
    #[inline(always)]
    pub const fn jofr1(self) -> crate::common::Reg<regs::Jofr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "injected channel data offset register 2"]
    #[inline(always)]
    pub const fn jofr2(self) -> crate::common::Reg<regs::Jofr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "injected channel data offset register 3"]
    #[inline(always)]
    pub const fn jofr3(self) -> crate::common::Reg<regs::Jofr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "injected channel data offset register 4"]
    #[inline(always)]
    pub const fn jofr4(self) -> crate::common::Reg<regs::Jofr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "watchdog higher threshold register"]
    #[inline(always)]
    pub const fn htr(self) -> crate::common::Reg<regs::Htr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "watchdog lower threshold register"]
    #[inline(always)]
    pub const fn ltr(self) -> crate::common::Reg<regs::Ltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "regular sequence register 1"]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "regular sequence register 2"]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "regular sequence register 3"]
    #[inline(always)]
    pub const fn sqr3(self) -> crate::common::Reg<regs::Sqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "injected sequence register"]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "injected data register 1"]
    #[inline(always)]
    pub const fn jdr1(self) -> crate::common::Reg<regs::Jdr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "injected data register 2"]
    #[inline(always)]
    pub const fn jdr2(self) -> crate::common::Reg<regs::Jdr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "injected data register 3"]
    #[inline(always)]
    pub const fn jdr3(self) -> crate::common::Reg<regs::Jdr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "injected data register 4"]
    #[inline(always)]
    pub const fn jdr4(self) -> crate::common::Reg<regs::Jdr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "regular data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "analog watchdog channel select bits"]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "analog watchdog channel select bits"]
        #[inline(always)]
        pub fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "interrupt enable for EOC"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for EOC"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "analog watchdog interrupt enable"]
        #[inline(always)]
        pub const fn awdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog interrupt enable"]
        #[inline(always)]
        pub fn set_awdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "interrupt enable for injected channels"]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for injected channels"]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "scan mode"]
        #[inline(always)]
        pub const fn scan(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "scan mode"]
        #[inline(always)]
        pub fn set_scan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "enable the watchdog on a single channel in scan mode"]
        #[inline(always)]
        pub const fn awdsgl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "enable the watchdog on a single channel in scan mode"]
        #[inline(always)]
        pub fn set_awdsgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "automatic injected group conversion"]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "automatic injected group conversion"]
        #[inline(always)]
        pub fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "discontinuous mode on regular channels"]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "discontinuous mode on regular channels"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "discontinuous mode on injected channels"]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "discontinuous mode on injected channels"]
        #[inline(always)]
        pub fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "discontinuous mode channel count"]
        #[inline(always)]
        pub const fn discnum(&self) -> super::vals::Discnum {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Discnum::from_bits(val as u8)
        }
        #[doc = "discontinuous mode channel count"]
        #[inline(always)]
        pub fn set_discnum(&mut self, val: super::vals::Discnum) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "analog watchdog enable on injected channels"]
        #[inline(always)]
        pub const fn jawden(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog enable on injected channels"]
        #[inline(always)]
        pub fn set_jawden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "analog watchdog enable on regular channels"]
        #[inline(always)]
        pub const fn awden(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog enable on regular channels"]
        #[inline(always)]
        pub fn set_awden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "A/D converter ON / OFF"]
        #[inline(always)]
        pub const fn adon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "A/D converter ON / OFF"]
        #[inline(always)]
        pub fn set_adon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "continuous conversion"]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "continuous conversion"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "A/D calibration"]
        #[inline(always)]
        pub const fn cal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "A/D calibration"]
        #[inline(always)]
        pub fn set_cal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "reset calibration"]
        #[inline(always)]
        pub const fn rstcal(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "reset calibration"]
        #[inline(always)]
        pub fn set_rstcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA disable selection (for single ADC mode)"]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DMA disable selection (for single ADC mode)"]
        #[inline(always)]
        pub fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "data alignment"]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "data alignment"]
        #[inline(always)]
        pub fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "external event select for injected group"]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "external event select for injected group"]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "external trigger conversion mode for injected channels"]
        #[inline(always)]
        pub const fn jexttrig(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "external trigger conversion mode for injected channels"]
        #[inline(always)]
        pub fn set_jexttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "external event select for regular group"]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "external event select for regular group"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "external trigger conversion mode for regular channels"]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "external trigger conversion mode for regular channels"]
        #[inline(always)]
        pub fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "start conversion of injected channels"]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "start conversion of injected channels"]
        #[inline(always)]
        pub fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "start conversion of regular channels"]
        #[inline(always)]
        pub const fn swstart(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "start conversion of regular channels"]
        #[inline(always)]
        pub fn set_swstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "temperature sensor and VREFINT enable"]
        #[inline(always)]
        pub const fn tsvrefe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sensor and VREFINT enable"]
        #[inline(always)]
        pub fn set_tsvrefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    #[doc = "regular data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Regular data"]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data"]
        #[inline(always)]
        pub fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    #[doc = "watchdog higher threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr(pub u32);
    impl Htr {
        #[doc = "Analog watchdog high threshold"]
        #[inline(always)]
        pub const fn ht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog high threshold"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Htr {
        #[inline(always)]
        fn default() -> Htr {
            Htr(0)
        }
    }
    #[doc = "injected data register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr1(pub u32);
    impl Jdr1 {
        #[doc = "Injected data"]
        #[inline(always)]
        pub const fn jdata1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data"]
        #[inline(always)]
        pub fn set_jdata1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Jdr1 {
        #[inline(always)]
        fn default() -> Jdr1 {
            Jdr1(0)
        }
    }
    #[doc = "injected data register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr2(pub u32);
    impl Jdr2 {
        #[doc = "Injected data"]
        #[inline(always)]
        pub const fn jdata2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data"]
        #[inline(always)]
        pub fn set_jdata2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Jdr2 {
        #[inline(always)]
        fn default() -> Jdr2 {
            Jdr2(0)
        }
    }
    #[doc = "injected data register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr3(pub u32);
    impl Jdr3 {
        #[doc = "Injected data"]
        #[inline(always)]
        pub const fn jdata3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data"]
        #[inline(always)]
        pub fn set_jdata3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Jdr3 {
        #[inline(always)]
        fn default() -> Jdr3 {
            Jdr3(0)
        }
    }
    #[doc = "injected data register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr4(pub u32);
    impl Jdr4 {
        #[doc = "Injected data"]
        #[inline(always)]
        pub const fn jdata4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data"]
        #[inline(always)]
        pub fn set_jdata4(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Jdr4 {
        #[inline(always)]
        fn default() -> Jdr4 {
            Jdr4(0)
        }
    }
    #[doc = "injected channel data offset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr1(pub u32);
    impl Jofr1 {
        #[doc = "data offset for injected channel 1"]
        #[inline(always)]
        pub const fn joffset1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "data offset for injected channel 1"]
        #[inline(always)]
        pub fn set_joffset1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr1 {
        #[inline(always)]
        fn default() -> Jofr1 {
            Jofr1(0)
        }
    }
    #[doc = "injected channel data offset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr2(pub u32);
    impl Jofr2 {
        #[doc = "data offset for injected channel 2"]
        #[inline(always)]
        pub const fn joffset2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "data offset for injected channel 2"]
        #[inline(always)]
        pub fn set_joffset2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr2 {
        #[inline(always)]
        fn default() -> Jofr2 {
            Jofr2(0)
        }
    }
    #[doc = "injected channel data offset register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr3(pub u32);
    impl Jofr3 {
        #[doc = "data offset for injected channel 3"]
        #[inline(always)]
        pub const fn joffset3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "data offset for injected channel 3"]
        #[inline(always)]
        pub fn set_joffset3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr3 {
        #[inline(always)]
        fn default() -> Jofr3 {
            Jofr3(0)
        }
    }
    #[doc = "injected channel data offset register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr4(pub u32);
    impl Jofr4 {
        #[doc = "data offset for injected channel 4"]
        #[inline(always)]
        pub const fn joffset4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "data offset for injected channel 4"]
        #[inline(always)]
        pub fn set_joffset4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr4 {
        #[inline(always)]
        fn default() -> Jofr4 {
            Jofr4(0)
        }
    }
    #[doc = "injected sequence register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "1st conversion in injected sequence"]
        #[inline(always)]
        pub const fn jsq1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in injected sequence"]
        #[inline(always)]
        pub fn set_jsq1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "2nd conversion in injected sequence"]
        #[inline(always)]
        pub const fn jsq2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "2nd conversion in injected sequence"]
        #[inline(always)]
        pub fn set_jsq2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "3rd conversion in injected sequence"]
        #[inline(always)]
        pub const fn jsq3(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "3rd conversion in injected sequence"]
        #[inline(always)]
        pub fn set_jsq3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "4th conversion in injected sequence"]
        #[inline(always)]
        pub const fn jsq4(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "4th conversion in injected sequence"]
        #[inline(always)]
        pub fn set_jsq4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "injected sequence length"]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "injected sequence length"]
        #[inline(always)]
        pub fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Jsqr {
        #[inline(always)]
        fn default() -> Jsqr {
            Jsqr(0)
        }
    }
    #[doc = "watchdog lower threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr(pub u32);
    impl Ltr {
        #[doc = "Analog watchdog low threshold"]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog low threshold"]
        #[inline(always)]
        pub fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Ltr {
        #[inline(always)]
        fn default() -> Ltr {
            Ltr(0)
        }
    }
    #[doc = "sample time register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr1(pub u32);
    impl Smpr1 {
        #[doc = "channel 10 sampling time selection"]
        #[inline(always)]
        pub const fn smp10(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 10 sampling time selection"]
        #[inline(always)]
        pub fn set_smp10(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "channel 11 sampling time selection"]
        #[inline(always)]
        pub const fn smp11(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 11 sampling time selection"]
        #[inline(always)]
        pub fn set_smp11(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "channel 12 sampling time selection"]
        #[inline(always)]
        pub const fn smp12(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 12 sampling time selection"]
        #[inline(always)]
        pub fn set_smp12(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "channel 13 sampling time selection"]
        #[inline(always)]
        pub const fn smp13(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 13 sampling time selection"]
        #[inline(always)]
        pub fn set_smp13(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "channel 14 sampling time selection"]
        #[inline(always)]
        pub const fn smp14(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 14 sampling time selection"]
        #[inline(always)]
        pub fn set_smp14(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "channel 15 sampling time selection"]
        #[inline(always)]
        pub const fn smp15(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 15usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 15 sampling time selection"]
        #[inline(always)]
        pub fn set_smp15(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
        }
        #[doc = "channel 16 sampling time selection"]
        #[inline(always)]
        pub const fn smp16(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 18usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 16 sampling time selection"]
        #[inline(always)]
        pub fn set_smp16(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
        }
        #[doc = "channel 17 sampling time selection"]
        #[inline(always)]
        pub const fn smp17(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 17 sampling time selection"]
        #[inline(always)]
        pub fn set_smp17(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "channel 18 sampling time selection"]
        #[inline(always)]
        pub const fn smp18(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 18 sampling time selection"]
        #[inline(always)]
        pub fn set_smp18(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Smpr1 {
        #[inline(always)]
        fn default() -> Smpr1 {
            Smpr1(0)
        }
    }
    #[doc = "sample time register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr2(pub u32);
    impl Smpr2 {
        #[doc = "channel 0 sampling time selection"]
        #[inline(always)]
        pub const fn smp0(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 0 sampling time selection"]
        #[inline(always)]
        pub fn set_smp0(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "channel 1 sampling time selection"]
        #[inline(always)]
        pub const fn smp1(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 1 sampling time selection"]
        #[inline(always)]
        pub fn set_smp1(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "channel 2 sampling time selection"]
        #[inline(always)]
        pub const fn smp2(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 2 sampling time selection"]
        #[inline(always)]
        pub fn set_smp2(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "channel 3 sampling time selection"]
        #[inline(always)]
        pub const fn smp3(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 3 sampling time selection"]
        #[inline(always)]
        pub fn set_smp3(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "channel 4 sampling time selection"]
        #[inline(always)]
        pub const fn smp4(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 4 sampling time selection"]
        #[inline(always)]
        pub fn set_smp4(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "channel 5 sampling time selection"]
        #[inline(always)]
        pub const fn smp5(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 15usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 5 sampling time selection"]
        #[inline(always)]
        pub fn set_smp5(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
        }
        #[doc = "channel 6 sampling time selection"]
        #[inline(always)]
        pub const fn smp6(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 18usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 6 sampling time selection"]
        #[inline(always)]
        pub fn set_smp6(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
        }
        #[doc = "channel 7 sampling time selection"]
        #[inline(always)]
        pub const fn smp7(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 7 sampling time selection"]
        #[inline(always)]
        pub fn set_smp7(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "channel 8 sampling time selection"]
        #[inline(always)]
        pub const fn smp8(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 8 sampling time selection"]
        #[inline(always)]
        pub fn set_smp8(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "channel 9 sampling time selection"]
        #[inline(always)]
        pub const fn smp9(&self) -> super::vals::SampleTime {
            let val = (self.0 >> 27usize) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "channel 9 sampling time selection"]
        #[inline(always)]
        pub fn set_smp9(&mut self, val: super::vals::SampleTime) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
        }
    }
    impl Default for Smpr2 {
        #[inline(always)]
        fn default() -> Smpr2 {
            Smpr2(0)
        }
    }
    #[doc = "regular sequence register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "13th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq13(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "13th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "14th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq14(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "14th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "15th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq15(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "15th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "16th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq16(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "16th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "regular channel sequence length"]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "regular channel sequence length"]
        #[inline(always)]
        pub fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Sqr1 {
        #[inline(always)]
        fn default() -> Sqr1 {
            Sqr1(0)
        }
    }
    #[doc = "regular sequence register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "7th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "7th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "8th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq8(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "8th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "9th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq9(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "9th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "10th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq10(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "10th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "11th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq11(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "11th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "12th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq12(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "12th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for Sqr2 {
        #[inline(always)]
        fn default() -> Sqr2 {
            Sqr2(0)
        }
    }
    #[doc = "regular sequence register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "1st conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "2nd conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "2nd conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "3rd conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq3(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "3rd conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "4th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq4(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "4th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "5th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "5th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "6th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq6(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "6th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for Sqr3 {
        #[inline(always)]
        fn default() -> Sqr3 {
            Sqr3(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "analog watchdog flag"]
        #[inline(always)]
        pub const fn awd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog flag"]
        #[inline(always)]
        pub fn set_awd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "end of conversion"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "end of conversion"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "injected channel end of conversion"]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "injected channel end of conversion"]
        #[inline(always)]
        pub fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "injected channel start flag"]
        #[inline(always)]
        pub const fn jstrt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "injected channel start flag"]
        #[inline(always)]
        pub fn set_jstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "regular channel start flag"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "regular channel start flag"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "overrun"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "overrun"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Discnum {
        #[doc = "1 conversions are discontinued and the conversion is carried out on one channel"]
        DISCNUM_1 = 0,
        #[doc = "2 conversion is discontinued and the conversions are carried out on 2 channels"]
        DISCNUM_2 = 0x01,
        #[doc = "3 conversions are discontinued and the conversions are carried out on 3 channels"]
        DISCNUM_3 = 0x02,
        #[doc = "4 conversions are discontinued and the conversions are carried out on 4 channels"]
        DISCNUM_4 = 0x03,
        #[doc = "5 conversions are discontinued and the conversions are carried out on 5 channels"]
        DISCNUM_5 = 0x04,
        #[doc = "6 conversions are discontinued and the conversions are carried out on 6 channels"]
        DISCNUM_6 = 0x05,
        #[doc = "7 conversions are discontinued and the conversions are carried out on 7 channels"]
        DISCNUM_7 = 0x06,
        #[doc = "8 conversions are discontinued and the conversions are carried out on 8 channels"]
        DISCNUM_8 = 0x07,
    }
    impl Discnum {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Discnum {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Discnum {
        #[inline(always)]
        fn from(val: u8) -> Discnum {
            Discnum::from_bits(val)
        }
    }
    impl From<Discnum> for u8 {
        #[inline(always)]
        fn from(val: Discnum) -> u8 {
            Discnum::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SampleTime {
        #[doc = "1.5 ADC clock cycles"]
        CYCLES1_5 = 0,
        #[doc = "7.5 ADC clock cycles"]
        CYCLES7_5 = 0x01,
        #[doc = "13.5 ADC clock cycles"]
        CYCLES13_5 = 0x02,
        #[doc = "28.5 ADC clock cycles"]
        CYCLES28_5 = 0x03,
        #[doc = "41.5 ADC clock cycles"]
        CYCLES41_5 = 0x04,
        #[doc = "55.5 ADC clock cycles"]
        CYCLES55_5 = 0x05,
        #[doc = "71.5 ADC clock cycles"]
        CYCLES71_5 = 0x06,
        #[doc = "239.5 ADC clock cycles"]
        CYCLES239_5 = 0x07,
    }
    impl SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleTime {
        #[inline(always)]
        fn from(val: u8) -> SampleTime {
            SampleTime::from_bits(val)
        }
    }
    impl From<SampleTime> for u8 {
        #[inline(always)]
        fn from(val: SampleTime) -> u8 {
            SampleTime::to_bits(val)
        }
    }
}
