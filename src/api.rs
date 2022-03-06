use arcropolis_api::arc_callback;
use arcropolis_api::hash40;
use arcropolis_api::load_original_file;
use crate::MEGAZARDX_SLOT;

#[arc_callback]
fn my_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    let costume_slot;
    if hash == hash40("fighter/plizardon/model/body/c00/model.nusktb").as_u64() {
        costume_slot = 0;
    }
    else if hash == hash40("fighter/plizardon/model/body/c01/model.nusktb").as_u64() {
        costume_slot = 1;
    }
    else if hash == hash40("fighter/plizardon/model/body/c02/model.nusktb").as_u64() {
        costume_slot = 2;
    }
    else if hash == hash40("fighter/plizardon/model/body/c03/model.nusktb").as_u64() {
        costume_slot = 3;
    }
    else if hash == hash40("fighter/plizardon/model/body/c04/model.nusktb").as_u64() {
        costume_slot = 4;
    }
    else if hash == hash40("fighter/plizardon/model/body/c05/model.nusktb").as_u64() {
        costume_slot = 5;
    }
    else if hash == hash40("fighter/plizardon/model/body/c06/model.nusktb").as_u64() {
        costume_slot = 6;
    }
    else {
        costume_slot = 7;
    }
    let res = load_original_file(hash, &mut data);
    let checksum = crc32fast::hash(&data[..0x5528]);
    unsafe {
        if checksum == 0x49ee4160 {
            MEGAZARDX_SLOT[costume_slot] = true;
        }
        else {
            MEGAZARDX_SLOT[costume_slot] = false;
        }
    }
    res
}

pub fn install() {
    my_callback::install("fighter/plizardon/model/body/c00/model.nusktb", 0x7C38);
    my_callback::install("fighter/plizardon/model/body/c01/model.nusktb", 0x7C38);
    my_callback::install("fighter/plizardon/model/body/c02/model.nusktb", 0x7C38);
    my_callback::install("fighter/plizardon/model/body/c03/model.nusktb", 0x7C38);
    my_callback::install("fighter/plizardon/model/body/c04/model.nusktb", 0x7C38);
    my_callback::install("fighter/plizardon/model/body/c05/model.nusktb", 0x7C38);
    my_callback::install("fighter/plizardon/model/body/c06/model.nusktb", 0x7C38);
    my_callback::install("fighter/plizardon/model/body/c07/model.nusktb", 0x7C38);
}