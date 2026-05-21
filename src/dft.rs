use std::f32::consts::TAU;

pub fn dft(samples: Vec<f32>) -> Vec<f32>{
    let number_of_samples = samples.len();

    let spectrum_lenth = (number_of_samples / 2) + 1;
    let mut spectrum: Vec<f32>  = vec![0.; spectrum_lenth];

    println!("starting dft");
    println!("number of samples = {}", number_of_samples);
    println!("spectrum length = {}", spectrum_lenth);

    for test_frequency_index in 0..spectrum_lenth {
        let mut sample_sum = [0., 0.];

        for current_sample_index in 0..samples.len() {
            let theta = current_sample_index as f32 / number_of_samples as f32
                * TAU
                * test_frequency_index as f32;
            sample_sum[0] += theta.cos() * samples[current_sample_index];
            sample_sum[1] += theta.sin() * samples[current_sample_index];
        }

        spectrum[test_frequency_index] = ((sample_sum[0]/number_of_samples as f32).exp2() + (sample_sum[1]/number_of_samples as f32).exp2()).sqrt();
        if spectrum[test_frequency_index] > 1.5 {
                println!("{}",spectrum[test_frequency_index]);
        }
    }

    spectrum
}
