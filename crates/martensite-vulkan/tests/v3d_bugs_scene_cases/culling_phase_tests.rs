use super::*;

#[derive(Debug, PartialEq, Eq)]
enum CullingCommand {
    Fps,
    Counters(i32),
    Dump(&'static str),
    Displayed(String),
}

#[derive(Default)]
struct NamedCullingCommands {
    commands: Vec<CullingCommand>,
    fail_dump: Option<&'static str>,
}

impl CullingPhaseCommands for NamedCullingCommands {
    fn print_fps(&mut self) -> Result<(), Box<dyn Error>> {
        self.commands.push(CullingCommand::Fps);
        Ok(())
    }

    fn counters(&mut self, flags: Perf) -> Result<(), Box<dyn Error>> {
        self.commands.push(CullingCommand::Counters(flags.0));
        Ok(())
    }

    fn dump(&mut self, name: &'static str) -> Result<(), Box<dyn Error>> {
        self.commands.push(CullingCommand::Dump(name));
        if self.fail_dump == Some(name) {
            return Err(format!("owned dump {name}; expected successful source capture").into());
        }
        Ok(())
    }

    fn displayed(&mut self, kind: &str) {
        self.commands
            .push(CullingCommand::Displayed(kind.to_owned()));
    }
}

#[test]
fn complete_phase_keeps_fps_before_none_ref_and_verbose_counter_image() {
    let mut commands = NamedCullingCommands::default();
    source_culling_phase(&mut commands, "off_ref.png", "off.png", "spheres").unwrap();
    assert_eq!(
        commands.commands,
        vec![
            CullingCommand::Fps,
            CullingCommand::Counters(0),
            CullingCommand::Dump("off_ref.png"),
            CullingCommand::Counters(0x3fc),
            CullingCommand::Dump("off.png"),
            CullingCommand::Displayed("spheres".into()),
        ]
    );
}

#[test]
fn second_phase_starts_with_the_previous_hud_state_still_selected() {
    let mut commands = NamedCullingCommands::default();
    source_culling_phase(&mut commands, "off_ref.png", "off.png", "boxes").unwrap();
    source_culling_phase(&mut commands, "on_ref.png", "on.png", "boxes").unwrap();
    assert_eq!(commands.commands[4], CullingCommand::Dump("off.png"));
    assert_eq!(commands.commands[6], CullingCommand::Fps);
    assert_eq!(commands.commands[7], CullingCommand::Counters(0));
    assert_eq!(commands.commands[10], CullingCommand::Dump("on.png"));
}

#[test]
fn a_failed_ref_dump_cannot_become_a_successful_complete_phase() {
    let mut commands = NamedCullingCommands {
        fail_dump: Some("off_ref.png"),
        ..Default::default()
    };
    assert!(source_culling_phase(&mut commands, "off_ref.png", "off.png", "spheres").is_err());
    assert_eq!(commands.commands.len(), 3);
    assert_eq!(
        commands.commands.last(),
        Some(&CullingCommand::Dump("off_ref.png"))
    );
}
