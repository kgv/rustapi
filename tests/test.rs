use anyhow::Result;
use rustapi::um::d3dcompiler::d3d_compile;

#[test]
fn test() -> Result<()> {
    let source_data = b"struct VS_OUT
        {
            float4 Position : SV_Position;
            float4 Color : COLOR0;
        };

        float4 main( VS_OUT input ) : SV_Target
        {
            return float4( 0.0f, 0.0f, 1.0f, 1.0f );
        }
        ";
    let _bytecode = d3d_compile()
        .source_data(source_data)
        .source_name("shader")
        .entrypoint("main")
        .target("ps_4_0")
        .build()()?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::LookupPrivilegeValue;
//     use anyhow::Result;
//     use winapi::um::winnt::SE_DEBUG_NAME;

//     #[test]
//     fn lookup_privilege_value() -> Result<()> {
//         let _privilege = LookupPrivilegeValue::builder().name(SE_DEBUG_NAME).build()()?;
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod read_process_memory {
//         use super::ReadProcessMemory;
//         use crate::um::processthreadsapi::GetCurrentProcess;
//         use anyhow::Result;

//         #[test]
//         fn with_process_and_base_address_and_buffer() -> Result<()> {
//             let process = GetCurrentProcess();
//             let address = 9;
//             let mut buffer = [9; 9];
//             let read_process_memory = ReadProcessMemory::builder()
//                 .process(&process)
//                 .base_address(&address)
//                 .buffer(&mut buffer)
//                 .build();
//             // assert_eq!(9, *read_process_memory.base_address);
//             // assert_eq!(&[9; 9], read_process_memory.buffer);
//             // assert_eq!(9, read_process_memory.buffer.len());
//             Ok(())
//         }
//     }

//     mod virtual_query {
//         use super::VirtualQuery;
//         use crate::um::processthreadsapi::GetCurrentProcess;
//         use anyhow::Result;

//         #[test]
//         fn virtual_query() -> Result<()> {
//             let information = VirtualQuery::builder().address(0).build()()?;
//             assert_eq!(information.base_address(), 0);
//             Ok(())
//         }

//         #[test]
//         fn extended_virtual_query() -> Result<()> {
//             let process = GetCurrentProcess();
//             let information = VirtualQuery::builder().process(&process).address(0).build()()?;
//             assert_eq!(information.base_address(), 0);
//             Ok(())
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anyhow::Result;
//     use winapi::um::winnt::TOKEN_ADJUST_PRIVILEGES;

//     #[test]
//     fn test() -> Result<()> {
//         let process = get_current_process();
//         let _token = open_process_token()
//             .process_handle(&process)
//             .desired_access(TOKEN_ADJUST_PRIVILEGES)
//             .build()()?;
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod create_file {
//         use super::CreateFile;
//         use anyhow::Result;
//         use std::{env::temp_dir, fs::remove_file};
//         use winapi::um::{fileapi::CREATE_NEW, winnt::FILE_ALL_ACCESS};

//         #[test]
//         fn create_file() -> Result<()> {
//             let path = &temp_dir().join("temp_file");
//             println!("path: {}", path.display());
//             if path.exists() {
//                 remove_file(path)?;
//             }
//             let _privilege = CreateFile::builder()
//                 .file_name(path)
//                 .access_mode(FILE_ALL_ACCESS)
//                 .creation_disposition(CREATE_NEW)
//                 .build()()?;
//             assert!(path.exists());
//             Ok(())
//         }
//     }

//     mod get_logical_drive_strings {
//         use super::GetLogicalDriveStrings;
//         use anyhow::Result;

//         #[test]
//         fn default() -> Result<()> {
//             let drives = GetLogicalDriveStrings::default()()?;
//             assert!(drives.contains(&format!("C:\\")));
//             println!("drives: {:?}", drives);
//             Ok(())
//         }

//         #[test]
//         fn with_buffer() -> Result<()> {
//             let drives = GetLogicalDriveStrings::builder()
//                 .buffer(Vec::with_capacity(260))
//                 .build()()?;
//             assert!(drives.contains(&format!("C:\\")));
//             println!("drives: {:?}", drives);
//             Ok(())
//         }
//     }

//     mod query_dos_device {
//         use super::QueryDosDevice;
//         use anyhow::Result;

//         #[test]
//         fn default() -> Result<()> {
//             let device = QueryDosDevice::default()()?;
//             println!("device: {:#?}", device);
//             Ok(())
//         }

//         #[test]
//         fn with_device_name() -> Result<()> {
//             let device = QueryDosDevice::builder().device_name("C:").build()()?;
//             println!("device: {:#?}", device);
//             Ok(())
//         }

//         #[test]
//         fn with_device_name_and_target_path() -> Result<()> {
//             let _device = QueryDosDevice::builder()
//                 .device_name("C:")
//                 .target_path(Vec::with_capacity(2))
//                 .build()()?;
//             Ok(())
//         }
//     }
// }
