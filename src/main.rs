mod audio_buffer;
use audio_buffer::*;
fn main() {
    let mut inbuff = [0f64; 32];
    let mut i = 0.0f64;
    for item in &mut inbuff {
        *item = i;
        i += 1.0;
    }

    const CHANNEL_COUNT: usize = 2;

    let mut outbuff = inbuff;
    let audio_in = AudioBuffer::new(&inbuff, CHANNEL_COUNT);
    let mut audio_out = AudioBufferMut::new(&mut outbuff, CHANNEL_COUNT);

    for (input, output) in audio_in.iter().zip(audio_out.iter_mut()) {
        for (i, o) in input.iter().zip(output.iter_mut()) {
            *o = *i;
        }
    }
}
