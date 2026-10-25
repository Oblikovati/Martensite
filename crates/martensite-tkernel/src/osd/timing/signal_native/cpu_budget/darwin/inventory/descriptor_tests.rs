//! A fresh process keeps descriptor and reaping fixtures out of the unit-test host (#289).
use std::fs::File;
use std::os::fd::{AsRawFd, OwnedFd};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

const CHILD_ROLE: &str = "MARTENSITE_CPU_DESCRIPTOR_CASE";
const EXECUTED_STATUS: i32 = 73;

#[test]
fn closes_late_high_handles_after_limit_reduction() {
    if std::env::var(CHILD_ROLE).as_deref() == Ok("isolated") {
        run_inventory_fixture();
    }
    let scope = module_path!().split_once("::").unwrap().1;
    let selector = format!("{scope}::closes_late_high_handles_after_limit_reduction");
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", &selector, "--nocapture"])
        .env(CHILD_ROLE, "isolated")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for_fixture(&mut child);
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(EXECUTED_STATUS), "{output:?}");
}

fn wait_for_fixture(child: &mut Child) {
    let deadline = Instant::now() + Duration::from_secs(12);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            let _ = child.kill();
            child.wait().unwrap();
            panic!("inventory child: expected completion inside the 12-second safety guard");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn run_inventory_fixture() -> ! {
    super::preflight().unwrap();
    make_high_descriptor_available();
    let (reader, writer) = nix::unistd::pipe().unwrap();
    let high = high_writer(&writer);
    drop(writer);
    let late = std::thread::spawn(|| File::open("/dev/null").unwrap())
        .join()
        .unwrap();
    lower_fixture_limit();
    fork_descriptor_child(high.as_raw_fd(), late.as_raw_fd());
    assert_eq!(descriptor_state(high.as_raw_fd()), Ok(true));
    assert_eq!(descriptor_state(late.as_raw_fd()), Ok(true));
    drop((reader, high, late));
    std::process::exit(EXECUTED_STATUS);
}

fn high_writer(writer: &OwnedFd) -> OwnedFd {
    let duplicated = rustix::io::fcntl_dupfd_cloexec(writer, 512).unwrap();
    assert!(
        duplicated.as_raw_fd() >= 512,
        "received fd {}; expected at least 512",
        duplicated.as_raw_fd()
    );
    duplicated
}

fn make_high_descriptor_available() {
    use rustix::process::{Resource, Rlimit, getrlimit, setrlimit};
    let original = getrlimit(Resource::Nofile);
    if original.current.is_some_and(|value| value < 1024) {
        setrlimit(
            Resource::Nofile,
            Rlimit {
                current: Some(1024),
                maximum: original.maximum,
            },
        )
        .unwrap();
    }
}

fn lower_fixture_limit() {
    use rustix::process::{Resource, Rlimit, getrlimit, setrlimit};
    let previous = getrlimit(Resource::Nofile);
    setrlimit(
        Resource::Nofile,
        Rlimit {
            current: Some(64),
            maximum: previous.maximum,
        },
    )
    .unwrap();
}

fn fork_descriptor_child(high: i32, late: i32) {
    let mut entries = [const { std::mem::MaybeUninit::uninit() }; super::STACK_ENTRIES];
    reset_fixture_reaping();
    // SAFETY: only native calls and checked stack storage run in the child before _exit.
    let child = unsafe { libc::fork() };
    assert!(child >= 0);
    if child == 0 {
        close_snapshot_child(high, late, &mut entries);
    }
    wait_owned_snapshot_child(child);
}

fn reset_fixture_reaping() {
    // SAFETY: this selected fresh fixture owns its child and has no application reaper.
    let previous = unsafe { libc::signal(libc::SIGCHLD, libc::SIG_DFL) };
    assert_ne!(previous, libc::SIG_ERR);
}

fn wait_owned_snapshot_child(child: i32) {
    let mut status = 0;
    // SAFETY: default reaping reserves this unreaped owned PID; no unrelated reaper runs.
    assert_eq!(unsafe { libc::waitpid(child, &mut status, 0) }, child);
    assert!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), 0);
}

fn close_snapshot_child(
    high: i32,
    late: i32,
    entries: &mut [std::mem::MaybeUninit<libc::proc_fdinfo>; super::STACK_ENTRIES],
) -> ! {
    // SAFETY: the timer is private to this single-threaded fixture child, not the test host.
    unsafe {
        let empty = 0;
        if libc::signal(libc::SIGALRM, libc::SIG_DFL) == libc::SIG_ERR
            || libc::sigprocmask(libc::SIG_SETMASK, &empty, std::ptr::null_mut()) != 0
        {
            libc::_exit(75);
        }
        libc::alarm(12);
    }
    let closed = super::close_inherited(libc::STDERR_FILENO, entries).is_ok();
    let verified = descriptor_state(high) == Ok(false) && descriptor_state(late) == Ok(false);
    // SAFETY: no copied Rust destructor or unit-test runtime may run after descriptor closure.
    unsafe { libc::_exit(if closed && verified { 0 } else { 74 }) }
}

fn descriptor_state(descriptor: i32) -> Result<bool, i32> {
    // SAFETY: F_GETFD reads this process's descriptor table without changing its handles.
    let result = unsafe { libc::fcntl(descriptor, libc::F_GETFD) };
    if result >= 0 {
        return Ok(true);
    }
    match super::errno() {
        libc::EBADF => Ok(false),
        failure => Err(failure),
    }
}
