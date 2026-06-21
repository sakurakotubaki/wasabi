#![no_main]
#![no_std]

use core::panic::PanicInfo;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};
use uefi::table::boot::ScopedProtocol;

#[entry]
fn efi_main(_image: Handle, system_table: SystemTable<Boot>) -> Status {
    let boot_services = system_table.boot_services();

    let gop_handle = match boot_services.get_handle_for_protocol::<GraphicsOutput>() {
        Ok(handle) => handle,
        Err(error) => return error.status(),
    };

    let mut gop = match boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
        Ok(gop) => gop,
        Err(error) => return error.status(),
    };

    fill_screen_white(&mut gop);

    loop {}
}

fn fill_screen_white(gop: &mut ScopedProtocol<GraphicsOutput>) {
    let info = gop.current_mode_info();
    let (width, height) = info.resolution();
    let white = BltPixel::new(255, 255, 255);

    let _ = gop.blt(BltOp::VideoFill {
        color: white,
        dest: (0, 0),
        dims: (width, height),
    });
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
