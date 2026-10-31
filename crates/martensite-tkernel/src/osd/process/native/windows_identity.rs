//! Account identity comes from the effective Windows token, not mutable environment variables.
#![forbid(unsafe_code)]
use winsafe::{CreateWellKnownSid, GetUserName, HACCESSTOKEN, co, prelude::*};

pub(super) fn is_super_user() -> Result<bool, i32> {
    let sid = CreateWellKnownSid(co::WELL_KNOWN_SID_TYPE::BuiltinAdministrators, None)
        .map_err(|error| error.raw() as i32)?;
    // A null token queries the effective token; a UAC-filtered administrator is false.
    HACCESSTOKEN::NULL
        .CheckTokenMembership(&sid)
        .map_err(|error| error.raw() as i32)
}

pub(super) fn user_name() -> Result<Vec<u8>, i32> {
    GetUserName()
        .map(String::into_bytes)
        .map_err(|error| error.raw() as i32)
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_account_name_is_nonempty_utf8_without_nul() {
        let name = super::user_name().unwrap();
        assert!(!name.is_empty());
        assert!(!name.contains(&0));
        assert!(std::str::from_utf8(&name).is_ok());
    }

    #[test]
    fn native_administrator_membership_is_available() {
        assert!(super::is_super_user().is_ok());
    }
}
