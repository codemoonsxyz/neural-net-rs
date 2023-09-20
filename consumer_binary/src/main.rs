use neural_network::network::Network;
use neural_network::activations::SIGMOID;
use neural_network::matrix::Matrix;
use std::env;
fn main() {
	env::set_var("RUST_BACKTRACE", "1");
    let inputs = vec![
		vec![0.0, 0.0],
		vec![0.0, 1.0],
		vec![1.0, 0.0],
		vec![1.0, 1.0],
	];
	let targets = vec![vec![0.0], vec![1.0], vec![0.0], vec![1.0]];

    let mut network = Network::new(vec![2,3,1],SIGMOID,0.5);
   

    network.train(inputs, targets, 100000);

	println!("{:?}", network.feed_forward(Matrix::from(vec![0.0, 0.0])));
	println!("{:?}", network.feed_forward(Matrix::from(vec![0.0, 1.0])));
	println!("{:?}", network.feed_forward(Matrix::from(vec![1.0, 0.0])));
	println!("{:?}", network.feed_forward(Matrix::from(vec![1.0, 1.0])));

    

}
