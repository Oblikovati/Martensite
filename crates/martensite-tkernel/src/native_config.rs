//! Preserve native sysconf sentinels and errno through the safe dependency (#89).

use nix::{errno::Errno, unistd::SysconfVar};

pub(crate) fn native_sysconf(selector: SysconfVar) -> libc::c_long {
    let previous_errno = Errno::last_raw();
    let queried = nix::unistd::sysconf(selector);
    native_sysconf_result(previous_errno, queried)
}

fn native_sysconf_result(
    previous_errno: i32,
    queried: nix::Result<Option<libc::c_long>>,
) -> libc::c_long {
    // nix clears errno; preserve the native successful-query contract (#89).
    if Errno::last_raw() == 0 {
        Errno::set_raw(previous_errno);
    }
    queried.ok().flatten().unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_sysconf_results_preserve_value_and_errno_contracts() {
        assert_sysconf_result(Ok(Some(100)), 0, 100, 12345);
        assert_sysconf_result(Ok(None), 0, -1, 12345);
        assert_sysconf_result(Err(Errno::EINVAL), libc::EINVAL, -1, libc::EINVAL);
        assert_sysconf_result(Ok(Some(100)), libc::EIO, 100, libc::EIO);
    }

    fn assert_sysconf_result(
        queried: nix::Result<Option<libc::c_long>>,
        query_errno: i32,
        expected_value: libc::c_long,
        expected_errno: i32,
    ) {
        let original = Errno::last_raw();
        Errno::set_raw(query_errno);
        let value = native_sysconf_result(12345, queried);
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        assert_eq!((value, observed), (expected_value, expected_errno));
    }
}
