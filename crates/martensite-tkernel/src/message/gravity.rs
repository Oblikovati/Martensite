//! The Message enumerations: gravity, console colors and alert metrics.

/// Ordered message severity levels; `Gravity::Warning > Gravity::Info`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Gravity {
    /// Low-level algorithm detail.
    Trace = 0,
    /// Informational output.
    Info = 1,
    /// Warning condition.
    Warning = 2,
    /// Nonfatal error.
    Alarm = 3,
    /// Fatal error.
    Fail = 4,
}
impl Gravity {
    /// Every gravity in enumerator order, from `Trace` to `Fail`.
    pub const ALL: [Gravity; 5] = [
        Gravity::Trace,
        Gravity::Info,
        Gravity::Warning,
        Gravity::Alarm,
        Gravity::Fail,
    ];
    /// The enumerator's integer; `Gravity::Fail.index() == 4`.
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// Console text colors; `ConsoleColor::Red`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConsoleColor {
    /// Default (white) color.
    Default = 0,
    /// Black.
    Black,
    /// White.
    White,
    /// Red.
    Red,
    /// Blue.
    Blue,
    /// Green.
    Green,
    /// Yellow.
    Yellow,
    /// Cyan.
    Cyan,
    /// Magenta.
    Magenta,
}

/// Alert metrics; `MetricType::WallClock as i32 == 5`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum MetricType {
    /// No computation.
    None = 0,
    /// Thread CPU user time.
    ThreadCpuUserTime,
    /// Thread CPU system time.
    ThreadCpuSystemTime,
    /// Process CPU user time.
    ProcessCpuUserTime,
    /// Process CPU system time.
    ProcessCpuSystemTime,
    /// Elapsed wall-clock time.
    WallClock,
    /// Private memory.
    MemPrivate,
    /// Virtual memory.
    MemVirtual,
    /// Working set.
    MemWorkingSet,
    /// Peak working set.
    MemWorkingSetPeak,
    /// Swap usage.
    MemSwapUsage,
    /// Peak swap usage.
    MemSwapUsagePeak,
    /// Heap usage.
    MemHeapUsage,
}
impl MetricType {
    /// Every metric in enumerator order.
    pub const ALL: [MetricType; 13] = [
        MetricType::None,
        MetricType::ThreadCpuUserTime,
        MetricType::ThreadCpuSystemTime,
        MetricType::ProcessCpuUserTime,
        MetricType::ProcessCpuSystemTime,
        MetricType::WallClock,
        MetricType::MemPrivate,
        MetricType::MemVirtual,
        MetricType::MemWorkingSet,
        MetricType::MemWorkingSetPeak,
        MetricType::MemSwapUsage,
        MetricType::MemSwapUsagePeak,
        MetricType::MemHeapUsage,
    ];
}
impl crate::ncollection::HashKey for MetricType {
    fn kernel_hash(&self) -> usize {
        *self as usize
    }
}
