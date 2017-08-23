//! Textual instruction formatting routines.

use gen::*;
use status::ZydisResult;
use std::mem;
use std::ffi::CString;
use std::os::raw::c_void; 


#[derive(Clone)]
pub enum Hook {
    FuncPre               (ZydisFormatterNotifyFunc       ),
    FuncPost              (ZydisFormatterNotifyFunc       ),
    FuncFormatInstruction (ZydisFormatterFormatFunc       ),
    FuncPrintPrefixes     (ZydisFormatterFormatFunc       ),
    FuncPrintMnemonic     (ZydisFormatterFormatFunc       ),
    FuncFormatOperandReg  (ZydisFormatterFormatOperandFunc),
    FuncFormatOperandMem  (ZydisFormatterFormatOperandFunc),
    FuncFormatOperandPtr  (ZydisFormatterFormatOperandFunc),
    FuncFormatOperandImm  (ZydisFormatterFormatOperandFunc),
    FuncPrintOperandsize  (ZydisFormatterFormatOperandFunc),
    FuncPrintSegment      (ZydisFormatterFormatOperandFunc),
    FuncPrintDecorator    (ZydisFormatterFormatOperandFunc),
    FuncPrintDisplacement (ZydisFormatterFormatOperandFunc),
    FuncPrintImmediate    (ZydisFormatterFormatOperandFunc),
    FuncPrintAddress      (ZydisFormatterFormatAddressFunc),
}

impl Hook {
    pub fn to_id(&self) -> ZydisFormatterHookTypes {
        use self::Hook::*;
        match *self {
            FuncPre               (_) => ZYDIS_FORMATTER_HOOK_PRE,
            FuncPost              (_) => ZYDIS_FORMATTER_HOOK_POST,
            FuncFormatInstruction (_) => ZYDIS_FORMATTER_HOOK_FORMAT_INSTRUCTION,
            FuncPrintPrefixes     (_) => ZYDIS_FORMATTER_HOOK_PRINT_PREFIXES,
            FuncPrintMnemonic     (_) => ZYDIS_FORMATTER_HOOK_PRINT_MNEMONIC,
            FuncFormatOperandReg  (_) => ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_REG,
            FuncFormatOperandMem  (_) => ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_MEM,
            FuncFormatOperandPtr  (_) => ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_PTR,
            FuncFormatOperandImm  (_) => ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_IMM,
            FuncPrintOperandsize  (_) => ZYDIS_FORMATTER_HOOK_PRINT_OPERANDSIZE,
            FuncPrintSegment      (_) => ZYDIS_FORMATTER_HOOK_PRINT_SEGMENT,
            FuncPrintDecorator    (_) => ZYDIS_FORMATTER_HOOK_PRINT_DECORATOR,
            FuncPrintDisplacement (_) => ZYDIS_FORMATTER_HOOK_PRINT_DISPLACEMENT,
            FuncPrintImmediate    (_) => ZYDIS_FORMATTER_HOOK_PRINT_IMMEDIATE,
            FuncPrintAddress      (_) => ZYDIS_FORMATTER_HOOK_PRINT_ADDRESS,
        }
    }

    pub unsafe fn to_raw(&self) -> *const c_void {
        use self::Hook::*;
        match *self {
            FuncPre               (ref x) => mem::transmute(x),
            FuncPost              (ref x) => mem::transmute(x),
            FuncFormatInstruction (ref x) => mem::transmute(x),
            FuncPrintPrefixes     (ref x) => mem::transmute(x),
            FuncPrintMnemonic     (ref x) => mem::transmute(x),
            FuncFormatOperandReg  (ref x) => mem::transmute(x),
            FuncFormatOperandMem  (ref x) => mem::transmute(x),
            FuncFormatOperandPtr  (ref x) => mem::transmute(x),
            FuncFormatOperandImm  (ref x) => mem::transmute(x),
            FuncPrintOperandsize  (ref x) => mem::transmute(x),
            FuncPrintSegment      (ref x) => mem::transmute(x),
            FuncPrintDecorator    (ref x) => mem::transmute(x),
            FuncPrintDisplacement (ref x) => mem::transmute(x),
            FuncPrintImmediate    (ref x) => mem::transmute(x),
            FuncPrintAddress      (ref x) => mem::transmute(x),
        }
    }

    pub unsafe fn from_raw(id: ZydisFormatterHookTypes, cb: *const c_void) -> Hook {
        use self::Hook::*;
        match id {
            ZYDIS_FORMATTER_HOOK_PRE                
                => FuncPre(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_POST               
                => FuncPost(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_FORMAT_INSTRUCTION 
                => FuncFormatInstruction(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_PREFIXES     
                => FuncPrintPrefixes(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_MNEMONIC     
                => FuncPrintMnemonic(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_REG 
                => FuncFormatOperandReg(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_MEM 
                => FuncFormatOperandMem(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_PTR 
                => FuncFormatOperandPtr(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_FORMAT_OPERAND_IMM 
                => FuncFormatOperandImm(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_OPERANDSIZE  
                => FuncPrintOperandsize(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_SEGMENT      
                => FuncPrintSegment(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_DECORATOR    
                => FuncPrintDecorator(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_DISPLACEMENT 
                => FuncPrintDisplacement(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_IMMEDIATE    
                => FuncPrintImmediate(mem::transmute(cb)),
            ZYDIS_FORMATTER_HOOK_PRINT_ADDRESS      
                => FuncPrintAddress(mem::transmute(cb)),
            _ 
                => unreachable!(),
        }
    }
}

pub struct Formatter {
    formatter: ZydisFormatter
}

impl Formatter {
    /// Creates a new formatter instance, accepting formatter flags.
    pub fn new_ex(
        style: ZydisFormatterStyles,
        flags: ZydisFormatterFlags,
        address_format: ZydisFormatterAddressFormats,
        displacement_format: ZydisFormatterDisplacementFormats,
        immmediate_format: ZydisFormatterImmediateFormats
    ) -> ZydisResult<Self> {
        unsafe {
            let mut formatter = Self { formatter: mem::uninitialized() };
            let status = ZydisFormatterInitEx(
                &mut formatter.formatter,
                style as ZydisFormatterStyle,
                flags,
                address_format as ZydisFormatterAddressFormat,
                displacement_format as ZydisFormatterDisplacementFormat,
                immmediate_format as ZydisFormatterImmediateFormat
            );
            match status {
                ZYDIS_STATUS_SUCCESS => Ok(formatter),
                _ => Err(status)
            }
        }
    }

    /// Creates a new formatter instance.
    pub fn new(style: ZydisFormatterStyles) -> ZydisResult<Self> {
        Self::new_ex(
            style, 
            0, 
            ZYDIS_FORMATTER_ADDR_DEFAULT, 
            ZYDIS_FORMATTER_DISP_DEFAULT, 
            ZYDIS_FORMATTER_IMM_DEFAULT,
        )
    }

    /// Formats the given instruction, returning a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut formatter = zydis::Formatter::new(
    ///     zydis::gen::ZYDIS_FORMATTER_STYLE_INTEL
    /// ).unwrap();
    /// let mut dec = zydis::Decoder::new(
    ///     zydis::gen::ZYDIS_MACHINE_MODE_LONG_64,
    ///     zydis::gen::ZYDIS_ADDRESS_WIDTH_64
    /// ).unwrap();
    ///
    /// static INT3: &'static [u8] = &[0xCCu8];
    /// let mut info = dec.decode(INT3, 0).unwrap();
    /// let fmt = formatter.format_instruction(&mut info).unwrap();
    /// assert_eq!(fmt, "int3");
    /// ```
    pub fn format_instruction(
        &mut self, instruction: &mut ZydisDecodedInstruction
    ) -> ZydisResult<String> {
        unsafe {
            let vec_buf = vec![0; 200];
            let vec_len = vec_buf.len();
            let str_buf = CString::from_vec_unchecked(vec_buf);
            let raw_buf = str_buf.into_raw();
            let status = ZydisFormatterFormatInstruction(
                &mut self.formatter, instruction, raw_buf, vec_len
            );
            match status {
                ZYDIS_STATUS_SUCCESS => Ok(
                    CString::from_raw(raw_buf)
                        .to_string_lossy()
                        .into_owned()
                ),
                _ => Err(status),
            }
        }
    }

    /// Sets a hook, allowing for customizations along the formatting process.
    pub fn set_hook(
        &mut self, 
        hook: Hook,
    ) -> ZydisResult<Hook> {
        unsafe {
            let mut cb = hook.to_raw();
            let hook_id = hook.to_id();
            let status = ZydisFormatterSetHook(
                &mut self.formatter, 
                hook_id as ZydisFormatterHookType, 
                &mut cb,
            );
            match status { 
                ZYDIS_STATUS_SUCCESS => Ok(Hook::from_raw(hook_id, cb)), 
                _ => Err(status) 
            }
        }
    }
}