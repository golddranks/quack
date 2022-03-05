use std::io::{Read, Seek};
use crate::elf::parse::ProgHead;

pub fn load(_ph: &[impl ProgHead], _reader: &mut (impl Read + Seek)) {
    
}