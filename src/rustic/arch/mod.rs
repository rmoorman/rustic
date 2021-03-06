/*
 * Copyright (c) 2014 Matthew Iselin
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

#[cfg(arch_i386)]
mod i386;

#[cfg(arch_armv6)]
mod armv6;

#[cfg(arch_armv7)]
mod armv7;

// State module pulls in architecture-specific state type as 'State' type.
mod state;

pub trait Architecture {
    fn initialise(&mut self) -> bool;

    fn register_trap(&mut self, uint, extern "Rust" fn(uint));

    fn get_interrupts(&self) -> bool;
    fn set_interrupts(&mut self, bool);

    fn wait_for_event(&self);
}

pub trait Threads {
    fn spawn_thread(&mut self, proc());

    fn thread_terminate(&mut self) -> !;

    // Trigger a reschedule.
    fn reschedule(&mut self);
}

pub trait TrapHandler {
    fn trap(&mut self, num: uint);
}

pub struct ArchitectureState {
    initialised: bool,
    state: state::State,
}

impl ArchitectureState {
    fn new() -> ArchitectureState {
        ArchitectureState{initialised: false, state: state::State::new()}
    }
}

pub fn create() -> Box<ArchitectureState> {
    box ArchitectureState::new()
}
