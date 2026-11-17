//! Measured package test costs that order PR jobs, split long suites and
//! balance release shards (#468). Weights are rounded median minutes of the
//! `cargo xtask ci test --package` step in nine PR runs from 2026-11-01 to
//! 2026-11-15, compilation included. A stale weight changes only scheduling;
//! every selected test still runs exactly once.

/// Approximate minutes for packages whose tests take noticeably long.
const MEASURED_MINUTES: [(&str, u32); 24] = [
    ("martensite-vulkan", 46),
    ("martensite-tkbo", 25),
    ("martensite-boolean-cases", 19),
    ("martensite-tkoffset", 12),
    ("martensite-tkdestep", 12),
    ("martensite-tkgeombase", 9),
    ("martensite-tkprim", 9),
    ("martensite-tkdeiges", 9),
    ("martensite-tkhlr", 7),
    ("martensite-tkdeobj", 6),
    ("martensite-tkfillet", 5),
    ("martensite-tkfeat", 5),
    ("martensite-tkxcaf", 5),
    ("martensite-tkdestl", 5),
    ("martensite-tkvcaf", 5),
    ("martensite-tkshhealing", 5),
    ("martensite-tkbool", 4),
    ("martensite-tkmeshvs", 4),
    ("martensite-tkcaf", 3),
    ("martensite-tktopalgo", 3),
    ("martensite-tkmesh", 3),
    ("martensite-tkgeomalgo", 3),
    ("martensite-tkmath", 3),
    ("martensite-tkv3d", 2),
];

/// Packages whose single PR job would bound the whole run; each part runs a
/// disjoint, deterministic subset of the package's tests (#470).
const PR_PARTS: [(&str, usize); 3] = [
    ("martensite-vulkan", 4),
    ("martensite-boolean-cases", 3),
    ("martensite-tkbo", 2),
];

/// Measured minutes for `package`, or 1 for a package without a measurement.
///
/// ```ignore
/// assert_eq!(package_minutes("martensite-vulkan"), 46);
/// ```
pub(crate) fn package_minutes(package: &str) -> u32 {
    lookup(&MEASURED_MINUTES, package).unwrap_or(1)
}

/// How many PR jobs share `package`'s tests; 1 keeps a single job.
///
/// ```ignore
/// assert_eq!(pr_parts("martensite-tkmath"), 1);
/// ```
pub(crate) fn pr_parts(package: &str) -> usize {
    lookup(&PR_PARTS, package).unwrap_or(1)
}

// Below this, a warm cache saves less time than the job spends restoring it,
// and the repository's 10 GB cache budget must hold every warmed store (#472).
const WARM_MINUTES: u32 = 5;

/// Packages whose PR build cache the scheduled warm job refreshes on `main`.
///
/// ```ignore
/// assert!(warm_packages().contains(&"martensite-vulkan"));
/// ```
pub(crate) fn warm_packages() -> Vec<&'static str> {
    MEASURED_MINUTES
        .iter()
        .filter(|(_, minutes)| *minutes >= WARM_MINUTES)
        .map(|(name, _)| *name)
        .collect()
}

fn lookup<T: Copy>(table: &[(&str, T)], package: &str) -> Option<T> {
    table
        .iter()
        .find(|(name, _)| *name == package)
        .map(|(_, value)| *value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measured_packages_report_their_minutes_and_others_default_to_one() {
        assert_eq!(package_minutes("martensite-vulkan"), 46);
        assert_eq!(package_minutes("martensite-process-policy"), 1);
    }

    #[test]
    fn only_listed_packages_are_split_and_every_split_has_two_or_more_parts() {
        assert_eq!(pr_parts("martensite-tkmath"), 1);
        assert!(PR_PARTS.iter().all(|(_, parts)| *parts >= 2));
    }

    #[test]
    fn warm_packages_are_the_long_measured_suites() {
        let warm = warm_packages();
        assert!(warm.contains(&"martensite-vulkan"));
        assert!(!warm.contains(&"martensite-tkmath"));
        assert!(
            warm.iter()
                .all(|name| package_minutes(name) >= WARM_MINUTES)
        );
    }

    #[test]
    fn every_table_names_each_package_once() {
        let names: Vec<_> = MEASURED_MINUTES.iter().map(|(name, _)| *name).collect();
        let unique: std::collections::BTreeSet<_> = names.iter().collect();
        assert_eq!(unique.len(), names.len(), "{names:?}");
    }
}
