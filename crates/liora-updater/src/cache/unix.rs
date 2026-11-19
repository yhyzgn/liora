use super::unsafe_cache;
use std::{
    fs, io,
    os::unix::fs::{DirBuilderExt, MetadataExt},
    path::Path,
};

pub(super) struct Guard(libc::uid_t);

impl Guard {
    pub(super) fn new() -> io::Result<Self> {
        // SAFETY: geteuid has no preconditions.
        Ok(Self(unsafe { libc::geteuid() }))
    }

    pub(super) fn check(&self, path: &Path, private: bool) -> io::Result<()> {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.is_dir() {
            return Err(unsafe_cache(path, "expected a directory without an alias"));
        }
        if !trusted_owner(metadata.uid(), self.0, private) {
            return Err(unsafe_cache(path, "directory has an untrusted owner"));
        }
        // On Linux the group bits bound effective rights of named POSIX ACL entries.
        let mask = if private { 0o077 } else { 0o022 };
        if metadata.mode() & mask != 0 {
            return Err(unsafe_cache(path, "directory permissions are too broad"));
        }
        #[cfg(target_os = "macos")]
        macos::check_acl(path, self.0, private)?;
        Ok(())
    }

    pub(super) fn create_private_dir(&self, path: &Path) -> io::Result<()> {
        fs::DirBuilder::new().mode(0o700).create(path)?;
        self.check(path, true)
    }
}

fn trusted_owner(owner: libc::uid_t, user: libc::uid_t, private: bool) -> bool {
    owner == user || (!private && owner == 0)
}
#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use std::{ffi::{CString, c_char, c_int, c_void}, os::unix::ffi::OsStrExt, ptr};

    // Darwin's <sys/acl.h> extended ACL API. These grants are independent of mode bits.
    const ACL_TYPE_EXTENDED: c_int = 0x100;
    const ACL_EXTENDED_ALLOW: c_int = 1;
    const ACL_EXTENDED_DENY: c_int = 2;
    const ACL_ENTRY_ONLY_INHERIT: c_int = 1 << 8;
    const WRITE_PERMISSIONS: u64 = (1 << 2) | (1 << 4) | (1 << 5) | (1 << 6)
        | (1 << 8) | (1 << 10) | (1 << 12) | (1 << 13);
    type AclPtr = *mut c_void;

    unsafe extern "C" {
        fn acl_get_link_np(path: *const c_char, acl_type: c_int) -> AclPtr;
        fn acl_free(p: AclPtr) -> c_int;
        fn acl_get_entry(acl: AclPtr, index: c_int, entry: *mut AclPtr) -> c_int;
        fn acl_get_tag_type(entry: AclPtr, tag: *mut c_int) -> c_int;
        fn acl_get_permset_mask_np(entry: AclPtr, mask: *mut u64) -> c_int;
        fn acl_get_flagset_np(entry: AclPtr, flags: *mut AclPtr) -> c_int;
        fn acl_get_flag_np(flags: AclPtr, flag: c_int) -> c_int;
        fn acl_get_qualifier(entry: AclPtr) -> AclPtr;
        fn mbr_uuid_to_id(uuid: *const u8, id: *mut libc::id_t, id_type: *mut c_int) -> c_int;
    }

    struct AclAllocation(AclPtr);

    impl Drop for AclAllocation {
        fn drop(&mut self) {
            // SAFETY: only constructed from successful allocating ACL calls.
            unsafe { acl_free(self.0) };
        }
    }

    pub(super) fn check_acl(path: &Path, user: libc::uid_t, private: bool) -> io::Result<()> {
        let name = CString::new(path.as_os_str().as_bytes())?;
        // SAFETY: the pathname is NUL-terminated and kept alive through the call.
        let acl = unsafe { acl_get_link_np(name.as_ptr(), ACL_TYPE_EXTENDED) };
        if acl.is_null() {
            let error = io::Error::last_os_error();
            // Darwin reports ENOENT when the already-validated directory has no ACL.
            return if error.raw_os_error() == Some(libc::ENOENT) { Ok(()) } else { Err(error) };
        }
        let acl = AclAllocation(acl);
        let mut entry = ptr::null_mut();
        let mut index = 0; // ACL_FIRST_ENTRY
        loop {
            // SAFETY: the ACL owns the returned entry until the guard is dropped.
            if unsafe { acl_get_entry(acl.0, index, &mut entry) } != 0 {
                let error = io::Error::last_os_error();
                // Darwin's iterator reports EINVAL at the end of a valid ACL.
                return if error.raw_os_error() == Some(libc::EINVAL) { Ok(()) } else { Err(error) };
            }
            index = -1; // ACL_NEXT_ENTRY
            if !entry_is_safe(entry, user, private)? {
                return Err(unsafe_cache(path, "extended ACL grants modification to another user"));
            }
        }
    }

    fn entry_is_safe(entry: AclPtr, user: libc::uid_t, private: bool) -> io::Result<bool> {
        let mut tag = 0;
        let mut mask = 0;
        let mut flags = ptr::null_mut();
        // SAFETY: entry points into a live native ACL. All outputs are initialized on success.
        unsafe {
            if acl_get_tag_type(entry, &mut tag) != 0 || acl_get_permset_mask_np(entry, &mut mask) != 0 {
                return Err(io::Error::last_os_error());
            }
            match tag {
                ACL_EXTENDED_DENY => return Ok(true),
                ACL_EXTENDED_ALLOW => {},
                _ => return Ok(false),
            }
            if mask & WRITE_PERMISSIONS == 0 {
                return Ok(true);
            }
            if !private {
                if acl_get_flagset_np(entry, &mut flags) != 0 {
                    return Err(io::Error::last_os_error());
                }
                match acl_get_flag_np(flags, ACL_ENTRY_ONLY_INHERIT) {
                    1 => return Ok(true),
                    0 => {},
                    _ => return Err(io::Error::last_os_error()),
                }
            }
            let qualifier = acl_get_qualifier(entry);
            if qualifier.is_null() {
                return Err(io::Error::last_os_error());
            }
            let qualifier = AclAllocation(qualifier);
            let mut id = 0;
            let mut kind = 0;
            // membership.h defines ID_TYPE_UID as 0. Neither groups nor unknown IDs are trusted.
            Ok(mbr_uuid_to_id(qualifier.0.cast(), &mut id, &mut kind) == 0
                && kind == 0 && (id == 0 || id == user))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_policy_rejects_other_users_and_requires_private_ownership() {
        assert!(!trusted_owner(1001, 1000, false));
        assert!(!trusted_owner(1001, 1000, true));
        assert!(trusted_owner(0, 1000, false));
        assert!(!trusted_owner(0, 1000, true));
        assert!(trusted_owner(1000, 1000, true));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn extended_acl_cannot_grant_modification_behind_private_mode_bits() {
        let fixture = crate::cache::tests::Fixture::new("macos-acl");
        let base = &fixture.0;
        let guard = Guard::new().unwrap();
        assert!(guard.check(&base, true).is_ok());
        let status = std::process::Command::new("/bin/chmod")
            .args(["+a", "everyone allow delete_child"])
            .arg(&base).status().unwrap();
        assert!(status.success());
        let result = guard.check(&base, true);
        assert!(matches!(result, Err(error) if error.kind() == io::ErrorKind::PermissionDenied));
    }
}
