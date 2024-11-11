use crate::Tm;

#[repr(C)]
pub(crate) struct TVariant {
    pub value: VariantValue,
    pub elements: u32, //Dimension for an one-dimensional array in pvarVal
    pub vt: VariantType,
}

#[repr(u16)]
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum VariantType {
    Empty = 0,
    Null,
    I2,        //int16_t
    I4,        //int32_t
    R4,        //float
    R8,        //double
    Date,      //DATE (double)
    TM,        //struct tm
    Pstr,      //struct str    string
    Interface, //struct iface
    Error,     //int32_t errCode
    Bool,      //bool
    Variant,   //struct _tVariant *
    I1,        //int8_t
    Ui1,       //uint8_t
    Ui2,       //uint16_t
    Ui4,       //uint32_t
    I8,        //int64_t
    Ui8,       //uint64_t
    Int,       //int   Depends on architecture
    Uint,      //unsigned int  Depends on architecture
    Hresult,   //long hRes
    Pwstr,     //struct wstr
    Blob,      //means in struct str binary data contain
    Clsid,     //UUID

    Undefined = 0xFFFF,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct DataStr {
    pub ptr: *mut u16,
    pub len: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct DataBlob {
    pub ptr: *mut u8,
    pub len: u32,
}

#[repr(C)]
pub(crate) union VariantValue {
    pub bool: bool,
    pub i32: i32,
    pub f64: f64,
    pub tm: Tm,
    pub data_str: DataStr,
    pub data_blob: DataBlob,
}
