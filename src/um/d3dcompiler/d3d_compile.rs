use crate::{r#macro::FnOnce, um::d3d11::Blob};
use anyhow::{ensure, Result};
use format::lazy_format;
use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    ptr::{null, null_mut},
};
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3dcommon::{ID3DBlob, ID3DInclude, D3D_SHADER_MACRO},
};
use wio::com::ComPtr;

/// Direct 3D compile.
#[derive(FnOnce, TypedBuilder)]
pub struct D3DCompile<'a> {
    source_data: &'a [u8],
    source_name: &'a str,
    #[builder(default)]
    defines: Option<&'a D3D_SHADER_MACRO>,
    #[builder(default, setter(strip_option))]
    include: Option<ComPtr<ID3DInclude>>,
    #[builder(default, setter(strip_option))]
    entrypoint: Option<&'a str>,
    target: &'a str,
    #[builder(default)]
    flags1: u32,
    #[builder(default)]
    flags2: u32,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    code: MaybeUninit<ComPtr<ID3DBlob>>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    error_messages: MaybeUninit<ComPtr<ID3DBlob>>,
}

impl FnOnce<()> for D3DCompile<'_> {
    type Output = Result<ComPtr<ID3DBlob>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::d3dcompiler::D3DCompile;

        let source_name = CString::new(self.source_name)?;
        let entrypoint = self.entrypoint.map(CString::new).transpose()?;
        let target = CString::new(self.target)?;

        #[allow(non_snake_case)]
        unsafe {
            let pSrcData = self.source_data.as_ptr() as _;
            let SrcDataSize = self.source_data.len();
            let pSourceName = source_name.as_ptr();
            let pDefines = self.defines.as_ref().map_or(null(), |v| *v as _); // TODO
            let pInclude = self.include.as_ref().map_or(null_mut(), |v| v.as_raw());
            let pEntrypoint = entrypoint.as_ref().map_or(null(), |v| v.as_ptr());
            let pTarget = target.as_ptr() as _;
            let Flags1 = self.flags1;
            let Flags2 = self.flags2;
            let ppCode = self.code.as_mut_ptr() as _;
            let ppErrorMsgs = self.error_messages.as_mut_ptr() as _;
            let r#return = D3DCompile(
                pSrcData,
                SrcDataSize,
                pSourceName,
                pDefines,
                pInclude,
                pEntrypoint,
                pTarget,
                Flags1,
                Flags2,
                ppCode,
                ppErrorMsgs,
            );
            ensure!(
                SUCCEEDED(r#return),
                "The D3DCompile call FAILED ({:#x}).{}",
                r#return,
                lazy_format!(|f| {
                    if !(*ppErrorMsgs).is_null() {
                        write!(
                            f,
                            "\n\t{}",
                            CStr::from_ptr(
                                self.error_messages.assume_init_ref().get_buffer_pointer() as _,
                            )
                            .to_string_lossy()
                        )?;
                    }
                    Ok(())
                })
            );
            Ok(self.code.assume_init())
        }
    }
}
