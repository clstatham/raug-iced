use raug::prelude::*;
use raug_iced::prelude::*;

fn main() {
    // initialize logging
    env_logger::init();

    // create a new graph
    let graph = IcedGraphBuilder::new();

    // add some outputs
    let out1 = graph.add_audio_output();
    let out2 = graph.add_audio_output();

    // add a sine oscillator
    let sine = graph.add(SineOscillator::default());

    // add a frequency knob
    let (freq_knob, freq_knob_params) = graph.knob();
    let [freq] = &freq_knob_params[..] else {
        unreachable!()
    };

    // connect the frequency knob to output an audio-type signal, and smooth it
    let freq = freq.smooth(0.01);

    // scale the smooth processor output to a frequency range
    let freq = freq * 1000.0;

    // set the frequency of the sine oscillator
    freq.output(0).connect(&sine.input(0));

    // connect the sine oscillator to the outputs
    sine.output(0).connect(&out1.input(0));
    sine.output(0).connect(&out2.input(0));

    // build the graph and run the runtime
    graph
        .build_runtime(freq_knob)
        .run(AudioBackend::Default, AudioDevice::default())
        .unwrap();
}
