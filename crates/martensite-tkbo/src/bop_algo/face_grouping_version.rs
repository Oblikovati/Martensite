//! Versioned identity comparison for face-grouping keys.
/// Select the replay contract for same-domain edge occurrences.
/// Example: `let version = FaceGroupingVersion::ExactOccurrencesV2;`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum FaceGroupingVersion {
    /// Historical one-way membership; retained for explicit replay only.
    LegacyOccurrencesV1 = 1,
    /// Equal identity multiplicities, independent of order and hash collisions.
    #[default]
    ExactOccurrencesV2 = 2,
}

#[cfg(test)]
mod tests {
    use super::FaceGroupingVersion;
    use crate::bop_algo::Options;

    #[test]
    fn grouping_versions_are_send_sync_and_keep_recorded_numeric_ids() {
        fn require_send_sync<T: Send + Sync>() {}
        require_send_sync::<FaceGroupingVersion>();
        assert_eq!(FaceGroupingVersion::LegacyOccurrencesV1 as u32, 1);
        assert_eq!(FaceGroupingVersion::default() as u32, 2);
    }

    #[test]
    fn option_clone_and_clear_preserve_explicit_legacy_replay_selection() {
        let mut options = Options::new();
        assert_eq!(
            options.face_grouping_version(),
            FaceGroupingVersion::ExactOccurrencesV2
        );
        options.set_face_grouping_version(FaceGroupingVersion::LegacyOccurrencesV1);
        let copied = options.clone();
        options.clear();
        assert_eq!(
            options.face_grouping_version(),
            FaceGroupingVersion::LegacyOccurrencesV1
        );
        assert_eq!(
            copied.face_grouping_version(),
            options.face_grouping_version()
        );
    }
}
