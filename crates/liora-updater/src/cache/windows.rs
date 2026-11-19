use super::unsafe_cache;
use std::{
    ffi::c_void,
    fs::OpenOptions,
    io,
    mem::size_of,
    os::windows::{ffi::OsStrExt, fs::{MetadataExt, OpenOptionsExt}, io::{AsRawHandle, FromRawHandle, OwnedHandle}},
    path::{Component, Path, Prefix},
    ptr,
};
use windows_sys::Win32::{
    Foundation::{ERROR_SUCCESS, GENERIC_ALL, GENERIC_WRITE, LocalFree},
    Security::{
        ACE_HEADER, ACL, CopySid, DACL_SECURITY_INFORMATION, EqualSid, GetAce,
        GetTokenInformation, INHERIT_ONLY_ACE, IsValidAcl, IsWellKnownSid,
        OWNER_SECURITY_INFORMATION, PSID, SECURITY_ATTRIBUTES, SECURITY_MAX_SID_SIZE,
        TOKEN_QUERY, TOKEN_USER, TokenUser, WinBuiltinAdministratorsSid, WinCreatorOwnerRightsSid,
        WinCreatorOwnerSid, WinLocalSystemSid,
        Authorization::{ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
            ConvertStringSidToSidW, GetSecurityInfo, SDDL_REVISION_1, SE_FILE_OBJECT},
    },
    Storage::FileSystem::{
        CreateDirectoryW, DELETE, FILE_ADD_FILE, FILE_ADD_SUBDIRECTORY, FILE_ATTRIBUTE_REPARSE_POINT,
        FILE_DELETE_CHILD, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
        FILE_READ_ATTRIBUTES, FILE_WRITE_ATTRIBUTES, FILE_WRITE_EA, GetDriveTypeW,
        GetVolumeInformationByHandleW,
        READ_CONTROL, WRITE_DAC, WRITE_OWNER,
    },
    System::{
        SystemServices::{ACCESS_ALLOWED_ACE_TYPE, ACCESS_DENIED_ACE_TYPE,
            ACCESS_DENIED_CALLBACK_ACE_TYPE, ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE,
            ACCESS_DENIED_OBJECT_ACE_TYPE, FILE_PERSISTENT_ACLS},
        Threading::{GetCurrentProcess, OpenProcessToken},
        WindowsProgramming::{DRIVE_FIXED, DRIVE_RAMDISK, DRIVE_REMOVABLE},
    },
};

const REPLACEMENT_RIGHTS: u32 = FILE_DELETE_CHILD | FILE_WRITE_ATTRIBUTES | FILE_WRITE_EA
    | DELETE | WRITE_DAC | WRITE_OWNER | GENERIC_WRITE | GENERIC_ALL;
const SID_WORDS: usize = SECURITY_MAX_SID_SIZE as usize / size_of::<u32>();
// The operating system servicing identity can own protected Windows directories.
const TRUSTED_INSTALLER: &str = "S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464";

fn local_drive_type(kind: u32) -> bool {
    matches!(kind, DRIVE_FIXED | DRIVE_REMOVABLE | DRIVE_RAMDISK)
}

pub(super) struct Guard {
    user: [u32; SID_WORDS],
    servicing: LocalAllocation,
}

impl Guard {
    pub(super) fn new() -> io::Result<Self> {
        let mut token = ptr::null_mut();
        // SAFETY: the pseudo-handle identifies this process and token is a valid out-pointer.
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: OpenProcessToken returned an owned handle.
        let token = unsafe { OwnedHandle::from_raw_handle(token) };
        #[repr(C)]
        struct UserBuffer {
            info: TOKEN_USER,
            sid: [u32; SID_WORDS],
        }
        let mut info = UserBuffer { info: TOKEN_USER::default(), sid: [0; SID_WORDS] };
        let mut returned = 0;
        let mut user = [0u32; SID_WORDS];
        // SAFETY: the aligned buffer holds TOKEN_USER plus the maximum SID size.
        // Copy the SID before that buffer goes out of scope.
        unsafe {
            if GetTokenInformation(token.as_raw_handle(), TokenUser, ptr::from_mut(&mut info).cast(),
                size_of::<UserBuffer>() as u32, &mut returned) == 0
                || CopySid(SECURITY_MAX_SID_SIZE, user.as_mut_ptr().cast(), info.info.User.Sid) == 0
            {
                return Err(io::Error::last_os_error());
            }
        }
        let text: Vec<_> = TRUSTED_INSTALLER.encode_utf16().chain(Some(0)).collect();
        let mut servicing = ptr::null_mut();
        // SAFETY: the string is NUL-terminated and the out-pointer is writable.
        if unsafe { ConvertStringSidToSidW(text.as_ptr(), &mut servicing) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self { user, servicing: LocalAllocation(servicing) })
    }

    fn user_sid(&self) -> PSID {
        self.user.as_ptr().cast_mut().cast()
    }

    // SAFETY: sid must point to a valid SID.
    unsafe fn trusted_sid(&self, sid: PSID) -> bool {
        unsafe { EqualSid(sid, self.user_sid()) != 0 || EqualSid(sid, self.servicing.0) != 0
            || IsWellKnownSid(sid, WinLocalSystemSid) != 0
            || IsWellKnownSid(sid, WinBuiltinAdministratorsSid) != 0 }
    }

    pub(super) fn check(&self, path: &Path, private: bool) -> io::Result<()> {
        let Some(Component::Prefix(prefix)) = path.components().next() else {
            return Err(unsafe_cache(path, "expected a resolved drive path"));
        };
        let Prefix::VerbatimDisk(drive) = prefix.kind() else {
            return Err(unsafe_cache(path, "expected a resolved drive path"));
        };
        let dir = OpenOptions::new().read(true).access_mode(READ_CONTROL | FILE_READ_ATTRIBUTES)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT).open(path)?;
        let metadata = dir.metadata()?;
        if !metadata.is_dir() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(unsafe_cache(path, "expected a directory without a reparse point"));
        }
        if path.components().count() == 2 {
            let root = [u16::from(drive), u16::from(b':'), u16::from(b'\\'), 0];
            // SAFETY: root is a NUL-terminated drive root with the required backslash.
            if !local_drive_type(unsafe { GetDriveTypeW(root.as_ptr()) }) {
                return Err(unsafe_cache(path, "cache drive is remote, unknown, or not writable media"));
            }
            let mut flags = 0;
            // SAFETY: flags is writable and the other outputs are optional.
            if unsafe { GetVolumeInformationByHandleW(dir.as_raw_handle(), ptr::null_mut(), 0,
                ptr::null_mut(), ptr::null_mut(), &mut flags, ptr::null_mut(), 0) } == 0
            {
                return Err(io::Error::last_os_error());
            }
            if flags & FILE_PERSISTENT_ACLS == 0 {
                return Err(unsafe_cache(path, "volume does not enforce persistent ACLs"));
            }
        }
        let mut owner = ptr::null_mut();
        let mut dacl = ptr::null_mut();
        let mut descriptor = ptr::null_mut();
        // SAFETY: the handle and all requested out-pointers are valid.
        let status = unsafe { GetSecurityInfo(dir.as_raw_handle(), SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner, ptr::null_mut(), &mut dacl, ptr::null_mut(), &mut descriptor) };
        if status != ERROR_SUCCESS {
            return Err(io::Error::from_raw_os_error(status as i32));
        }
        let _descriptor = LocalAllocation(descriptor);
        // SAFETY: the owner and DACL are borrowed from the live security descriptor.
        let owner_is_trusted = !owner.is_null() && unsafe {
            if private { EqualSid(owner, self.user_sid()) != 0 } else { self.trusted_sid(owner) }
        };
        if !owner_is_trusted {
            return Err(unsafe_cache(path, "directory has an untrusted owner"));
        }
        if dacl.is_null() || !unsafe { self.dacl_is_safe(dacl, private)? } {
            return Err(unsafe_cache(path, "directory DACL permits or cannot rule out modification by another user"));
        }
        Ok(())
    }

    // SAFETY: dacl must point into a live directory security descriptor.
    unsafe fn dacl_is_safe(&self, dacl: *const ACL, private: bool) -> io::Result<bool> {
        if unsafe { IsValidAcl(dacl) } == 0 {
            return Ok(false);
        }
        let rights = if private { REPLACEMENT_RIGHTS | FILE_ADD_FILE | FILE_ADD_SUBDIRECTORY }
            else { REPLACEMENT_RIGHTS };
        for index in 0..u32::from(unsafe { (*dacl).AceCount }) {
            let mut ace = ptr::null_mut();
            if unsafe { GetAce(dacl, index, &mut ace) } == 0 {
                return Err(io::Error::last_os_error());
            }
            let header = unsafe { &*ace.cast::<ACE_HEADER>() };
            let inherit_only = u32::from(header.AceFlags) & INHERIT_ONLY_ACE != 0;
            if !private && inherit_only {
                continue;
            }
            match u32::from(header.AceType) {
                ACCESS_DENIED_ACE_TYPE | ACCESS_DENIED_OBJECT_ACE_TYPE
                | ACCESS_DENIED_CALLBACK_ACE_TYPE | ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE => continue,
                ACCESS_ALLOWED_ACE_TYPE => {},
                _ => return Ok(false),
            }
            // An allow ACE holds a header, a DWORD mask, and a variable-length SID.
            let size = usize::from(header.AceSize);
            if size < 16 {
                return Ok(false);
            }
            let mask = unsafe { ace.cast::<u32>().add(1).read_unaligned() };
            if mask & rights == 0 {
                continue;
            }
            let sid = unsafe { ace.cast::<u8>().add(8) };
            let subauthorities = usize::from(unsafe { *sid.add(1) });
            if size < 16 + size_of::<u32>() * subauthorities {
                return Ok(false);
            }
            let sid = sid.cast();
            // OWNER RIGHTS follows the proven owner. CREATOR OWNER is substituted only on inheritance.
            if !unsafe { self.trusted_sid(sid) || IsWellKnownSid(sid, WinCreatorOwnerRightsSid) != 0
                || (inherit_only && IsWellKnownSid(sid, WinCreatorOwnerSid) != 0) }
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(super) fn create_private_dir(&self, path: &Path) -> io::Result<()> {
        let mut sid = ptr::null_mut();
        // SAFETY: user_sid holds a valid SID copied from the process token.
        if unsafe { ConvertSidToStringSidW(self.user_sid(), &mut sid) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let _sid = LocalAllocation(sid.cast());
        // SAFETY: Windows returned a NUL-terminated SID string.
        let length = (0..).find(|&i| unsafe { *sid.add(i) } == 0).unwrap();
        let user = String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(sid, length) });
        let sddl: Vec<_> = format!("O:{user}D:P(A;OICI;FA;;;{user})(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)")
            .encode_utf16().chain(Some(0)).collect();
        let mut descriptor = ptr::null_mut();
        // SAFETY: the descriptor string is NUL-terminated and the out-pointer is valid.
        if unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorW(sddl.as_ptr(),
            SDDL_REVISION_1, &mut descriptor, ptr::null_mut()) } == 0
        {
            return Err(io::Error::last_os_error());
        }
        let descriptor = LocalAllocation(descriptor);
        let attributes = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: descriptor.0,
            bInheritHandle: 0,
        };
        let name: Vec<_> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        // SAFETY: the path and descriptor remain alive for the call.
        if unsafe { CreateDirectoryW(name.as_ptr(), &attributes) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

struct LocalAllocation(*mut c_void);

impl Drop for LocalAllocation {
    fn drop(&mut self) {
        // SAFETY: created only from APIs that return LocalFree-owned allocations.
        unsafe { LocalFree(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::Security::GetSecurityDescriptorDacl;

    fn check_dacl(text: &str, private: bool) -> bool {
        let text: Vec<_> = text.encode_utf16().chain(Some(0)).collect();
        let mut descriptor = ptr::null_mut();
        // SAFETY: the NUL-terminated SDDL and writable output remain live.
        assert_ne!(unsafe {
            ConvertStringSecurityDescriptorToSecurityDescriptorW(text.as_ptr(),
                SDDL_REVISION_1, &mut descriptor, ptr::null_mut())
        }, 0);
        let descriptor = LocalAllocation(descriptor);
        let mut present = 0;
        let mut defaulted = 0;
        let mut dacl = ptr::null_mut();
        // SAFETY: descriptor is valid and all output pointers are writable.
        assert_ne!(unsafe {
            GetSecurityDescriptorDacl(descriptor.0, &mut present, &mut dacl, &mut defaulted)
        }, 0);
        assert_ne!(present, 0);
        assert!(!dacl.is_null());
        // SAFETY: dacl is borrowed from the live descriptor.
        unsafe { Guard::new().unwrap().dacl_is_safe(dacl, private).unwrap() }
    }

    #[test]
    fn dacl_policy_rejects_shared_modification_and_preserves_root_creation_rules() {
        assert!(!check_dacl("D:(A;;FA;;;WD)", false));
        assert!(!check_dacl("D:(A;;FA;;;WD)", true));
        assert!(check_dacl("D:(A;;FA;;;SY)(A;;FA;;;BA)", true));
        assert!(check_dacl("D:(A;;0x00000004;;;BU)", false));
        assert!(!check_dacl("D:(A;;0x00000004;;;BU)", true));
        assert!(check_dacl("D:(A;OICIIO;FA;;;CO)", false));
        assert!(check_dacl("D:(A;OICIIO;FA;;;CO)", true));
    }

    #[test]
    fn drive_policy_rejects_remote_and_indeterminate_drives() {
        use windows_sys::Win32::System::WindowsProgramming::{
            DRIVE_NO_ROOT_DIR, DRIVE_REMOTE, DRIVE_UNKNOWN,
        };
        for kind in [DRIVE_NO_ROOT_DIR, DRIVE_REMOTE, DRIVE_UNKNOWN] {
            assert!(!local_drive_type(kind));
        }
        for kind in [DRIVE_FIXED, DRIVE_REMOVABLE, DRIVE_RAMDISK] {
            assert!(local_drive_type(kind));
        }
    }
}
