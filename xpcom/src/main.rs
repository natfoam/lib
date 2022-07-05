mod guid;
mod types;

use crate::types::{interface, method, param, ptr, struct_, Library, Type};

fn main() {
    let library = Library::from([
        struct_("S", &[param("begin", Type::U8)]),
        interface(
            "IMy",
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            &[method("GetValue", &[], ptr(Type::U32))],
        ),
    ]);

    // let x = serde_json::to_string(&library).unwrap();
    let x = serde_yaml::to_string(&library).unwrap();
    println!("{}", x);
}
