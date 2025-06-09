pub fn similarity<const N: usize>(
    distribution1: &[f32; N],
    distribution2: &[f32; N],
) -> f32 {
    let mut sum = 0.;
    for i in 0..N {
        let d1 = distribution1[i] + f32::EPSILON;
        let d2 = distribution2[i] + f32::EPSILON;
        sum += distribution1[i] * (d1 / d2).log2();
    }
    sum
}

pub fn symmetric_similarity<const N: usize>(
    distribution1: &[f32; N],
    distribution2: &[f32; N],
) -> f32 {
    let mut mix: [f32; N] = [0.; N];
    for i in 0..N {
        mix[i] = (distribution1[i] + distribution2[i]) * 0.5;
    }
    (similarity(distribution1, &mix) + similarity(distribution2, &mix)) * 0.5
}
