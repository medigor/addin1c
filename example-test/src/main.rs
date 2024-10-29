use std::error::Error;

use addin1c_test::{name, str1c, TestAddinLib, Variant};

use colored::Colorize;

fn main() -> Result<(), Box<dyn Error>> {
    let addin_lib = TestAddinLib::new(path_addin())?;
    let object1 = addin_lib.create_addin("Class1")?;

    let value = Variant::create_i32(321);
    object1.set_property(name!("PropI32"), &value)?;

    let new_value = object1.get_property(name!("PropI32"))?;
    assert_eq!(new_value.get_i32(), Some(321));

    let str1c = "Привет из Rust!".encode_utf16().collect::<Vec<_>>();
    let value = Variant::create_str(&str1c);
    object1.set_property(name!("PropStr"), &value)?;

    let new_value = object1.get_property(name!("PropStr"))?;
    assert_eq!(Some(str1c.as_slice()), new_value.get_str());

    let mut params = [
        Variant::create_str(str1c!("11")),
        Variant::create_str(str1c!("22")),
        Variant::create_str(str1c!("33")),
    ];

    let result = object1.call_as_func(name!("Method1"), &mut params)?;
    assert_eq!(result.get_str(), Some(str1c!("112233").as_slice()));

    println!("{}", "Test result: Ok".green());
    Ok(())
}

fn path_addin() -> &'static str {
    match (cfg!(windows), cfg!(debug_assertions)) {
        (true, true) => "../addin1c/target/debug/addin.dll",
        (true, false) => "../addin1c/target/release/addin.dll",
        (false, true) => "../addin1c/target/debug/libaddin.so",
        (false, false) => "../addin1c/target/release/libaddin.so",
    }
}
