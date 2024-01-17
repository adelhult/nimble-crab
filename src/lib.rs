/// Auto generated C bindings
mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::{
    cmp::min,
    ffi::CString,
    mem,
    os::raw::{c_char, c_int, c_void},
    ptr,
    time::Duration,
};

use c::{
    g_clog, transmuteVmInit, TransmuteInput, TransmuteParticipantInput, TransmuteState,
    TransmuteVm, TransmuteVmSetup, TransmuteVmVersion,
};

/// TODO: Since both the Nimble client, Nimble server, and renderer wants to access the state
/// we should probably have a wrapper newtype or at least a type alias:
/// type SharedVm = Pin<Rc<RefCell<Vm>>>. Feels like a pretty akward solution though, but I'm
/// not sure how to do it better. I think (?) the Rust port of GGPO might uses a queue of requests
/// instead of owning the state? I Should take a look at that.
pub struct Vm<T> {
    inner_vm: TransmuteVm,
    simulation: T,
}

impl From<&[u8]> for TransmuteState {
    fn from(value: &[u8]) -> Self {
        TransmuteState {
            state: value.as_ptr() as *mut c_void,
            octetSize: value.len(),
        }
    }
}

impl From<TransmuteState> for &[u8] {
    fn from(value: TransmuteState) -> Self {
        let slice = ptr::slice_from_raw_parts(value.state, value.octetSize);
        unsafe { &*(slice as *const [u8]) }
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl From<Version> for TransmuteVmVersion {
    fn from(
        Version {
            major,
            minor,
            patch,
        }: Version,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

pub trait Simulation {
    type Input; // TODO: not sure if it is a good idea to have a associated type for the input
                // but in that case, you would also have a trait `AsInput` for the input type that
                // requires a to_string method and a way of converting to a &[u8]
    fn tick(&mut self, input: Self::Input);
    fn get_state(&self) -> &[u8];
    fn set_state(&mut self, state: &[u8]);
    fn display_state(state: &[u8]) -> String;
}

pub trait AsInput
where
    Self: Default,
{
    // FIXME
}

unsafe extern "C" fn tick<T: Simulation>(vm: *mut c_void, _input: *const TransmuteInput)
where
    T::Input: AsInput,
{
    let vm = &mut *(vm as *mut Vm<T>);
    // TODO: use the input
    vm.simulation.tick(T::Input::default());
}

unsafe extern "C" fn get_state<T: Simulation>(vm: *const c_void) -> TransmuteState {
    let vm = &*(vm as *const Vm<T>);
    vm.simulation.get_state().into()
}

unsafe extern "C" fn set_state<T: Simulation>(vm: *mut c_void, state: *const TransmuteState) {
    let vm = &mut *(vm as *mut Vm<T>);
    vm.simulation.set_state((*state).into());
}

unsafe extern "C" fn state_to_string<T: Simulation>(
    _: *mut c_void,
    state: *const TransmuteState,
    target: *mut c_char,
    max_size: usize,
) -> c_int {
    let str = T::display_state((*state).into());
    let byte_str = str.as_bytes();

    let target = std::slice::from_raw_parts_mut(target as *mut u8, max_size);
    let bytes_written = min(target.len(), byte_str.len());

    target[..bytes_written].copy_from_slice(&byte_str[..bytes_written]);

    bytes_written as c_int
}

unsafe extern "C" fn input_to_string<T: Simulation>(
    _vm: *mut c_void,
    _input: *const TransmuteParticipantInput,
    _target: *mut c_char,
    _max_size: usize,
) -> c_int {
    // see comment about the Input associated type
    todo!("Implement input to string")
}

impl<T: Simulation> Vm<T>
where
    T::Input: AsInput,
{
    pub fn new(simulation: T, version: Version, tick_duration: Duration) -> Self {
        let transmute_setup = TransmuteVmSetup {
            tickFn: Some(tick::<T>),
            getStateFn: Some(get_state::<T>),
            setStateFn: Some(set_state::<T>),
            stateToString: Some(state_to_string::<T>),
            inputToString: Some(input_to_string::<T>),
            tickDurationMs: tick_duration.as_millis() as usize,
            version: version.into(),
        };

        unsafe {
            let mut vm = Self {
                inner_vm: mem::zeroed(),
                simulation,
            };

            let vm_ptr = &mut vm as *mut _ as *mut c_void;

            // FIXME: logging should be hanled in a different way
            let c_str = CString::new("vm").unwrap();
            let logger = c::Clog {
                constantPrefix: c_str.as_ptr() as *const c_char,
                config: &mut g_clog,
            };

            transmuteVmInit(&mut vm.inner_vm, vm_ptr, transmute_setup, logger);
            vm
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    // todo: store game state and logic in here
    struct Game;

    impl AsInput for () {}

    // implement the nimble (transmute VM) trait
    impl Simulation for Game {
        type Input = ();

        fn get_state(&self) -> &[u8] {
            todo!()
        }

        fn set_state(&mut self, state: &[u8]) {
            todo!()
        }

        fn tick(&mut self, input: Self::Input) {
            todo!()
        }

        fn display_state(state: &[u8]) -> String {
            todo!()
        }
    }

    #[test]
    fn api() {
        let game = Game;
        let vm = Vm::new(game, Version::new(1, 1, 1), Duration::from_millis(16));
        /*
        Results in a linker error:
        .../nimble-crab/nimble/build/deps/piot/transmute-c/src/lib/libtransmute.a(transmute.c.o): relocation R_X86_64_32 against `.rodata.str1.1' can not be used when making a PIE object; recompile with -fPIE
          /usr/bin/ld: failed to set dynamic section sizes: bad value
          collect2: error: ld returned 1 exit status
        */
    }
}
