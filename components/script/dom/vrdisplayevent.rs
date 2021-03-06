/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::EventBinding::EventBinding::EventMethods;
use crate::dom::bindings::codegen::Bindings::VRDisplayEventBinding;
use crate::dom::bindings::codegen::Bindings::VRDisplayEventBinding::VRDisplayEventMethods;
use crate::dom::bindings::codegen::Bindings::VRDisplayEventBinding::VRDisplayEventReason;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::Event;
use crate::dom::globalscope::GlobalScope;
use crate::dom::vrdisplay::VRDisplay;
use crate::dom::window::Window;
use dom_struct::dom_struct;
use servo_atoms::Atom;
use webvr_traits::{WebVRDisplayEvent, WebVRDisplayEventReason};

#[dom_struct]
pub struct VRDisplayEvent {
    event: Event,
    display: Dom<VRDisplay>,
    reason: Option<VRDisplayEventReason>,
}

impl VRDisplayEvent {
    fn new_inherited(display: &VRDisplay, reason: Option<VRDisplayEventReason>) -> VRDisplayEvent {
        VRDisplayEvent {
            event: Event::new_inherited(),
            display: Dom::from_ref(display),
            reason: reason.clone(),
        }
    }

    pub fn new(
        global: &GlobalScope,
        type_: Atom,
        bubbles: bool,
        cancelable: bool,
        display: &VRDisplay,
        reason: Option<VRDisplayEventReason>,
    ) -> DomRoot<VRDisplayEvent> {
        let ev = reflect_dom_object(
            Box::new(VRDisplayEvent::new_inherited(&display, reason)),
            global,
            VRDisplayEventBinding::Wrap,
        );
        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, bubbles, cancelable);
        }
        ev
    }

    pub fn new_from_webvr(
        global: &GlobalScope,
        display: &VRDisplay,
        event: &WebVRDisplayEvent,
    ) -> DomRoot<VRDisplayEvent> {
        let (name, reason) = match *event {
            WebVRDisplayEvent::Connect(_) => ("vrdisplayconnect", None),
            WebVRDisplayEvent::Disconnect(_) => ("vrdisplaydisconnect", None),
            WebVRDisplayEvent::Activate(_, reason) => ("vrdisplayactivate", Some(reason)),
            WebVRDisplayEvent::Deactivate(_, reason) => ("vrdisplaydeactivate", Some(reason)),
            WebVRDisplayEvent::Blur(_) => ("vrdisplayblur", None),
            WebVRDisplayEvent::Focus(_) => ("vrdisplayfocus", None),
            WebVRDisplayEvent::PresentChange(_, _) => ("vrdisplaypresentchange", None),
            WebVRDisplayEvent::Change(_) |
            WebVRDisplayEvent::Pause(_) |
            WebVRDisplayEvent::Resume(_) |
            WebVRDisplayEvent::Exit(_) => panic!("{:?} event not available in WebVR", event),
        };

        // map to JS enum values
        let reason = reason.map(|r| match r {
            WebVRDisplayEventReason::Navigation => VRDisplayEventReason::Navigation,
            WebVRDisplayEventReason::Mounted => VRDisplayEventReason::Mounted,
            WebVRDisplayEventReason::Unmounted => VRDisplayEventReason::Unmounted,
        });

        VRDisplayEvent::new(
            &global,
            Atom::from(DOMString::from(name)),
            false,
            false,
            &display,
            reason,
        )
    }

    #[allow(non_snake_case)]
    pub fn Constructor(
        window: &Window,
        type_: DOMString,
        init: &VRDisplayEventBinding::VRDisplayEventInit,
    ) -> Fallible<DomRoot<VRDisplayEvent>> {
        Ok(VRDisplayEvent::new(
            &window.global(),
            Atom::from(type_),
            init.parent.bubbles,
            init.parent.cancelable,
            &init.display,
            init.reason,
        ))
    }
}

impl VRDisplayEventMethods for VRDisplayEvent {
    // https://w3c.github.io/webvr/#dom-vrdisplayevent-display
    fn Display(&self) -> DomRoot<VRDisplay> {
        DomRoot::from_ref(&*self.display)
    }

    // https://w3c.github.io/webvr/#enumdef-vrdisplayeventreason
    fn GetReason(&self) -> Option<VRDisplayEventReason> {
        self.reason
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
