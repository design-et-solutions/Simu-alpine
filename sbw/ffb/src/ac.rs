use super::model::SPageFilePhysics;
use crate::FFBData;
use std::{ffi::CString, mem, ptr};
use windows::{
    Win32::{Foundation::CloseHandle, System::Memory::*},
    core::*,
};

pub fn read_ac_data() -> Option<FFBData> {
    unsafe {
        let name = match CString::new(r"Local\acpmf_physics") {
            Ok(n) => n,
            Err(e) => {
                eprintln!("[AC FFB] Failed to create CString: {e}");
                return None;
            }
        };
        let mapping =
            match OpenFileMappingA(FILE_MAP_READ.0, false, PCSTR(name.as_ptr() as *const u8)) {
                Ok(handle) => handle,
                Err(e) => {
                    println!("[AC FFB] Could not open shared memory mapping: {e}");
                    return None;
                }
            };
        if mapping.is_invalid() {
            println!("[AC FFB] Could not open shared memory mapping (AC not running?)");
            return None;
        }
        let view = MapViewOfFile(
            mapping,
            FILE_MAP_READ,
            0,
            0,
            mem::size_of::<SPageFilePhysics>(),
        );
        if view.Value.is_null() {
            eprintln!("[AC FFB] Failed to map view of file");
            if let Err(e) = CloseHandle(mapping) {
                println!("[AC FFB] CloseHandle failed: {e}");
                return None;
            }
            return None;
        }

        let physics_ptr = view.Value as *const SPageFilePhysics;
        let physics = ptr::read_unaligned(physics_ptr);
        if let Err(e) = UnmapViewOfFile(view as _) {
            println!("[AC FFB] UnmapViewOfFile failed: {e}");
            return None;
        }
        if let Err(e) = CloseHandle(mapping) {
            println!("[AC FFB] CloseHandle failed: {e}");
            return None;
        }

        Some(FFBData {
            final_ff: physics.finalFF,
            steer_angle: physics.steerAngle,
        })
    }
}
