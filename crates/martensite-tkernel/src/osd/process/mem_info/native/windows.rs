//! WinSafe owns the native structures and heap lock lifetime (#334).
#![forbid(unsafe_code)]
use winsafe::{HHEAP, HPROCESS, co};

pub(super) fn counters() -> Option<[usize; 7]> {
    let process = HPROCESS::GetCurrentProcess();
    let usage = process.GetProcessMemoryInfo().ok()?;
    Some([
        usage.PrivateUsage,
        virtual_bytes(&process).unwrap_or(usize::MAX),
        usage.WorkingSetSize,
        usage.PeakWorkingSetSize,
        usage.PagefileUsage,
        usage.PeakPagefileUsage,
        heap_bytes().unwrap_or(usize::MAX),
    ])
}

fn virtual_bytes(process: &HPROCESS) -> Option<usize> {
    let (mut address, mut total) = (0usize, 0usize);
    loop {
        let region = match process.VirtualQueryEx(Some(address as *mut _)) {
            Ok(region) => region,
            Err(error) if error == co::ERROR::INVALID_PARAMETER => return Some(total),
            Err(_) => return None,
        };
        if region.State != co::MEM_STATE::FREE {
            total = total.checked_add(region.RegionSize)?;
        }
        let next = (region.BaseAddress as usize).checked_add(region.RegionSize)?;
        if next <= address {
            return None;
        }
        address = next;
    }
}

fn heap_bytes() -> Option<usize> {
    let heap = HHEAP::GetProcessHeap().ok()?;
    let _lock = heap.HeapLock().ok()?;
    let mut total = 0usize;
    // Do not allocate while walking the locked heap; concurrent allocations wait.
    for entry in heap.HeapWalk() {
        let entry = entry.ok()?;
        if entry.wFlags.has(co::PROCESS_HEAP::ENTRY_BUSY) {
            total = total.checked_add(entry.cbData as usize)?;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use crate::osd::{MemCounter, MemInfo, MemInfoSource};

    struct MemorySnapshot(Option<[usize; 7]>);

    impl MemInfoSource for MemorySnapshot {
        fn heap_usage(&self) -> usize {
            0
        }
        fn proc_status(&self) -> Option<String> {
            None
        }
        fn task_counters(&self) -> Option<(usize, usize, usize)> {
            None
        }
        fn windows_counters(&self) -> Option<[usize; 7]> {
            self.0
        }
    }

    #[test]
    fn windows_update_preserves_counter_order_and_disabled_state() {
        let mut info =
            MemInfo::with_source(MemorySnapshot(Some([11, 22, 33, 44, 55, 66, 77])), false);
        info.set_counter_active(MemCounter::WorkingSet, false);
        info.update();
        for (index, counter) in MemCounter::ALL.into_iter().enumerate() {
            let expected = if counter == MemCounter::WorkingSet {
                usize::MAX
            } else {
                (index + 1) * 11
            };
            assert_eq!(info.value(counter), expected);
        }
    }

    #[test]
    fn failed_windows_snapshot_leaves_counters_unknown() {
        let info = MemInfo::with_source(MemorySnapshot(None), true);
        for counter in MemCounter::ALL {
            assert!(!info.has_value(counter));
        }
    }

    #[test]
    fn native_process_counters_are_available_and_ordered() {
        let counters = super::counters().unwrap();
        assert!(counters[2] > 0);
        assert!(counters[3] >= counters[2]);
        assert!(counters[5] >= counters[4]);
    }

    #[test]
    fn native_address_space_and_heap_queries_succeed() {
        assert!(super::virtual_bytes(&winsafe::HPROCESS::GetCurrentProcess()).unwrap() > 0);
        assert!(super::heap_bytes().unwrap() > 0);
    }
}
