use raug::prelude::*;
use raug_ext::prelude::*;
use raug_iced::prelude::*;

fn main() {
    // initialize logging
    env_logger::init();

    // create a new graph
    let graph = Graph::new();

    // add a frequency knob
    let (freq_knob, freq_knob_params) = graph.add_widget(Knob::new());
    let [freq] = &freq_knob_params[..] else {
        unreachable!()
    };
    let freq = freq * 1000.0;
    let freq = freq[0].smooth(0.01);

    // add a sine oscillator
    let sine = SineOscillator::default().node(&graph, freq, (), ());

    // connect the sine oscillator to the outputs
    graph.dac(&sine[0]);
    graph.dac(&sine[0]);

    let rt = IcedRuntime::new(graph, freq_knob);
    rt.run(AudioBackend::Default, AudioDevice::Default).unwrap();

    // build the graph and run the runtime
    // graph
    //     .build_runtime(freq_knob)
    //     .run(AudioBackend::Default, AudioDevice::default())
    //     .unwrap();
}
