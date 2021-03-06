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
// Implement this structure to handle events related to browser life span. The
// functions of this structure will be called on the UI thread unless otherwise
// indicated.
//
#[repr(C)]
pub struct _cef_life_span_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called on the IO thread before a new popup browser is created. The
  // |browser| and |frame| values represent the source of the popup request. The
  // |target_url| and |target_frame_name| values indicate where the popup
  // browser should navigate and may be NULL if not specified with the request.
  // The |target_disposition| value indicates where the user intended to open
  // the popup (e.g. current tab, new tab, etc). The |user_gesture| value will
  // be true (1) if the popup was opened via explicit user gesture (e.g.
  // clicking a link) or false (0) if the popup opened automatically (e.g. via
  // the DomContentLoaded event). The |popupFeatures| structure contains
  // additional information about the requested popup window. To allow creation
  // of the popup browser optionally modify |windowInfo|, |client|, |settings|
  // and |no_javascript_access| and return false (0). To cancel creation of the
  // popup browser return true (1). The |client| and |settings| values will
  // default to the source browser's values. If the |no_javascript_access| value
  // is set to false (0) the new browser will not be scriptable and may not be
  // hosted in the same renderer process as the source browser.
  pub on_before_popup: Option<extern "C" fn(this: *mut cef_life_span_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      target_url: *const types::cef_string_t,
      target_frame_name: *const types::cef_string_t,
      target_disposition: types::cef_window_open_disposition_t,
      user_gesture: libc::c_int,
      popupFeatures: *const interfaces::cef_popup_features_t,
      windowInfo: *mut interfaces::cef_window_info_t,
      client: *mut interfaces::cef_client_t,
      settings: *mut interfaces::cef_browser_settings_t,
      no_javascript_access: *mut libc::c_int) -> libc::c_int>,

  //
  // Called after a new browser is created.
  //
  pub on_after_created: Option<extern "C" fn(this: *mut cef_life_span_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // Called when a modal window is about to display and the modal loop should
  // begin running. Return false (0) to use the default modal loop
  // implementation or true (1) to use a custom implementation.
  //
  pub run_modal: Option<extern "C" fn(this: *mut cef_life_span_handler_t,
      browser: *mut interfaces::cef_browser_t) -> libc::c_int>,

  //
  // Called when a browser has recieved a request to close. This may result
  // directly from a call to cef_browser_host_t::close_browser() or indirectly
  // if the browser is a top-level OS window created by CEF and the user
  // attempts to close the window. This function will be called after the
  // JavaScript 'onunload' event has been fired. It will not be called for
  // browsers after the associated OS window has been destroyed (for those
  // browsers it is no longer possible to cancel the close).
  //
  // If CEF created an OS window for the browser returning false (0) will send
  // an OS close notification to the browser window's top-level owner (e.g.
  // WM_CLOSE on Windows, performClose: on OS-X and "delete_event" on Linux). If
  // no OS window exists (window rendering disabled) returning false (0) will
  // cause the browser object to be destroyed immediately. Return true (1) if
  // the browser is parented to another window and that other window needs to
  // receive close notification via some non-standard technique.
  //
  // If an application provides its own top-level window it should handle OS
  // close notifications by calling cef_browser_host_t::CloseBrowser(false (0))
  // instead of immediately closing (see the example below). This gives CEF an
  // opportunity to process the 'onbeforeunload' event and optionally cancel the
  // close before do_close() is called.
  //
  // The cef_life_span_handler_t::on_before_close() function will be called
  // immediately before the browser object is destroyed. The application should
  // only exit after on_before_close() has been called for all existing
  // browsers.
  //
  // If the browser represents a modal window and a custom modal loop
  // implementation was provided in cef_life_span_handler_t::run_modal() this
  // callback should be used to restore the opener window to a usable state.
  //
  // By way of example consider what should happen during window close when the
  // browser is parented to an application-provided top-level OS window. 1.
  // User clicks the window close button which sends an OS close
  //     notification (e.g. WM_CLOSE on Windows, performClose: on OS-X and
  //     "delete_event" on Linux).
  // 2.  Application's top-level window receives the close notification and:
  //     A. Calls CefBrowserHost::CloseBrowser(false).
  //     B. Cancels the window close.
  // 3.  JavaScript 'onbeforeunload' handler executes and shows the close
  //     confirmation dialog (which can be overridden via
  //     CefJSDialogHandler::OnBeforeUnloadDialog()).
  // 4.  User approves the close. 5.  JavaScript 'onunload' handler executes. 6.
  // Application's do_close() handler is called. Application will:
  //     A. Set a flag to indicate that the next close attempt will be allowed.
  //     B. Return false.
  // 7.  CEF sends an OS close notification. 8.  Application's top-level window
  // receives the OS close notification and
  //     allows the window to close based on the flag from #6B.
  // 9.  Browser OS window is destroyed. 10. Application's
  // cef_life_span_handler_t::on_before_close() handler is called and
  //     the browser object is destroyed.
  // 11. Application exits by calling cef_quit_message_loop() if no other
  // browsers
  //     exist.
  //
  pub do_close: Option<extern "C" fn(this: *mut cef_life_span_handler_t,
      browser: *mut interfaces::cef_browser_t) -> libc::c_int>,

  //
  // Called just before a browser is destroyed. Release all references to the
  // browser object and do not attempt to execute any functions on the browser
  // object after this callback returns. If this is a modal window and a custom
  // modal loop implementation was provided in run_modal() this callback should
  // be used to exit the custom modal loop. See do_close() documentation for
  // additional usage information.
  //
  pub on_before_close: Option<extern "C" fn(this: *mut cef_life_span_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_life_span_handler_t = _cef_life_span_handler_t;


//
// Implement this structure to handle events related to browser life span. The
// functions of this structure will be called on the UI thread unless otherwise
// indicated.
//
pub struct CefLifeSpanHandler {
  c_object: *mut cef_life_span_handler_t,
}

impl Clone for CefLifeSpanHandler {
  fn clone(&self) -> CefLifeSpanHandler{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefLifeSpanHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefLifeSpanHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefLifeSpanHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_life_span_handler_t) -> CefLifeSpanHandler {
    CefLifeSpanHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_life_span_handler_t) -> CefLifeSpanHandler {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefLifeSpanHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_life_span_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_life_span_handler_t {
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
  // Called on the IO thread before a new popup browser is created. The
  // |browser| and |frame| values represent the source of the popup request. The
  // |target_url| and |target_frame_name| values indicate where the popup
  // browser should navigate and may be NULL if not specified with the request.
  // The |target_disposition| value indicates where the user intended to open
  // the popup (e.g. current tab, new tab, etc). The |user_gesture| value will
  // be true (1) if the popup was opened via explicit user gesture (e.g.
  // clicking a link) or false (0) if the popup opened automatically (e.g. via
  // the DomContentLoaded event). The |popupFeatures| structure contains
  // additional information about the requested popup window. To allow creation
  // of the popup browser optionally modify |windowInfo|, |client|, |settings|
  // and |no_javascript_access| and return false (0). To cancel creation of the
  // popup browser return true (1). The |client| and |settings| values will
  // default to the source browser's values. If the |no_javascript_access| value
  // is set to false (0) the new browser will not be scriptable and may not be
  // hosted in the same renderer process as the source browser.
  pub fn on_before_popup(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, target_url: &[u16],
      target_frame_name: &[u16],
      target_disposition: types::cef_window_open_disposition_t,
      user_gesture: libc::c_int, popupFeatures: &interfaces::CefPopupFeatures,
      windowInfo: &mut interfaces::CefWindowInfo,
      client: interfaces::CefClient,
      settings: &mut interfaces::CefBrowserSettings,
      no_javascript_access: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_popup.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(target_url),
          CefWrap::to_c(target_frame_name),
          CefWrap::to_c(target_disposition),
          CefWrap::to_c(user_gesture),
          CefWrap::to_c(popupFeatures),
          CefWrap::to_c(windowInfo),
          CefWrap::to_c(client),
          CefWrap::to_c(settings),
          CefWrap::to_c(no_javascript_access)))
    }
  }

  //
  // Called after a new browser is created.
  //
  pub fn on_after_created(&self, browser: interfaces::CefBrowser) -> () {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_after_created.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }

  //
  // Called when a modal window is about to display and the modal loop should
  // begin running. Return false (0) to use the default modal loop
  // implementation or true (1) to use a custom implementation.
  //
  pub fn run_modal(&self, browser: interfaces::CefBrowser) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).run_modal.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }

  //
  // Called when a browser has recieved a request to close. This may result
  // directly from a call to cef_browser_host_t::close_browser() or indirectly
  // if the browser is a top-level OS window created by CEF and the user
  // attempts to close the window. This function will be called after the
  // JavaScript 'onunload' event has been fired. It will not be called for
  // browsers after the associated OS window has been destroyed (for those
  // browsers it is no longer possible to cancel the close).
  //
  // If CEF created an OS window for the browser returning false (0) will send
  // an OS close notification to the browser window's top-level owner (e.g.
  // WM_CLOSE on Windows, performClose: on OS-X and "delete_event" on Linux). If
  // no OS window exists (window rendering disabled) returning false (0) will
  // cause the browser object to be destroyed immediately. Return true (1) if
  // the browser is parented to another window and that other window needs to
  // receive close notification via some non-standard technique.
  //
  // If an application provides its own top-level window it should handle OS
  // close notifications by calling cef_browser_host_t::CloseBrowser(false (0))
  // instead of immediately closing (see the example below). This gives CEF an
  // opportunity to process the 'onbeforeunload' event and optionally cancel the
  // close before do_close() is called.
  //
  // The cef_life_span_handler_t::on_before_close() function will be called
  // immediately before the browser object is destroyed. The application should
  // only exit after on_before_close() has been called for all existing
  // browsers.
  //
  // If the browser represents a modal window and a custom modal loop
  // implementation was provided in cef_life_span_handler_t::run_modal() this
  // callback should be used to restore the opener window to a usable state.
  //
  // By way of example consider what should happen during window close when the
  // browser is parented to an application-provided top-level OS window. 1.
  // User clicks the window close button which sends an OS close
  //     notification (e.g. WM_CLOSE on Windows, performClose: on OS-X and
  //     "delete_event" on Linux).
  // 2.  Application's top-level window receives the close notification and:
  //     A. Calls CefBrowserHost::CloseBrowser(false).
  //     B. Cancels the window close.
  // 3.  JavaScript 'onbeforeunload' handler executes and shows the close
  //     confirmation dialog (which can be overridden via
  //     CefJSDialogHandler::OnBeforeUnloadDialog()).
  // 4.  User approves the close. 5.  JavaScript 'onunload' handler executes. 6.
  // Application's do_close() handler is called. Application will:
  //     A. Set a flag to indicate that the next close attempt will be allowed.
  //     B. Return false.
  // 7.  CEF sends an OS close notification. 8.  Application's top-level window
  // receives the OS close notification and
  //     allows the window to close based on the flag from #6B.
  // 9.  Browser OS window is destroyed. 10. Application's
  // cef_life_span_handler_t::on_before_close() handler is called and
  //     the browser object is destroyed.
  // 11. Application exits by calling cef_quit_message_loop() if no other
  // browsers
  //     exist.
  //
  pub fn do_close(&self, browser: interfaces::CefBrowser) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).do_close.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }

  //
  // Called just before a browser is destroyed. Release all references to the
  // browser object and do not attempt to execute any functions on the browser
  // object after this callback returns. If this is a modal window and a custom
  // modal loop implementation was provided in run_modal() this callback should
  // be used to exit the custom modal loop. See do_close() documentation for
  // additional usage information.
  //
  pub fn on_before_close(&self, browser: interfaces::CefBrowser) -> () {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_close.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }
} 

impl CefWrap<*mut cef_life_span_handler_t> for CefLifeSpanHandler {
  fn to_c(rust_object: CefLifeSpanHandler) -> *mut cef_life_span_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_life_span_handler_t) -> CefLifeSpanHandler {
    CefLifeSpanHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_life_span_handler_t> for Option<CefLifeSpanHandler> {
  fn to_c(rust_object: Option<CefLifeSpanHandler>) -> *mut cef_life_span_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_life_span_handler_t) -> Option<CefLifeSpanHandler> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefLifeSpanHandler::from_c_object_addref(c_object))
    }
  }
}

