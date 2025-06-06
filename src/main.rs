use efficient_pure_functional::{pipe_like_chain, use_copy_or_move, use_ref_mut};

fn main() {
    // use side effect
    let mut vec = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|x| *x as f64)
        .collect::<Vec<f64>>();
    let rms = use_ref_mut::rms(&mut vec);
    println!("rms = {rms}");

    // no side effect
    let vec = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|x| *x as f64)
        .collect::<Vec<f64>>();

    // copy
    let rms = use_copy_or_move::rms(vec.clone());
    println!("rms = {rms}");

    // move
    let rms = use_copy_or_move::rms(vec);
    println!("rms = {rms}");

    // pipe like chain
    let vec = pipe_like_chain::VecFp64(
        vec![1, 2, 3, 4, 5]
            .iter()
            .map(|x| *x as f64)
            .collect::<Vec<f64>>(),
    );

    // pipeのようなチェーンン
    let rms = vec.clone().squared().mean().sqrt();
    println!("rms = {rms}");

    let rms = vec.rms();
    println!("rms = {rms}");
}
