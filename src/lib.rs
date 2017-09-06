#![feature(const_fn)]
#![feature(unique)]
#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate volatile;

mod vgabuffer;
mod vgawriter;
use core::ptr::Unique;
use core::ptr;


#[no_mangle]
pub extern fn rust_main() {

  print_something();
  //print_plain();
  
  // keep OS alive
  loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] extern fn panic_fmt() -> ! { loop{} }

// move this to vga writer
fn print_plain() {
 let buff: *mut u8 = (0xb8000 + 1988) as *mut _;
  unsafe {
      ptr::write(buff, (b'f'));
      ptr::write(buff.offset(1), 0x1f);
      ptr::write(buff.offset(2), (b'o'));
      ptr::write(buff.offset(3), 0x1f);
      ptr::write(buff.offset(4), (b'o'));
  }
}

fn print_something() {
  let mut writer = vgabuffer::Writer {
    column_pos : 0,
    color_code : vgabuffer::ColorCode::new(vgabuffer::Color::LightGreen, vgabuffer::Color::Black),
    buffer : unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
  };
  writer.clear_screen();
  writer.write_byte(b'A');

}