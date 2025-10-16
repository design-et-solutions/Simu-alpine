// use anyhow::Result;
// use windows::{Win32::System::Com::*, Win32::UI::Input::DirectInput::*, core::*};

// fn main() -> Result<()> {
//     unsafe {
//         CoInitializeEx(None, COINIT_MULTITHREADED)?;

//         let mut di: Option<IDirectInput8W> = None;
//         DirectInput8Create(
//             GetModuleHandleW(None)?,
//             DIRECTINPUT_VERSION,
//             &IDirectInput8W::IID,
//             di.set_abi(),
//             None,
//         )?;

//         println!("DirectInput initialized!");

//         Ok(())
//     }
// }

use windows::{
    Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*, core::*,
};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event)?;
        WaitForSingleObject(event, 0);
        CloseHandle(event)?;

        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
    }

    Ok(())
}
