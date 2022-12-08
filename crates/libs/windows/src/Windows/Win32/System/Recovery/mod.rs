#[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplicationRecoveryFinished<P0>(bsuccess: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn ApplicationRecoveryFinished ( bsuccess : super::super::Foundation:: BOOL ) -> ( ) );
    ApplicationRecoveryFinished(bsuccess.into())
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplicationRecoveryInProgress() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::core::link ! ( "kernel32.dll""system" fn ApplicationRecoveryInProgress ( pbcancelled : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ApplicationRecoveryInProgress(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn GetApplicationRecoveryCallback<P0>(hprocess: P0, precoverycallback: *mut super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, ppvparameter: ::core::option::Option<*mut *mut ::core::ffi::c_void>, pdwpinginterval: ::core::option::Option<*mut u32>, pdwflags: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetApplicationRecoveryCallback ( hprocess : super::super::Foundation:: HANDLE , precoverycallback : *mut super::WindowsProgramming:: APPLICATION_RECOVERY_CALLBACK , ppvparameter : *mut *mut ::core::ffi::c_void , pdwpinginterval : *mut u32 , pdwflags : *mut u32 ) -> :: windows::core::HRESULT );
    GetApplicationRecoveryCallback(hprocess.into(), precoverycallback, ::core::mem::transmute(ppvparameter.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwpinginterval.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwflags.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationRestartSettings<P0>(hprocess: P0, pwzcommandline: ::windows::core::PWSTR, pcchsize: *mut u32, pdwflags: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetApplicationRestartSettings ( hprocess : super::super::Foundation:: HANDLE , pwzcommandline : :: windows::core::PWSTR , pcchsize : *mut u32 , pdwflags : *mut u32 ) -> :: windows::core::HRESULT );
    GetApplicationRestartSettings(hprocess.into(), ::core::mem::transmute(pwzcommandline), pcchsize, ::core::mem::transmute(pdwflags.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn RegisterApplicationRecoveryCallback(precoveycallback: super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, pvparameter: ::core::option::Option<*const ::core::ffi::c_void>, dwpinginterval: u32, dwflags: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "kernel32.dll""system" fn RegisterApplicationRecoveryCallback ( precoveycallback : super::WindowsProgramming:: APPLICATION_RECOVERY_CALLBACK , pvparameter : *const ::core::ffi::c_void , dwpinginterval : u32 , dwflags : u32 ) -> :: windows::core::HRESULT );
    RegisterApplicationRecoveryCallback(precoveycallback, ::core::mem::transmute(pvparameter.unwrap_or(::std::ptr::null())), dwpinginterval, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
#[inline]
pub unsafe fn RegisterApplicationRestart<P0>(pwzcommandline: P0, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn RegisterApplicationRestart ( pwzcommandline : :: windows::core::PCWSTR , dwflags : REGISTER_APPLICATION_RESTART_FLAGS ) -> :: windows::core::HRESULT );
    RegisterApplicationRestart(pwzcommandline.into(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
#[inline]
pub unsafe fn UnregisterApplicationRecoveryCallback() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "kernel32.dll""system" fn UnregisterApplicationRecoveryCallback ( ) -> :: windows::core::HRESULT );
    UnregisterApplicationRecoveryCallback().ok()
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
#[inline]
pub unsafe fn UnregisterApplicationRestart() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "kernel32.dll""system" fn UnregisterApplicationRestart ( ) -> :: windows::core::HRESULT );
    UnregisterApplicationRestart().ok()
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REGISTER_APPLICATION_RESTART_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_CRASH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_HANG: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_PATCH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_REBOOT: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(8u32);
impl ::core::marker::Copy for REGISTER_APPLICATION_RESTART_FLAGS {}
impl ::core::clone::Clone for REGISTER_APPLICATION_RESTART_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGISTER_APPLICATION_RESTART_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REGISTER_APPLICATION_RESTART_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGISTER_APPLICATION_RESTART_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTER_APPLICATION_RESTART_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
