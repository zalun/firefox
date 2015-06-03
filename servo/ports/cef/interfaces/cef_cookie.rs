// Copyright (c) 2015 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::mem;
use std::ptr;

//
// Structure used for managing cookies. The functions of this structure may be
// called on any thread unless otherwise indicated.
//
#[repr(C)]
pub struct _cef_cookie_manager_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Set the schemes supported by this manager. By default only "http" and
  // "https" schemes are supported. If |callback| is non-NULL it will be
  // executed asnychronously on the IO thread after the change has been applied.
  // Must be called before any cookies are accessed.
  //
  pub set_supported_schemes: Option<extern "C" fn(
      this: *mut cef_cookie_manager_t, schemes: &types::cef_string_list_t,
      callback: *mut interfaces::cef_completion_callback_t) -> ()>,

  //
  // Visit all cookies on the IO thread. The returned cookies are ordered by
  // longest path, then by earliest creation date. Returns false (0) if cookies
  // cannot be accessed.
  //
  pub visit_all_cookies: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      visitor: *mut interfaces::cef_cookie_visitor_t) -> libc::c_int>,

  //
  // Visit a subset of cookies on the IO thread. The results are filtered by the
  // given url scheme, host, domain and path. If |includeHttpOnly| is true (1)
  // HTTP-only cookies will also be included in the results. The returned
  // cookies are ordered by longest path, then by earliest creation date.
  // Returns false (0) if cookies cannot be accessed.
  //
  pub visit_url_cookies: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      url: *const types::cef_string_t, includeHttpOnly: libc::c_int,
      visitor: *mut interfaces::cef_cookie_visitor_t) -> libc::c_int>,

  //
  // Sets a cookie given a valid URL and explicit user-provided cookie
  // attributes. This function expects each attribute to be well-formed. It will
  // check for disallowed characters (e.g. the ';' character is disallowed
  // within the cookie value attribute) and fail without setting the cookie if
  // such characters are found. If |callback| is non-NULL it will be executed
  // asnychronously on the IO thread after the cookie has been set. Returns
  // false (0) if an invalid URL is specified or if cookies cannot be accessed.
  //
  pub set_cookie: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      url: *const types::cef_string_t, cookie: *const interfaces::cef_cookie_t,
      callback: *mut interfaces::cef_set_cookie_callback_t) -> libc::c_int>,

  //
  // Delete all cookies that match the specified parameters. If both |url| and
  // |cookie_name| values are specified all host and domain cookies matching
  // both will be deleted. If only |url| is specified all host cookies (but not
  // domain cookies) irrespective of path will be deleted. If |url| is NULL all
  // cookies for all hosts and domains will be deleted. If |callback| is non-
  // NULL it will be executed asnychronously on the IO thread after the cookies
  // have been deleted. Returns false (0) if a non-NULL invalid URL is specified
  // or if cookies cannot be accessed. Cookies can alternately be deleted using
  // the Visit*Cookies() functions.
  //
  pub delete_cookies: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      url: *const types::cef_string_t, cookie_name: *const types::cef_string_t,
      callback: *mut interfaces::cef_delete_cookies_callback_t) -> libc::c_int>,

  //
  // Sets the directory path that will be used for storing cookie data. If
  // |path| is NULL data will be stored in memory only. Otherwise, data will be
  // stored at the specified |path|. To persist session cookies (cookies without
  // an expiry date or validity interval) set |persist_session_cookies| to true
  // (1). Session cookies are generally intended to be transient and most Web
  // browsers do not persist them. If |callback| is non-NULL it will be executed
  // asnychronously on the IO thread after the manager's storage has been
  // initialized. Returns false (0) if cookies cannot be accessed.
  //
  pub set_storage_path: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      path: *const types::cef_string_t, persist_session_cookies: libc::c_int,
      callback: *mut interfaces::cef_completion_callback_t) -> libc::c_int>,

  //
  // Flush the backing store (if any) to disk. If |callback| is non-NULL it will
  // be executed asnychronously on the IO thread after the flush is complete.
  // Returns false (0) if cookies cannot be accessed.
  //
  pub flush_store: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      callback: *mut interfaces::cef_completion_callback_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_cookie_manager_t = _cef_cookie_manager_t;


//
// Structure used for managing cookies. The functions of this structure may be
// called on any thread unless otherwise indicated.
//
pub struct CefCookieManager {
  c_object: *mut cef_cookie_manager_t,
}

impl Clone for CefCookieManager {
  fn clone(&self) -> CefCookieManager{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefCookieManager {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefCookieManager {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefCookieManager {
  pub unsafe fn from_c_object(c_object: *mut cef_cookie_manager_t) -> CefCookieManager {
    CefCookieManager {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_cookie_manager_t) -> CefCookieManager {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefCookieManager {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_cookie_manager_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_cookie_manager_t {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null() || self.c_object as usize == mem::POST_DROP_USIZE
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null() && self.c_object as usize != mem::POST_DROP_USIZE
  }

  //
  // Set the schemes supported by this manager. By default only "http" and
  // "https" schemes are supported. If |callback| is non-NULL it will be
  // executed asnychronously on the IO thread after the change has been applied.
  // Must be called before any cookies are accessed.
  //
  pub fn set_supported_schemes(&self, schemes: &Vec<String>,
      callback: interfaces::CefCompletionCallback) -> () {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_supported_schemes.unwrap())(
          self.c_object,
          CefWrap::to_c(schemes),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Visit all cookies on the IO thread. The returned cookies are ordered by
  // longest path, then by earliest creation date. Returns false (0) if cookies
  // cannot be accessed.
  //
  pub fn visit_all_cookies(&self,
      visitor: interfaces::CefCookieVisitor) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit_all_cookies.unwrap())(
          self.c_object,
          CefWrap::to_c(visitor)))
    }
  }

  //
  // Visit a subset of cookies on the IO thread. The results are filtered by the
  // given url scheme, host, domain and path. If |includeHttpOnly| is true (1)
  // HTTP-only cookies will also be included in the results. The returned
  // cookies are ordered by longest path, then by earliest creation date.
  // Returns false (0) if cookies cannot be accessed.
  //
  pub fn visit_url_cookies(&self, url: &[u16], includeHttpOnly: libc::c_int,
      visitor: interfaces::CefCookieVisitor) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit_url_cookies.unwrap())(
          self.c_object,
          CefWrap::to_c(url),
          CefWrap::to_c(includeHttpOnly),
          CefWrap::to_c(visitor)))
    }
  }

  //
  // Sets a cookie given a valid URL and explicit user-provided cookie
  // attributes. This function expects each attribute to be well-formed. It will
  // check for disallowed characters (e.g. the ';' character is disallowed
  // within the cookie value attribute) and fail without setting the cookie if
  // such characters are found. If |callback| is non-NULL it will be executed
  // asnychronously on the IO thread after the cookie has been set. Returns
  // false (0) if an invalid URL is specified or if cookies cannot be accessed.
  //
  pub fn set_cookie(&self, url: &[u16], cookie: &interfaces::CefCookie,
      callback: interfaces::CefSetCookieCallback) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_cookie.unwrap())(
          self.c_object,
          CefWrap::to_c(url),
          CefWrap::to_c(cookie),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Delete all cookies that match the specified parameters. If both |url| and
  // |cookie_name| values are specified all host and domain cookies matching
  // both will be deleted. If only |url| is specified all host cookies (but not
  // domain cookies) irrespective of path will be deleted. If |url| is NULL all
  // cookies for all hosts and domains will be deleted. If |callback| is non-
  // NULL it will be executed asnychronously on the IO thread after the cookies
  // have been deleted. Returns false (0) if a non-NULL invalid URL is specified
  // or if cookies cannot be accessed. Cookies can alternately be deleted using
  // the Visit*Cookies() functions.
  //
  pub fn delete_cookies(&self, url: &[u16], cookie_name: &[u16],
      callback: interfaces::CefDeleteCookiesCallback) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).delete_cookies.unwrap())(
          self.c_object,
          CefWrap::to_c(url),
          CefWrap::to_c(cookie_name),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Sets the directory path that will be used for storing cookie data. If
  // |path| is NULL data will be stored in memory only. Otherwise, data will be
  // stored at the specified |path|. To persist session cookies (cookies without
  // an expiry date or validity interval) set |persist_session_cookies| to true
  // (1). Session cookies are generally intended to be transient and most Web
  // browsers do not persist them. If |callback| is non-NULL it will be executed
  // asnychronously on the IO thread after the manager's storage has been
  // initialized. Returns false (0) if cookies cannot be accessed.
  //
  pub fn set_storage_path(&self, path: &[u16],
      persist_session_cookies: libc::c_int,
      callback: interfaces::CefCompletionCallback) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_storage_path.unwrap())(
          self.c_object,
          CefWrap::to_c(path),
          CefWrap::to_c(persist_session_cookies),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Flush the backing store (if any) to disk. If |callback| is non-NULL it will
  // be executed asnychronously on the IO thread after the flush is complete.
  // Returns false (0) if cookies cannot be accessed.
  //
  pub fn flush_store(&self,
      callback: interfaces::CefCompletionCallback) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).flush_store.unwrap())(
          self.c_object,
          CefWrap::to_c(callback)))
    }
  }

  //
  // Returns the global cookie manager. By default data will be stored at
  // CefSettings.cache_path if specified or in memory otherwise. If |callback|
  // is non-NULL it will be executed asnychronously on the IO thread after the
  // manager's storage has been initialized. Using this function is equivalent
  // to calling cef_request_tContext::cef_request_context_get_global_context()->
  // get_default_cookie_manager().
  //
  pub fn get_global_manager(
      callback: interfaces::CefCompletionCallback) -> interfaces::CefCookieManager {
    unsafe {
      CefWrap::to_rust(
        ::cookie::cef_cookie_manager_get_global_manager(
          CefWrap::to_c(callback)))
    }
  }

  //
  // Creates a new cookie manager. If |path| is NULL data will be stored in
  // memory only. Otherwise, data will be stored at the specified |path|. To
  // persist session cookies (cookies without an expiry date or validity
  // interval) set |persist_session_cookies| to true (1). Session cookies are
  // generally intended to be transient and most Web browsers do not persist
  // them. If |callback| is non-NULL it will be executed asnychronously on the
  // IO thread after the manager's storage has been initialized.
  //
  pub fn create_manager(path: &[u16], persist_session_cookies: libc::c_int,
      callback: interfaces::CefCompletionCallback) -> interfaces::CefCookieManager {
    unsafe {
      CefWrap::to_rust(
        ::cookie::cef_cookie_manager_create_manager(
          CefWrap::to_c(path),
          CefWrap::to_c(persist_session_cookies),
          CefWrap::to_c(callback)))
    }
  }
} 

impl CefWrap<*mut cef_cookie_manager_t> for CefCookieManager {
  fn to_c(rust_object: CefCookieManager) -> *mut cef_cookie_manager_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_manager_t) -> CefCookieManager {
    CefCookieManager::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_cookie_manager_t> for Option<CefCookieManager> {
  fn to_c(rust_object: Option<CefCookieManager>) -> *mut cef_cookie_manager_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_manager_t) -> Option<CefCookieManager> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefCookieManager::from_c_object_addref(c_object))
    }
  }
}


//
// Structure to implement for visiting cookie values. The functions of this
// structure will always be called on the IO thread.
//
#[repr(C)]
pub struct _cef_cookie_visitor_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be called once for each cookie. |count| is the 0-based
  // index for the current cookie. |total| is the total number of cookies. Set
  // |deleteCookie| to true (1) to delete the cookie currently being visited.
  // Return false (0) to stop visiting cookies. This function may never be
  // called if no cookies are found.
  //
  pub visit: Option<extern "C" fn(this: *mut cef_cookie_visitor_t,
      cookie: *const interfaces::cef_cookie_t, count: libc::c_int,
      total: libc::c_int, deleteCookie: *mut libc::c_int) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_cookie_visitor_t = _cef_cookie_visitor_t;


//
// Structure to implement for visiting cookie values. The functions of this
// structure will always be called on the IO thread.
//
pub struct CefCookieVisitor {
  c_object: *mut cef_cookie_visitor_t,
}

impl Clone for CefCookieVisitor {
  fn clone(&self) -> CefCookieVisitor{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefCookieVisitor {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefCookieVisitor {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefCookieVisitor {
  pub unsafe fn from_c_object(c_object: *mut cef_cookie_visitor_t) -> CefCookieVisitor {
    CefCookieVisitor {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_cookie_visitor_t) -> CefCookieVisitor {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefCookieVisitor {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_cookie_visitor_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_cookie_visitor_t {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null() || self.c_object as usize == mem::POST_DROP_USIZE
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null() && self.c_object as usize != mem::POST_DROP_USIZE
  }

  //
  // Method that will be called once for each cookie. |count| is the 0-based
  // index for the current cookie. |total| is the total number of cookies. Set
  // |deleteCookie| to true (1) to delete the cookie currently being visited.
  // Return false (0) to stop visiting cookies. This function may never be
  // called if no cookies are found.
  //
  pub fn visit(&self, cookie: &interfaces::CefCookie, count: libc::c_int,
      total: libc::c_int, deleteCookie: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit.unwrap())(
          self.c_object,
          CefWrap::to_c(cookie),
          CefWrap::to_c(count),
          CefWrap::to_c(total),
          CefWrap::to_c(deleteCookie)))
    }
  }
} 

impl CefWrap<*mut cef_cookie_visitor_t> for CefCookieVisitor {
  fn to_c(rust_object: CefCookieVisitor) -> *mut cef_cookie_visitor_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_visitor_t) -> CefCookieVisitor {
    CefCookieVisitor::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_cookie_visitor_t> for Option<CefCookieVisitor> {
  fn to_c(rust_object: Option<CefCookieVisitor>) -> *mut cef_cookie_visitor_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_visitor_t) -> Option<CefCookieVisitor> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefCookieVisitor::from_c_object_addref(c_object))
    }
  }
}


//
// Structure to implement to be notified of asynchronous completion via
// cef_cookie_manager_t::set_cookie().
//
#[repr(C)]
pub struct _cef_set_cookie_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be called upon completion. |success| will be true (1) if
  // the cookie was set successfully.
  //
  pub on_complete: Option<extern "C" fn(this: *mut cef_set_cookie_callback_t,
      success: libc::c_int) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_set_cookie_callback_t = _cef_set_cookie_callback_t;


//
// Structure to implement to be notified of asynchronous completion via
// cef_cookie_manager_t::set_cookie().
//
pub struct CefSetCookieCallback {
  c_object: *mut cef_set_cookie_callback_t,
}

impl Clone for CefSetCookieCallback {
  fn clone(&self) -> CefSetCookieCallback{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefSetCookieCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefSetCookieCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefSetCookieCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_set_cookie_callback_t) -> CefSetCookieCallback {
    CefSetCookieCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_set_cookie_callback_t) -> CefSetCookieCallback {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefSetCookieCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_set_cookie_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_set_cookie_callback_t {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null() || self.c_object as usize == mem::POST_DROP_USIZE
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null() && self.c_object as usize != mem::POST_DROP_USIZE
  }

  //
  // Method that will be called upon completion. |success| will be true (1) if
  // the cookie was set successfully.
  //
  pub fn on_complete(&self, success: libc::c_int) -> () {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_complete.unwrap())(
          self.c_object,
          CefWrap::to_c(success)))
    }
  }
} 

impl CefWrap<*mut cef_set_cookie_callback_t> for CefSetCookieCallback {
  fn to_c(rust_object: CefSetCookieCallback) -> *mut cef_set_cookie_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_set_cookie_callback_t) -> CefSetCookieCallback {
    CefSetCookieCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_set_cookie_callback_t> for Option<CefSetCookieCallback> {
  fn to_c(rust_object: Option<CefSetCookieCallback>) -> *mut cef_set_cookie_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_set_cookie_callback_t) -> Option<CefSetCookieCallback> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefSetCookieCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Structure to implement to be notified of asynchronous completion via
// cef_cookie_manager_t::delete_cookies().
//
#[repr(C)]
pub struct _cef_delete_cookies_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be called upon completion. |num_deleted| will be the
  // number of cookies that were deleted or -1 if unknown.
  //
  pub on_complete: Option<extern "C" fn(
      this: *mut cef_delete_cookies_callback_t, num_deleted: libc::c_int) -> (
      )>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_delete_cookies_callback_t = _cef_delete_cookies_callback_t;


//
// Structure to implement to be notified of asynchronous completion via
// cef_cookie_manager_t::delete_cookies().
//
pub struct CefDeleteCookiesCallback {
  c_object: *mut cef_delete_cookies_callback_t,
}

impl Clone for CefDeleteCookiesCallback {
  fn clone(&self) -> CefDeleteCookiesCallback{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDeleteCookiesCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDeleteCookiesCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDeleteCookiesCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_delete_cookies_callback_t) -> CefDeleteCookiesCallback {
    CefDeleteCookiesCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_delete_cookies_callback_t) -> CefDeleteCookiesCallback {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDeleteCookiesCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_delete_cookies_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_delete_cookies_callback_t {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null() || self.c_object as usize == mem::POST_DROP_USIZE
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null() && self.c_object as usize != mem::POST_DROP_USIZE
  }

  //
  // Method that will be called upon completion. |num_deleted| will be the
  // number of cookies that were deleted or -1 if unknown.
  //
  pub fn on_complete(&self, num_deleted: libc::c_int) -> () {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_complete.unwrap())(
          self.c_object,
          CefWrap::to_c(num_deleted)))
    }
  }
} 

impl CefWrap<*mut cef_delete_cookies_callback_t> for CefDeleteCookiesCallback {
  fn to_c(rust_object: CefDeleteCookiesCallback) -> *mut cef_delete_cookies_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_delete_cookies_callback_t) -> CefDeleteCookiesCallback {
    CefDeleteCookiesCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_delete_cookies_callback_t> for Option<CefDeleteCookiesCallback> {
  fn to_c(rust_object: Option<CefDeleteCookiesCallback>) -> *mut cef_delete_cookies_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_delete_cookies_callback_t) -> Option<CefDeleteCookiesCallback> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefDeleteCookiesCallback::from_c_object_addref(c_object))
    }
  }
}

