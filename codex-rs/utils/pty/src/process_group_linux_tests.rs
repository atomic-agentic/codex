use std::io;
use std::os::unix::process::CommandExt;
use std::process::Command;

use pretty_assertions::assert_eq;

use super::parent_death_signal_unavailable;
use super::set_parent_death_signal;

#[test]
fn a_refused_or_unsupported_prctl_is_tolerated() {
    for errno in [libc::EPERM, libc::EACCES, libc::ENOSYS, libc::EINVAL] {
        assert!(
            parent_death_signal_unavailable(&io::Error::from_raw_os_error(errno)),
            "errno {errno} should be tolerated"
        );
    }
}

#[test]
fn other_prctl_failures_are_still_errors() {
    for errno in [libc::EFAULT, libc::EBADF, libc::ENOMEM] {
        assert!(
            !parent_death_signal_unavailable(&io::Error::from_raw_os_error(errno)),
            "errno {errno} should not be tolerated"
        );
    }
}

#[test]
fn a_child_with_the_parent_death_signal_runs() {
    let parent_pid = unsafe { libc::getpid() };
    let mut command = Command::new("/bin/true");
    // SAFETY: the closure makes only async-signal-safe system calls.
    unsafe {
        command.pre_exec(move || set_parent_death_signal(parent_pid));
    }
    let status = command.status().expect("spawn /bin/true");
    assert_eq!(status.code(), Some(0));
}
