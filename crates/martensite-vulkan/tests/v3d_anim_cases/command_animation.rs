//! A timeline atom that runs a command at each frame with its formatted progress, as the
//! animation command's callback does (`%pts`, `%localNormalized`).
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkv3d::ais::{AisAnimationBase, AisAnimationProgress, AisAnimationSource};

/// The command of one callback atom, given the progress the frame applies.
pub(super) type FrameCommand =
    Box<dyn Fn(&AisAnimationProgress) -> Result<(), Failure> + Send + Sync>;

/// A named callback atom; `CommandAnimation::scheduled("1", Box::new(|p| Ok(())))`.
pub(super) struct CommandAnimation {
    transient: StandardTransient,
    base: AisAnimationBase,
    command: FrameCommand,
}
martensite_tkernel::impl_transient!(CommandAnimation,"ViewerTest_AnimationProc",dyn AisAnimationSource,transient,
    interfaces:[dyn AisAnimationSource]);

impl CommandAnimation {
    /// A stopped atom `name` running `command`; schedule it like any other atom.
    pub(super) fn scheduled(name: &str, command: FrameCommand) -> Handle<dyn AisAnimationSource> {
        let animation = Self {
            transient: StandardTransient::default(),
            base: AisAnimationBase::new(name),
            command,
        };
        Handle::new(animation).cast()
    }
}

impl AisAnimationSource for CommandAnimation {
    fn animation_base(&self) -> &AisAnimationBase {
        &self.base
    }
    fn update_own(&self, progress: &AisAnimationProgress) -> Result<(), Failure> {
        (self.command)(progress)
    }
}

/// A progress value as the callback substitutes it into the command text: the C `%g`
/// form, read back as a number; `substituted(0.6000000000000001) == 0.6`.
pub(super) fn substituted(value: f64) -> f64 {
    substituted_text(value).parse().expect("a %g number parses")
}

/// The `%g` text of a progress value; `substituted_text(2.0) == "2"`.
pub(super) fn substituted_text(value: f64) -> String {
    String::from_utf8_lossy(AsciiString::from_real(value).as_bytes()).into_owned()
}
