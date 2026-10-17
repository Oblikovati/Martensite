# Safe Windows process local-time query

Feature #80 / task #89 research at
`c77e8d94dbb60dc2d32f910fc3c002813b21c7e8`, for
`crates/martensite-tkernel/src/osd/process/native.rs`.
This research preceded the baseline's field-conversion extraction and native
validation. The baseline is now validated; the safe-wrapper candidate awaits its
own native results. See the [current evidence](windows-local-time-wrapper.json).
The unavailable humanizer prose pass remains for review.

## Small replacement candidate

The existing Windows-only winsafe 0.0.29 dependency exposes safe
`GetLocalTime() -> SYSTEMTIME`. Its implementation initializes a SYSTEMTIME
with its derived Default, invokes the native function once, then returns that
same record. There is no status conversion, retry, timezone conversion,
LastError access or allocation in the wrapper. Its record is repr(C), with
eight u16 fields and a derived zero default. Sources at the package's recorded
revision `71ed88c2a0d18b03ee452f6d4261f4c22f443483`:
[wrapper, lines 537–541](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/funcs.rs#L537) and
[SYSTEMTIME declaration](https://github.com/rodrigocfd/winsafe/blob/71ed88c2a0d18b03ee452f6d4261f4c22f443483/src/kernel/structs.rs#L977).

Replace only the Windows local-time acquisition with this wrapper and retain
the seven `i32::from(u16)` field conversions and `Ok(LocalTime { ... })` result.
Unlike the CPU-time wrappers, this wrapper does not discard outputs on a
failure status: the underlying GetLocalTime has no return value. Microsoft's
[GetLocalTime documentation](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlocaltime)
specifies current local date/time, not UTC. Keep this same API rather than
reconstructing local time from an epoch, cached offset or UTC query.

This would remove the one first-party unsafe call in the Windows process
local-time function. It trusts the established dependency's existing FFI, not
a new custom unsafe implementation. No new dependency, public API or version
bump is needed for the candidate itself. The existing Unix branches remain
outside this change.

## Actual mapping and downstream behavior

The current code preserves milliseconds. Only weekday is omitted. Its mapping
is month/day/year/hour/minute/second/millisecond from the corresponding seven
SYSTEMTIME fields, widened losslessly to i32. It performs no normalization,
range check, year offset, rounding or truncation. Preserve even synthetic u16
boundary values in a conversion fixture; these test widening, not values the
operating system promises to produce. The authoritative sources are
[native process code](https://github.com/Oblikovati/Martensite/blob/c77e8d94dbb60dc2d32f910fc3c002813b21c7e8/crates/martensite-tkernel/src/osd/process/native.rs) and
[LocalTime fields](https://github.com/Oblikovati/Martensite/blob/c77e8d94dbb60dc2d32f910fc3c002813b21c7e8/crates/martensite-tkernel/src/osd/process/source.rs).

`Process::system_date` passes those seven values to Date::new with microseconds
zero. Its injected-source error path records the OS error with `GetSystem`,
then attempts the existing default date. The native Windows query remains an
Ok path; do not invent an Err from LastError or alter downstream Date
validation, formatting or error messages. See
[Process::system_date](https://github.com/Oblikovati/Martensite/blob/c77e8d94dbb60dc2d32f910fc3c002813b21c7e8/crates/martensite-tkernel/src/osd/process.rs#L77).
The schema creation-date consumer is separate and selects only date fields;
its existing formatting work does not authorize dropping process milliseconds.

## Existing evidence to reuse

PR #119's explicit initialization work is recorded in
[system-record-initialization.json](system-record-initialization.json).
`clock_initialization_tests::system_time_starts_with_all_fields_zero` asserts
all eight original fields. The existing
`native_windows_system_date_reads_calendar_and_milliseconds` test in
`tests/osd_process_api_behavior.rs` exercises the Windows date path. Keep that
native behavior fixture unchanged. Its calendar-range assertions are useful
but do not independently prove exact field mapping or LastError behavior.
The separate schema initialization record is
[creation-date-initialization.json](creation-date-initialization.json).
These historical records contain pending-validation text; inspect actual run
artifacts before claiming a gate was completed.

## Baseline and replacement gates

1. Before replacing the native call, pin all seven conversions with distinct
   literal fields, nonzero milliseconds, and u16 zero/max boundaries. Assert
   changing only weekday cannot change LocalTime. Extract a small private
   conversion seam if needed, keeping native acquisition unchanged in that
   structural commit. Use an owned named fake only if a query seam is needed;
   do not add unsafe callbacks for tests.
2. Retain the existing all-zero initialization fixture for the original
   baseline. Before deleting its obsolete helper with the replacement, check
   all eight fields of the actual winsafe default record too. Do not retain an
   unused production initializer merely to keep its test green.
3. Obtain native Windows results for the original query and candidate, using
   several named LastError sentinels. Save the test thread's previous error,
   set the incoming sentinel, capture immediately after acquisition, then
   restore before assertions. Compare original and wrapper observations;
   neither the GetLocalTime docs nor general success rules justify assuming
   that the sentinel must be preserved. The wrapper source adds no writes.
   See [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).
4. Do not assert equality between successive real clock samples, monotonic
   local time, UTC equality or a fixed offset. Day boundaries, timezone/DST
   changes and clock synchronization make those invalid general contracts.
   Microsoft explicitly notes that system time can move forward or backward
   in [SYSTEMTIME](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime).
   Deterministic fixtures prove mapping; native smoke tests prove execution
   on the tested host. Do not mutate host time or timezone to obtain coverage.
5. Run the existing process tests, focused conversion tests, Windows build
   check and `cargo xtask pr`; obtain exact-head native validation and unsafe
   inventories. Review Standards/Spec before integration. Remove the replaced
   call and obsolete initializer only after their replacement gates pass.

No DST transition, timezone mutation, clock-adjustment or arbitrary raw
LastError-sentinel runtime experiment is claimed by this note. Those limits
do not establish impossibility: the proposed wrapper calls the same native
primitive with the same initialized record and preserves its returned fields.
