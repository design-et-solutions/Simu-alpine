use super::model::SPageFilePhysics;
use crate::FFBData;
use anyhow::{Result, anyhow};
use std::{ffi::CString, mem};
use windows::{Win32::System::Memory::*, core::*};

pub fn read_ac_data() -> Result<FFBData> {
    unsafe {
        let name = CString::new("Local\\acpmf_physics")?;
        let mapping = OpenFileMappingA(FILE_MAP_READ.0, false, PCSTR(name.as_ptr() as *const u8))?;

        let ptr = MapViewOfFile(
            mapping,
            FILE_MAP_READ,
            0,
            0,
            mem::size_of::<SPageFilePhysics>(),
        );
        if ptr.Value.is_null() {
            return Err(anyhow!("Failed to map view of file"));
        }
        let physics: &SPageFilePhysics = &*(ptr.Value as *const SPageFilePhysics);
        UnmapViewOfFile(ptr as _)?;
        Ok(FFBData {
            finalFF: physics.finalFF,
            // steerAngle: physics.steerAngle,
        })
    }
}
