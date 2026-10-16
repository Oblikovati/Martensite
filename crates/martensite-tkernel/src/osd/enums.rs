//! The OSD enumerations, with their declaration order as `i32` discriminants.
//!
//! Each enum keeps a fixed order because callers (and the protection tables) index
//! by the raw value; `from_raw` is the checked inverse of `as i32`.

macro_rules! osd_enum {
    ($(#[$meta:meta])* $name:ident { $($(#[$vmeta:meta])* $variant:ident),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(i32)]
        pub enum $name {
            $($(#[$vmeta])* $variant),+
        }
        impl $name {
            /// Every value in declaration order; `ALL[raw as usize]` inverts `as i32`.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
            /// Converts a raw discriminant, or `None` outside the enumeration;
            #[doc = concat!("`", stringify!($name), "::from_raw(0)`.")]
            pub fn from_raw(raw: i32) -> Option<Self> {
                usize::try_from(raw).ok().and_then(|index| Self::ALL.get(index).copied())
            }
        }
    };
}

osd_enum! {
    /// The operating-system dialect a path is spelled in.
    SysType {
        /// An unknown dialect.
        Unknown,
        /// The dialect of the running host.
        Default,
        /// BSD Unix.
        UnixBsd,
        /// System V Unix.
        UnixSystemV,
        /// VMS.
        Vms,
        /// OS/2.
        Os2,
        /// OSF/1.
        Osf,
        /// Classic Mac OS.
        MacOs,
        /// Taligent.
        Taligent,
        /// Windows NT.
        WindowsNt,
        /// Red Hat Linux.
        LinuxRedHat,
        /// AIX.
        Aix,
    }
}

osd_enum! {
    /// Access rights as bits: read (1), write (2), execute (4) and delete (8) bits.
    SingleProtection {
        /// No right.
        None,
        /// Read.
        R,
        /// Write.
        W,
        /// Read and write.
        Rw,
        /// Execute.
        X,
        /// Read and execute.
        Rx,
        /// Write and execute.
        Wx,
        /// Read, write and execute.
        Rwx,
        /// Delete.
        D,
        /// Read and delete.
        Rd,
        /// Write and delete.
        Wd,
        /// Read, write and delete.
        Rwd,
        /// Execute and delete.
        Xd,
        /// Read, execute and delete.
        Rxd,
        /// Write, execute and delete.
        Wxd,
        /// Every right.
        Rwxd,
    }
}

osd_enum! {
    /// The origin of a file seek.
    FromWhere {
        /// From the start of the file.
        FromBeginning,
        /// From the current position.
        FromHere,
        /// From the end of the file.
        FromEnd,
    }
}

osd_enum! {
    /// The kind of a file-system node.
    KindFile {
        /// A regular file.
        File,
        /// A directory.
        Directory,
        /// A symbolic link.
        Link,
        /// A socket.
        Socket,
        /// Any other node kind.
        Unknown,
    }
}

osd_enum! {
    /// The lock held on an open file.
    LockType {
        /// No lock.
        NoLock,
        /// A shared read lock.
        ReadLock,
        /// A write lock.
        WriteLock,
        /// An exclusive lock.
        ExclusiveLock,
    }
}

osd_enum! {
    /// The access mode of an open file.
    OpenMode {
        /// Read only.
        ReadOnly,
        /// Write only.
        WriteOnly,
        /// Read and write.
        ReadWrite,
    }
}

osd_enum! {
    /// The hardware vendor of a host.
    OemType {
        /// Unknown vendor.
        Unavailable,
        /// Sun.
        Sun,
        /// DEC.
        Dec,
        /// SGI.
        Sgi,
        /// NEC.
        Nec,
        /// Apple.
        Mac,
        /// PC.
        Pc,
        /// HP.
        Hp,
        /// IBM.
        Ibm,
        /// VAX.
        Vax,
        /// Linux.
        Lin,
        /// AIX.
        Aix,
    }
}

osd_enum! {
    /// When a shared library resolves its symbols.
    LoadMode {
        /// Resolve symbols on first use (`RTLD_LAZY`).
        RtldLazy,
        /// Resolve symbols at load time (`RTLD_NOW`).
        RtldNow,
    }
}

osd_enum! {
    /// The OSD type that recorded an error, selecting its message.
    WhoAmI {
        /// A directory.
        Directory,
        /// A directory iterator.
        DirectoryIterator,
        /// An environment variable.
        Environment,
        /// A file.
        File,
        /// A file-system node.
        FileNode,
        /// A file iterator.
        FileIterator,
        /// A path.
        Path,
        /// The process.
        Process,
        /// A protection.
        Protection,
        /// The host.
        Host,
        /// A disk.
        Disk,
        /// A chronometer.
        Chronometer,
        /// A timer.
        Timer,
        /// A shared library.
        Package,
        /// An environment iterator.
        EnvironmentIterator,
    }
}

osd_enum! {
    /// How signal setup treats the process signal handlers.
    SignalMode {
        /// Do not set or remove signal handlers.
        AsIs,
        /// Set the kernel's signal handlers.
        Set,
        /// Set the kernel's handlers only where none is set.
        SetUnhandled,
        /// Restore the system default handlers.
        Unset,
    }
}
