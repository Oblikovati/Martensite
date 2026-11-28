use super::*;

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::*;

    /// The power-on Darwin environment: every x87 and SSE exception masked.
    fn masked() -> DarwinFenv {
        DarwinFenv {
            control: 0x037f,
            mxcsr: 0x1f80,
            ..DarwinFenv::default()
        }
    }

    #[test]
    fn enabling_unmasks_exactly_the_osd_exceptions_in_both_units() {
        let env = with_traps(masked(), true);
        assert_eq!(env.control, 0x037f & !0x0d);
        assert_eq!(env.mxcsr, 0x1f80 & !(0x0d << 7));
        assert!(traps_enabled(env));
    }

    #[test]
    fn disabling_restores_the_masks_and_clears_pending_flags() {
        let enabled = with_traps(masked(), true);
        let raised = DarwinFenv {
            status: 0x3f,
            mxcsr: enabled.mxcsr | 0x3f,
            ..enabled
        };
        let env = with_traps(raised, false);
        assert_eq!((env.control, env.status, env.mxcsr), (0x037f, 0, 0x1f80));
        assert!(!traps_enabled(env));
    }
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use super::*;

    #[test]
    fn enabling_sets_the_fpcr_trap_bits_and_clears_pending_flags() {
        let raised = DarwinFenv {
            fpsr: 0x9f,
            fpcr: 0,
        };
        let env = with_traps(raised, true);
        assert_eq!((env.fpsr, env.fpcr), (0, 0x0700));
        assert!(traps_enabled(env));
        assert!(!traps_enabled(with_traps(env, false)));
    }
}

#[test]
fn native_round_trip_on_a_fresh_thread_reports_the_hardware_state() {
    std::thread::spawn(|| {
        set_floating_signal(true);
        let catching = to_catch_floating_signals();
        set_floating_signal(false);
        // Intel Macs honor the masks; Apple Silicon reads its trap-enable bits as zero.
        assert_eq!(catching, cfg!(target_arch = "x86_64"));
        assert!(!to_catch_floating_signals());
    })
    .join()
    .expect("the trap round trip runs on its own thread");
}
