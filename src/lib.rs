#![no_std]

pub mod constants {
    pub const USB_CLASS_NONE: u8 = 0x00;
    pub const USB_AUDIO_CLASS: u8 = 0x01;
    pub const USB_AUDIOCONTROL_SUBCLASS: u8 = 0x01;
    pub const USB_MIDISTREAMING_SUBCLASS: u8 = 0x03;

    pub const MIDI_PACKET_SIZE: usize = 4;
    pub const MAX_PACKET_SIZE: usize = 64;
}

mod midi_device;
