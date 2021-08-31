//use std::f32;
use petgraph::stable_graph::NodeIndex;
use processgraph::process::set_sr;
use processgraph::*;
use std::os::raw::{c_double, c_float, c_int, c_uint};
use std::slice;
use std::str::from_utf8;

pub struct UGenState {
    output: Vec<f64>,
    flow: Vec<NodeIndex>,
    graph: UGenGraph,
    n_outputs: usize,
}

#[no_mangle]
pub extern "C" fn new_state(samplerate: c_double) -> *mut UGenState {
    set_sr(samplerate as f64);
    Box::into_raw(Box::new(UGenState {
        output: Vec::new(),
        flow: Vec::new(),
        graph: UGenGraph::new(),
        n_outputs: 1,
    }))
}

#[no_mangle]
pub extern "C" fn state_free(state: *mut UGenState) {
    if state.is_null() {
        return;
    }
    // here it goes out of scope and thus gets dropped by rust
    unsafe { Box::from_raw(state) };
}

#[no_mangle]
pub extern "C" fn set_graph(state: *mut UGenState) {
    unsafe {
        (*state).n_outputs = 1;
        (*state).output = vec![0.0; (*state).n_outputs];
        (*state).flow = vec![];

        (*state).graph = UGenGraph::new();

        let input1 = soundinput(1);

        let mut ugen1 = sinosc(10.0);
        ugen1.set_output(0, 1.0);
        let idx_input = (*state).graph.add(input1);
        let idx_ugen = (*state).graph.add(ugen1);
        (*state)
            .graph
            .connect(idx_input, idx_ugen, Connection::new(0, 1.0));
        (*state)
            .graph
            .update_connections_and_flow(&mut (*state).flow);
    }
}

#[no_mangle]
pub extern "C" fn process(
    state: &mut UGenState,
    sc_in: *mut *mut c_float,
    sc_out: *mut *mut c_float,
    sc_nsamples: c_int,
) {
    let n_out = (*state).n_outputs;
    let n_in = (*state).graph.number_of_inputs();

    unsafe {
        let out_buffer: &mut [*mut f32] = slice::from_raw_parts_mut(sc_out, n_out as usize);
        let in_buffer: &mut [*mut f32] = slice::from_raw_parts_mut(sc_in, n_in);
        let mut out_channels: Vec<&mut [f32]> = Vec::new();
        for c in 0..n_out {
            out_channels.push(slice::from_raw_parts_mut(
                out_buffer[c],
                sc_nsamples as usize,
            ));
        }
        let mut in_channels: Vec<&mut [f32]> = Vec::new();
        if n_in > 0 {
            for c in 0..n_in {
                in_channels.push(slice::from_raw_parts_mut(
                    in_buffer[c],
                    sc_nsamples as usize,
                ));
            }
        }

        for i in 0..sc_nsamples {
            let inputs: Vec<f64> = in_channels
                .iter()
                .map(|input| input[i as usize] as f64)
                .collect();
            (*state)
                .graph
                .process(&(*state).flow, inputs.as_slice(), &mut (*state).output);
            for channel in 0..n_out {
                out_channels[channel][i as usize] = (*state).output[channel] as f32;
            }
        }
    }
}
