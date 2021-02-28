//! Virtual Events
//! This module provides a wrapping of platform-specific events with a list of events easier to work with.
//!
//! 3rd party renderers are responsible for forming this virtual events from events
//!
//! The goal here is to provide a consistent event interface across all renderer types
use generational_arena::Index;

use crate::innerlude::CbIdx;

#[derive(Debug)]
pub struct EventTrigger {
    pub component_id: Index,
    pub listener_id: usize,
    pub event: VirtualEvent,
}

impl EventTrigger {
    pub fn new(event: VirtualEvent, cb: CbIdx) -> Self {
        let CbIdx {
            gi_id,
            gi_gen,
            listener_idx,
        } = cb;

        let component_id = Index::from_raw_parts(gi_id, gi_gen);
        Self {
            component_id,
            listener_id: listener_idx,
            event,
        }
    }
}

#[derive(Debug)]
pub enum VirtualEvent {
    // Real events
    ClipboardEvent(ClipboardEvent),
    CompositionEvent(CompositionEvent),
    KeyboardEvent(KeyboardEvent),
    FocusEvent(FocusEvent),
    FormEvent(FormEvent),
    GenericEvent(GenericEvent),
    MouseEvent(MouseEvent),
    PointerEvent(PointerEvent),
    SelectionEvent(SelectionEvent),
    TouchEvent(TouchEvent),
    UIEvent(UIEvent),
    WheelEvent(WheelEvent),
    MediaEvent(MediaEvent),
    ImageEvent(ImageEvent),
    AnimationEvent(AnimationEvent),
    TransitionEvent(TransitionEvent),

    OtherEvent,
}

// these should reference the underlying event

#[derive(Debug)]
pub struct ClipboardEvent {}
#[derive(Debug)]
pub struct CompositionEvent {}
#[derive(Debug)]
pub struct KeyboardEvent {}
#[derive(Debug)]
pub struct FocusEvent {}
#[derive(Debug)]
pub struct FormEvent {}
#[derive(Debug)]
pub struct GenericEvent {}
#[derive(Debug)]
pub struct MouseEvent {}
#[derive(Debug)]
pub struct PointerEvent {}
#[derive(Debug)]
pub struct SelectionEvent {}
#[derive(Debug)]
pub struct TouchEvent {}
#[derive(Debug)]
pub struct UIEvent {}
#[derive(Debug)]
pub struct WheelEvent {}
#[derive(Debug)]
pub struct MediaEvent {}
#[derive(Debug)]
pub struct ImageEvent {}
#[derive(Debug)]
pub struct AnimationEvent {}
#[derive(Debug)]
pub struct TransitionEvent {}