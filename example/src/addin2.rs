use std::{error::Error, thread, time::Duration};

use addin1c::{
    cstr1c, name, AddinResult, CStr1C, CString1C, Connection, MethodInfo, Methods, PropInfo,
    SimpleAddin, Variant,
};
use chrono::Utc;

pub struct Addin2 {
    prop1: i32,
    last_error: Option<Box<dyn Error>>,
    interface: Option<&'static Connection>,
}

impl Addin2 {
    pub fn new() -> Addin2 {
        Addin2 {
            prop1: 0,
            last_error: None,
            interface: None,
        }
    }

    fn last_error(&mut self, value: &mut Variant) -> AddinResult {
        match &self.last_error {
            Some(err) => value
                .set_str1c(err.to_string().as_str())
                .map_err(|e| e.into()),
            None => value.set_str1c("").map_err(|e| e.into()),
        }
    }

    fn method1(&mut self, param: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        let value = param.get_i32()?;
        self.prop1 = value;
        ret_value.set_i32(value * 2);
        Ok(())
    }

    fn method2(
        &mut self,
        param1: &mut Variant,
        param2: &mut Variant,
        ret_value: &mut Variant,
    ) -> AddinResult {
        let value1 = param1.get_i32()?;
        let value2 = param2.get_i32()?;
        self.prop1 = value1 + value2;
        ret_value.set_i32(self.prop1);
        Ok(())
    }

    fn set_prop1(&mut self, value: &Variant) -> AddinResult {
        let value = value.get_i32()?;
        self.prop1 = value;
        Ok(())
    }

    fn get_prop1(&mut self, value: &mut Variant) -> AddinResult {
        value.set_i32(self.prop1);
        Ok(())
    }

    fn panic1(&mut self, _ret_value: &mut Variant) -> AddinResult {
        panic!("Panic1")
    }

    fn panic2(&mut self, _ret_value: &mut Variant) -> AddinResult {
        panic!("Panic{}", self.prop1)
    }

    fn method_no_params(&mut self, _ret_value: &mut Variant) -> AddinResult {
        Ok(())
    }

    fn utc(&mut self, ret_value: &mut Variant) -> AddinResult {
        ret_value.set_date(Utc::now().into());
        Ok(())
    }

    fn call_external_event(&mut self, ret_value: &mut Variant) -> AddinResult {
        if let Some(interface) = self.interface {
            let buffer_depth1 = interface.get_event_buffer_depth();
            interface.set_event_buffer_depth(100);
            let buffer_depth2 = interface.get_event_buffer_depth();
            let result =
                interface.external_event(cstr1c!("Addin2"), cstr1c!("Message1"), cstr1c!("Test1"));
            ret_value.set_str1c(format!(
                "result: {result}, buffer_depth1: {buffer_depth1}, buffer_depth2: {buffer_depth2}"
            ))?;

            thread::spawn(move || {
                let interface = interface;
                for i in 2..6 {
                    thread::sleep(Duration::from_millis(500));
                    interface.external_event(
                        cstr1c!("Addin2"),
                        CString1C::from_str(format!("Message{i}").as_str()),
                        CString1C::from_str(format!("Test{i}").as_str()),
                    );
                }
                interface.external_event(
                    cstr1c!("Addin2"),
                    cstr1c!("Shutdown"),
                    cstr1c!("Shutdown"),
                );
            });
        } else {
            ret_value.set_str1c("нет интерфейса")?;
        }
        Ok(())
    }
}

impl SimpleAddin for Addin2 {
    fn name() -> &'static CStr1C {
        name!("Class2")
    }

    fn init(&mut self, interface: &'static Connection) -> bool {
        self.interface = Some(interface);
        true
    }

    fn save_error(&mut self, err: Option<Box<dyn Error>>) {
        self.last_error = err;
    }

    fn methods() -> &'static [MethodInfo<Self>] {
        &[
            MethodInfo {
                name: name!("Method1"),
                method: Methods::Method1(Self::method1),
            },
            MethodInfo {
                name: name!("Method2"),
                method: Methods::Method2(Self::method2),
            },
            MethodInfo {
                name: name!("Panic1"),
                method: Methods::Method0(Self::panic1),
            },
            MethodInfo {
                name: name!("Panic2"),
                method: Methods::Method0(Self::panic2),
            },
            MethodInfo {
                name: name!("MethodNoParams"),
                method: Methods::Method0(Self::method_no_params),
            },
            MethodInfo {
                name: name!("Utc"),
                method: Methods::Method0(Self::utc),
            },
            MethodInfo {
                name: name!("CallExternalEvent"),
                method: Methods::Method0(Self::call_external_event),
            },
        ]
    }

    fn properties() -> &'static [PropInfo<Self>] {
        &[
            PropInfo {
                name: name!("Prop1"),
                getter: Some(Self::get_prop1),
                setter: Some(Self::set_prop1),
            },
            PropInfo {
                name: name!("LastError"),
                getter: Some(Self::last_error),
                setter: None,
            },
        ]
    }
}
