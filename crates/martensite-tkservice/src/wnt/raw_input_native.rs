//! Win32 owns the incoming handle; copy bytes before dispatching WM_INPUT cleanup.
#![allow(unsafe_code)]
use super::*;
use windows_sys::Win32::UI::Input::*;

fn failure(detail: &str) -> Failure {
    Failure::with_type(
        "Standard_DomainError",
        &format!("WNT raw input: {detail}; expected a complete bounded HID packet"),
        "",
    )
}

pub(in crate::wnt) fn register(window: usize) -> Result<(), Failure> {
    let device = RAWINPUTDEVICE {
        usUsagePage: 1,
        usUsage: 8,
        dwFlags: 0,
        hwndTarget: window as *mut _,
    };
    // SAFETY: this initialized single-device descriptor is live; flags request
    // foreground multi-axis input, without suppressing legacy mouse/keyboard input.
    if unsafe { RegisterRawInputDevices(&device, 1, size_of::<RAWINPUTDEVICE>() as u32) } == 0 {
        return Err(failure(&std::io::Error::last_os_error().to_string()));
    }
    Ok(())
}

fn packet_size(handle: usize) -> Result<u32, Failure> {
    let mut size = 0u32;
    let header_size = size_of::<RAWINPUTHEADER>() as u32;
    // SAFETY: a null data pointer and zero capacity query the required length.
    if unsafe {
        GetRawInputData(
            handle as *mut _,
            RID_INPUT,
            std::ptr::null_mut(),
            &mut size,
            header_size,
        )
    } == u32::MAX
    {
        return Err(failure(&std::io::Error::last_os_error().to_string()));
    }
    if !(header_size..=1024 * 1024).contains(&size) {
        return Err(failure(&format!("native byte count {size}")));
    }
    Ok(size)
}

fn packet_bytes(handle: usize) -> Result<(Vec<usize>, u32), Failure> {
    let mut size = packet_size(handle)?;
    let header_size = size_of::<RAWINPUTHEADER>() as u32;
    let mut words = vec![0usize; (size as usize).div_ceil(size_of::<usize>())];
    // SAFETY: word storage is aligned and has at least the advertised writable bytes.
    let copied = unsafe {
        GetRawInputData(
            handle as *mut _,
            RID_INPUT,
            words.as_mut_ptr().cast(),
            &mut size,
            header_size,
        )
    };
    if copied == u32::MAX
        || copied > size
        || copied < header_size
        || copied as usize > words.len() * size_of::<usize>()
    {
        return Err(failure("native copy failed or returned a truncated header"));
    }
    Ok((words, copied))
}

pub(in crate::wnt) fn read(handle: usize) -> Result<Vec<WntRawHidReport>, Failure> {
    let (words, copied) = packet_bytes(handle)?;
    // SAFETY: at least a complete header was copied into aligned initialized storage.
    let header = unsafe { std::ptr::read(words.as_ptr().cast::<RAWINPUTHEADER>()) };
    if header.dwType != RIM_TYPEHID {
        return Ok(Vec::new());
    }
    if header.dwSize != copied {
        return Err(failure("header byte count does not match copied bytes"));
    }
    let identity = device_identity(header.hDevice)?;
    // SAFETY: copied bytes lie within the initialized storage and are copied below.
    let bytes = unsafe { std::slice::from_raw_parts(words.as_ptr().cast::<u8>(), copied as usize) };
    reports(bytes, header.hDevice as usize, identity)
}

fn device_info(device: windows_sys::Win32::Foundation::HANDLE) -> Result<RID_DEVICE_INFO, Failure> {
    let mut info = RID_DEVICE_INFO {
        cbSize: size_of::<RID_DEVICE_INFO>() as u32,
        ..Default::default()
    };
    let mut size = info.cbSize;
    // SAFETY: info is initialized and writable with its stated size; no pointers are retained.
    if unsafe {
        GetRawInputDeviceInfoW(
            device,
            RIDI_DEVICEINFO,
            (&mut info as *mut RID_DEVICE_INFO).cast(),
            &mut size,
        )
    } == u32::MAX
        || info.dwType != RIM_TYPEHID
    {
        return Err(failure("native device is unavailable or is not HID"));
    }
    Ok(info)
}

fn device_identity(device: windows_sys::Win32::Foundation::HANDLE) -> Result<(u32, u32), Failure> {
    let info = device_info(device)?;
    // SAFETY: the successful query's discriminator selects the initialized HID member.
    let hid = unsafe { info.Anonymous.hid };
    Ok((hid.dwVendorId, hid.dwProductId))
}

fn reports(
    bytes: &[u8],
    device: usize,
    (vendor_id, product_id): (u32, u32),
) -> Result<Vec<WntRawHidReport>, Failure> {
    let (width, payload) = report_payload(bytes)?;
    Ok(payload
        .chunks_exact(width)
        .map(|bytes| WntRawHidReport {
            device,
            vendor_id,
            product_id,
            bytes: bytes.to_vec(),
        })
        .collect())
}

fn report_payload(bytes: &[u8]) -> Result<(usize, &[u8]), Failure> {
    let offset = std::mem::offset_of!(RAWINPUT, data);
    let sizes = bytes
        .get(offset..offset + 8)
        .ok_or_else(|| failure("missing HID dimensions"))?;
    let width = u32::from_le_bytes([sizes[0], sizes[1], sizes[2], sizes[3]]) as usize;
    let count = u32::from_le_bytes([sizes[4], sizes[5], sizes[6], sizes[7]]) as usize;
    let length = width
        .checked_mul(count)
        .ok_or_else(|| failure("HID dimensions overflow"))?;
    if width == 0 || count > 4096 {
        return Err(failure(&format!("HID report width {width}, count {count}")));
    }
    let payload = bytes
        .get(offset + 8..)
        .and_then(|payload| payload.get(..length))
        .ok_or_else(|| failure("HID report bytes are truncated"))?;
    Ok((width, payload))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_invalid_input_handle_is_rejected() {
        assert!(read(0).is_err());
        assert!(packet_size(0).is_err());
        assert!(packet_bytes(0).is_err());
        assert!(device_info(std::ptr::null_mut()).is_err());
        assert!(device_identity(std::ptr::null_mut()).is_err());
    }
    #[test]
    fn hid_dimensions_are_bounded_and_reports_keep_packet_order() {
        let offset = std::mem::offset_of!(RAWINPUT, data);
        let mut bytes = vec![0; offset];
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(&[1, 2, 3, 4]);
        let decoded = reports(&bytes, 7, (0x256f, 1)).unwrap();
        assert_eq!(decoded[0].bytes, [1, 2]);
        assert_eq!(decoded[1].bytes, [3, 4]);
        assert_eq!(decoded[0].vendor_id, 0x256f);
        bytes.pop();
        assert!(reports(&bytes, 7, (0, 0)).is_err());
        assert!(reports(&[], 0, (0, 0)).is_err());
    }
}
