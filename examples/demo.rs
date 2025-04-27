use raug::prelude::*;
use raug_ext::prelude::*;
use raug_iced::prelude::*;

fn main() {
    // initialize logging
    env_logger::init();

    // create a new graph
    let graph = Graph::new(0, 2);

    // add a frequency knob
    let (freq_knob, freq_knob_params) = graph.add_widget(Knob::new(60.0, 1.0, 120.0));
    let [freq] = &freq_knob_params[..] else {
        unreachable!()
    };

    let freq = PitchToFreq::default().node(&graph, freq);
    let freq = freq[0].smooth(0.01);

    // add a sine oscillator
    let sine = BlSawOscillator::default().node(&graph, freq);
    let sine = &sine[0] * 0.1;

    // connect the sine oscillator to the outputs
    graph.dac((&sine[0], &sine[0]));

    // run it!
    let rt = IcedRuntime::new(graph, freq_knob);
    rt.run(AudioBackend::Default, AudioDevice::Default).unwrap();
}
