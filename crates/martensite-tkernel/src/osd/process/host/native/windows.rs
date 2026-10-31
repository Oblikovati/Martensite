//! Safe Windows host queries (#334).
#![forbid(unsafe_code)]
use super::{HostSource, NativeHost, SystemNames};
use std::net::ToSocketAddrs;

impl HostSource for NativeHost {
    fn system_names(&self) -> SystemNames {
        self.system_names_checked().unwrap_or_default()
    }

    fn system_names_checked(&self) -> Result<SystemNames, i32> {
        super::windows_version::system_names()
    }

    fn host_name(&self) -> Result<Vec<u8>, i32> {
        winsafe::GetComputerName()
            .map(String::into_bytes)
            .map_err(|error| error.raw() as i32)
    }

    fn resolve_ipv4(&self, name: &[u8]) -> Option<[u8; 4]> {
        let name = std::str::from_utf8(name).ok()?;
        (name, 0)
            .to_socket_addrs()
            .ok()?
            .find_map(|address| match address.ip() {
                std::net::IpAddr::V4(address) => Some(address.octets()),
                std::net::IpAddr::V6(_) => None,
            })
    }

    fn available_memory_bytes(&self) -> Result<u64, i32> {
        winsafe::GlobalMemoryStatusEx()
            .map(|status| status.ullAvailPageFile)
            .map_err(|error| error.raw() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::osd::{Host, OsdErrorAccess};

    struct MemoryHost(Result<u64, i32>);

    impl HostSource for MemoryHost {
        fn system_names(&self) -> SystemNames {
            SystemNames::default()
        }
        fn host_name(&self) -> Result<Vec<u8>, i32> {
            Ok(b"test-host".to_vec())
        }
        fn resolve_ipv4(&self, name: &[u8]) -> Option<[u8; 4]> {
            (name == b"localhost").then_some([127, 0, 0, 1])
        }
        fn available_memory_bytes(&self) -> Result<u64, i32> {
            self.0
        }
    }

    struct FailedVersion;

    impl HostSource for FailedVersion {
        fn system_names(&self) -> SystemNames {
            SystemNames::default()
        }
        fn system_names_checked(&self) -> Result<SystemNames, i32> {
            Err(5)
        }
        fn host_name(&self) -> Result<Vec<u8>, i32> {
            Err(5)
        }
        fn resolve_ipv4(&self, _: &[u8]) -> Option<[u8; 4]> {
            None
        }
    }

    #[test]
    fn version_failure_records_the_operation() {
        let mut host = Host::with_source(FailedVersion);
        assert!(host.system_version().is_empty());
        assert!(host.failed());
        assert_eq!(host.osd_error().message().as_bytes(), b"SystemVersion");
    }

    #[test]
    fn native_host_version_is_available_through_the_public_api() {
        let mut host = Host::new();
        assert!(host.system_version().as_bytes().starts_with(b"Windows NT "));
        assert!(!host.failed());
    }

    #[test]
    fn memory_conversion_preserves_small_values_and_caps_large_values() {
        for bytes in [0, 42, i32::MAX as u64, u64::MAX] {
            let mut host = Host::with_source(MemoryHost(Ok(bytes)));
            assert_eq!(host.available_memory(), bytes.min(i32::MAX as u64) as i32);
            assert!(!host.failed());
        }
    }

    #[test]
    fn memory_failure_records_the_operation() {
        let mut host = Host::with_source(MemoryHost(Err(5)));
        assert_eq!(host.available_memory(), 0);
        assert!(host.failed());
        assert_eq!(host.osd_error().message().as_bytes(), b"AvailableMemory");
    }

    #[test]
    fn host_queries_use_the_source_and_resolve_localhost_as_fallback() {
        let mut host = Host::with_source(MemoryHost(Ok(0)));
        assert_eq!(host.host_name().as_bytes(), b"test-host");
        assert_eq!(host.internet_address().as_bytes(), b"127.0.0.1");
        assert!(!host.failed());
    }

    #[test]
    fn computer_name_is_nonempty_utf8() {
        let name = NativeHost.host_name().unwrap();
        assert!(!name.is_empty());
        assert!(std::str::from_utf8(&name).is_ok());
        assert!(!name.contains(&0));
    }

    #[test]
    fn ipv4_resolution_accepts_loopback_and_rejects_invalid_utf8() {
        assert_eq!(NativeHost.resolve_ipv4(b"127.0.0.1"), Some([127, 0, 0, 1]));
        assert_eq!(NativeHost.resolve_ipv4(b"\xff"), None);
        assert_eq!(NativeHost.resolve_ipv4(b"::1"), None);
    }

    #[test]
    fn available_memory_query_is_supported() {
        assert!(NativeHost.available_memory_bytes().is_ok());
    }
}
