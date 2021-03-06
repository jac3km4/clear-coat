/* Copyright 2015 Jordan Miner
 *
 * Licensed under the MIT license <LICENSE or
 * http://opensource.org/licenses/MIT>. This file may not be copied,
 * modified, or distributed except according to those terms.
 */

#[macro_use]
extern crate clear_coat;

use std::rc::Rc;
use clear_coat::*;

// Tests that callbacks are dropped when the control they are added to is destroyed.

#[test]
fn test_callback_drop_container() {
    let x = Rc::new(0);
    let x2 = x.clone();
    let button = Button::new();
    button.action_event().add(move || println!("{}", *x2));
    let x = Rc::try_unwrap(x).unwrap_err();
    let hbox = hbox!(&button);
    let x = Rc::try_unwrap(x).unwrap_err();
    drop(button);
    let x = Rc::try_unwrap(x).unwrap_err();
    drop(hbox);
    let _ = Rc::try_unwrap(x).unwrap();
}
