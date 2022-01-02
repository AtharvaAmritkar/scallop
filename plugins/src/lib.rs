use scallop::bash::builtins::Builtin;

#[export_name = "has_struct"]
pub static mut HAS_STRUCT: Option<Builtin> = None;
#[export_name = "hasv_struct"]
pub static mut HASV_STRUCT: Option<Builtin> = None;
#[export_name = "ver_rs_struct"]
pub static mut VER_RS_STRUCT: Option<Builtin> = None;

#[link_section = ".init_array"]
pub static INITIALIZE_BUILTINS: extern "C" fn() = initialize_builtins;

#[no_mangle]
pub extern "C" fn initialize_builtins() {
    unsafe {
        HAS_STRUCT = Some(Builtin::register("has"));
        HASV_STRUCT = Some(Builtin::register("hasv"));
        VER_RS_STRUCT = Some(Builtin::register("ver_rs"));
    }
}
