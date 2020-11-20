use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
// use eetf::Term;
// use std::io::Cursor;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("pack", pack);
}

fn pack(_interface: &mut dyn Interface, _zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let result_box: Box<[u8]> = Box::new([1]);
    Op::Sync(result_box)
}
