//! A path split into node, user, password, disk, trek, name and extension,
//! with a portable `|` trek separator and `^` parent marker.
//!
//! There are two independent rule sets, and both are kept: the Unix rules
//! parse and spell the Unix, DOS, VMS and MacOS dialects by `SysType`; the Windows
//! rules accept only the Windows dialect, parse with `_splitpath` and keep the trek
//! without its trailing `|`. `PathSyntax` selects the rules a path follows; `native()` is
//! the running host's.
use super::SysType;
use crate::{standard::Failure, tcollection::AsciiString};

mod predicates;
mod relative;
mod text;
mod trek;
mod unix_host;
mod windows_host;
use text::Text;

/// The rule set a path follows; `PathSyntax::Unix`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathSyntax {
    /// The Unix rules (Linux, macOS and the other Unix hosts).
    Unix,
    /// The Windows rules.
    Windows,
}
impl PathSyntax {
    /// Selects the current platform's rules; `PathSyntax::native()`.
    pub const fn native() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else {
            Self::Unix
        }
    }
    /// The host system of these rules: WindowsNT, UnixBSD on macOS, and
    /// LinuxREDHAT on the other Unix hosts; `PathSyntax::Unix.host_sys_type()`.
    pub const fn host_sys_type(self) -> SysType {
        match self {
            Self::Windows => SysType::WindowsNt,
            Self::Unix if cfg!(target_vendor = "apple") => SysType::UnixBsd,
            Self::Unix => SysType::LinuxRedHat,
        }
    }
}

/// The seven path components; `path.values().name`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PathComponents {
    /// Host (node) name.
    pub node: AsciiString,
    /// User name.
    pub user_name: AsciiString,
    /// Password.
    pub password: AsciiString,
    /// Disk (drive or device).
    pub disk: AsciiString,
    /// Directory trek in portable syntax.
    pub trek: AsciiString,
    /// File name without extension.
    pub name: AsciiString,
    /// Extension including its dot.
    pub extension: AsciiString,
}

/// A path; `CadPath::new("/tmp/part.step").extension()`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CadPath {
    syntax: PathSyntax,
    node: Text,
    user_name: Text,
    password: Text,
    disk: Text,
    trek: Text,
    name: Text,
    extension: Text,
}
impl Default for CadPath {
    fn default() -> Self {
        Self::empty(PathSyntax::native())
    }
}
impl CadPath {
    /// Every component empty; `CadPath::empty(PathSyntax::Unix)`.
    pub const fn empty(syntax: PathSyntax) -> Self {
        Self {
            syntax,
            node: Text::new(),
            user_name: Text::new(),
            password: Text::new(),
            disk: Text::new(),
            trek: Text::new(),
            name: Text::new(),
            extension: Text::new(),
        }
    }
    /// Parses `name` in the host's own dialect; `CadPath::new("/tmp/part.step")`.
    pub fn new(path: impl AsRef<[u8]>) -> Self {
        Self::with_syntax(path, PathSyntax::native())
    }
    /// Parses `name` in the given rule set's own dialect, which never fails;
    /// `CadPath::with_syntax("/tmp/part.step", PathSyntax::Unix)`.
    pub fn with_syntax(path: impl AsRef<[u8]>, syntax: PathSyntax) -> Self {
        let path: &[u8] = c_bytes(path.as_ref());
        match syntax {
            PathSyntax::Unix => unix_host::parse_unix(path),
            PathSyntax::Windows => windows_host::parse(path),
        }
    }
    /// Parses `name` in dialect `sys_type` on the running host; `CadPath::with_sys_type("disk:[a]b.c", SysType::Vms)?`.
    pub fn with_sys_type(path: impl AsRef<[u8]>, sys_type: SysType) -> Result<Self, Failure> {
        Self::parse(path, sys_type, PathSyntax::native())
    }
    /// Parses `name` in dialect `sys_type` with explicit rules; `CadPath::parse("a:b", SysType::MacOs, PathSyntax::Unix)?`.
    ///
    /// The Unix build raises `Standard_OutOfRange` for some malformed VMS names; the
    /// Windows build raises `Standard_ProgramError` for any dialect but Default/WindowsNT.
    pub fn parse(
        path: impl AsRef<[u8]>,
        sys_type: SysType,
        syntax: PathSyntax,
    ) -> Result<Self, Failure> {
        let path: &[u8] = c_bytes(path.as_ref());
        match syntax {
            PathSyntax::Unix => unix_host::parse(path, sys_type),
            PathSyntax::Windows => {
                windows_host::check_sys_type(sys_type, "OSD_Path")?;
                Ok(windows_host::parse(path))
            }
        }
    }
    /// Builds a path from its seven components on the running host;
    /// `CadPath::from_values("", "", "", "", "|tmp|", "part", ".step")`.
    pub fn from_values(
        node: impl AsRef<[u8]>,
        user_name: impl AsRef<[u8]>,
        password: impl AsRef<[u8]>,
        disk: impl AsRef<[u8]>,
        trek: impl AsRef<[u8]>,
        name: impl AsRef<[u8]>,
        extension: impl AsRef<[u8]>,
    ) -> Self {
        let mut path: Self = Self::default();
        path.set_values(node, user_name, password, disk, trek, name, extension);
        path
    }
    /// The build whose rules this path follows; `path.syntax()`.
    pub const fn syntax(&self) -> PathSyntax {
        self.syntax
    }
    /// Every component; the Windows build appends `|` to a nonempty trek;
    /// `path.values().trek`.
    pub fn values(&self) -> PathComponents {
        PathComponents {
            node: self.node(),
            user_name: self.user_name(),
            password: self.password(),
            disk: self.disk(),
            trek: self.trek(),
            name: self.name(),
            extension: self.extension(),
        }
    }
    /// Replaces every component; the Windows build prefixes a missing
    /// extension dot and removes duplicate separators from the trek;
    /// `path.set_values("", "", "", "", "|tmp|", "part", ".step")`.
    #[allow(clippy::too_many_arguments)] // One argument per path component.
    pub fn set_values(
        &mut self,
        node: impl AsRef<[u8]>,
        user_name: impl AsRef<[u8]>,
        password: impl AsRef<[u8]>,
        disk: impl AsRef<[u8]>,
        trek: impl AsRef<[u8]>,
        name: impl AsRef<[u8]>,
        extension: impl AsRef<[u8]>,
    ) {
        self.node = c_bytes(node.as_ref()).to_vec();
        self.user_name = c_bytes(user_name.as_ref()).to_vec();
        self.password = c_bytes(password.as_ref()).to_vec();
        self.disk = c_bytes(disk.as_ref()).to_vec();
        self.trek = c_bytes(trek.as_ref()).to_vec();
        self.name = c_bytes(name.as_ref()).to_vec();
        self.extension = c_bytes(extension.as_ref()).to_vec();
        if self.syntax == PathSyntax::Windows {
            windows_host::normalize_values(self);
        }
    }
    /// The spelling in the host's own dialect (every Unix host system is
    /// spelled the Unix way); `path.system_name()`.
    pub fn system_name(&self) -> AsciiString {
        let spelled: Text = match self.syntax {
            PathSyntax::Unix => unix_host::unix_name(self),
            PathSyntax::Windows => windows_host::system_name(self),
        };
        AsciiString::from_raw(&spelled)
    }
    /// Example: `path.system_name_as(SysType::Vms)?`.
    ///
    /// The Windows build raises `Standard_ProgramError` for any dialect but
    /// Default/WindowsNT; the Unix build raises `Standard_OutOfRange` for a DOS spelling
    /// of a non-drive disk with an empty trek.
    pub fn system_name_as(&self, sys_type: SysType) -> Result<AsciiString, Failure> {
        if sys_type == SysType::Default {
            return Ok(self.system_name());
        }
        let spelled: Text = match self.syntax {
            PathSyntax::Unix => unix_host::system_name(self, sys_type)?,
            PathSyntax::Windows => {
                windows_host::check_sys_type(sys_type, "SystemName")?;
                windows_host::system_name(self)
            }
        };
        Ok(AsciiString::from_raw(&spelled))
    }
    /// Both builds leave `name` unchanged; `path.expanded_name(&mut name)`.
    pub fn expanded_name(&self, _name: &mut AsciiString) {}
    /// Whether `name` is valid in dialect `sys_type` on the running host; `CadPath::is_valid("a:b", SysType::WindowsNt)`.
    pub fn is_valid(name: impl AsRef<[u8]>, sys_type: SysType) -> bool {
        relative::is_valid(c_bytes(name.as_ref()), sys_type, PathSyntax::native())
    }
    /// The same with `Default` resolved for explicit rules;
    /// `CadPath::is_valid_on("a:b", SysType::Default, PathSyntax::Windows)`.
    pub fn is_valid_on(name: impl AsRef<[u8]>, sys_type: SysType, syntax: PathSyntax) -> bool {
        relative::is_valid(c_bytes(name.as_ref()), sys_type, syntax)
    }
}

/// Component accessors and setters.
impl CadPath {
    /// `Node()`; `path.node()`.
    pub fn node(&self) -> AsciiString {
        AsciiString::from_raw(&self.node)
    }
    /// Example: `path.user_name()`.
    pub fn user_name(&self) -> AsciiString {
        AsciiString::from_raw(&self.user_name)
    }
    /// Example: `path.password()`.
    pub fn password(&self) -> AsciiString {
        AsciiString::from_raw(&self.password)
    }
    /// `Disk()`; `path.disk()`.
    pub fn disk(&self) -> AsciiString {
        AsciiString::from_raw(&self.disk)
    }
    /// The Windows build appends `|` to a nonempty trek; `path.trek()`.
    pub fn trek(&self) -> AsciiString {
        let mut trek: Text = self.trek.clone();
        if self.syntax == PathSyntax::Windows && trek.last().is_some_and(|&byte| byte != b'|') {
            trek.push(b'|');
        }
        AsciiString::from_raw(&trek)
    }
    /// The file name without extension; `path.name()`.
    pub fn name(&self) -> AsciiString {
        AsciiString::from_raw(&self.name)
    }
    /// The extension including its dot; `path.extension()`.
    pub fn extension(&self) -> AsciiString {
        AsciiString::from_raw(&self.extension)
    }
    /// Example: `path.set_node("host")`.
    pub fn set_node(&mut self, name: impl AsRef<[u8]>) {
        self.node = c_bytes(name.as_ref()).to_vec();
    }
    /// Example: `path.set_user_name("user")`.
    pub fn set_user_name(&mut self, name: impl AsRef<[u8]>) {
        self.user_name = c_bytes(name.as_ref()).to_vec();
    }
    /// Example: `path.set_password("secret")`.
    pub fn set_password(&mut self, name: impl AsRef<[u8]>) {
        self.password = c_bytes(name.as_ref()).to_vec();
    }
    /// Example: `path.set_disk("C:")`.
    pub fn set_disk(&mut self, name: impl AsRef<[u8]>) {
        self.disk = c_bytes(name.as_ref()).to_vec();
    }
    /// The Windows build removes duplicate separators; `path.set_trek("|tmp|")`.
    pub fn set_trek(&mut self, name: impl AsRef<[u8]>) {
        self.trek = c_bytes(name.as_ref()).to_vec();
        if self.syntax == PathSyntax::Windows {
            windows_host::remove_duplicates(&mut self.trek);
        }
    }
    /// Example: `path.set_name("part")`.
    pub fn set_name(&mut self, name: impl AsRef<[u8]>) {
        self.name = c_bytes(name.as_ref()).to_vec();
    }
    /// Stored as given on both builds; `path.set_extension(".step")`.
    pub fn set_extension(&mut self, name: impl AsRef<[u8]>) {
        self.extension = c_bytes(name.as_ref()).to_vec();
    }
}

/// The bytes a C-string argument carries: up to the first NUL.
fn c_bytes(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len())]
}
