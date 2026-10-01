use axonml::autograd::Variable;
use axonml::nn::{Conv1d, Module};
use axonml::nn::{CrossEntropyLoss, Embedding, GRU, Linear, Parameter, ReLU};
use axonml::optim::{Adam, Optimizer};
use axonml::tensor::Tensor;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
/*
Gaurav Sablok
gsablok@proton.me
 */
pub struct DnaClassifier {
    embedding: Embedding,
    conv1: Conv1d,
    relu: ReLU,
    gru: GRU,
    fc: Linear,
}
impl DnaClassifier {
    fn new(
        vocab_size: usize,
        embed_dim: usize,
        conv_channels: usize,
        gru_hidden: usize,
        num_classes: usize,
    ) -> Self {
        Self {
            embedding: Embedding::new(vocab_size, embed_dim),
            conv1: Conv1d::new(embed_dim, conv_channels, 2),
            relu: ReLU,
            gru: GRU::new(conv_channels, gru_hidden, 1),
            fc: Linear::new(gru_hidden, num_classes),
        }
    }
}
impl Module for DnaClassifier {
    fn forward(&self, x: &Variable) -> Variable {
        // Embedding: [batch, seq_len, embed_dim]
        let batch = self.embedding.forward(&x);
        // Conv1d expects [batch, channels, seq_len], so swap the last two
        // axes to move embed_dim into the channel slot. Using transpose
        // instead of permute: axonml's autograd Variable exposes a
        // differentiable two-axis transpose, not a general N-dim permute.
        let batch_cl = batch.transpose(1, 2); // -> [batch, embed_dim, seq_len]
        let conv_out = self.relu.forward(&self.conv1.forward(&batch_cl)); // -> [batch, conv_channels, seq_len']

        // GRU expects [batch, seq_len, features], so transpose back.
        let batchc = conv_out.transpose(1, 2); // -> [batch, seq_len', conv_channels]
        let out = self.gru.forward(&batchc);
        let last = out.flatten(out.shape()[1] - 1);
        self.fc.forward(&last)
    }
    fn parameters(&self) -> Vec<Parameter> {
        [
            self.embedding.parameters(),
            self.conv1.parameters(),
            self.gru.parameters(),
            self.fc.parameters(),
        ]
        .concat()
    }
}
fn baseidx(seq: &str) -> Vec<f32> {
    seq.chars()
        .map(|c| match c.to_ascii_uppercase() {
            'A' => 0.0,
            'C' => 1.0,
            'G' => 2.0,
            'T' => 3.0,
            _ => 0.0,
        })
        .collect()
}
#[tokio::main]
pub async fn trainaxonml(
    pathname: &str,
    label: &str,
    epochs: &str,
) -> Result<String, Box<dyn Error>> {
    let model = DnaClassifier::new(4, 16, 32, 64, 2);
    let mut optimizer = Adam::new(model.parameters(), 0.001);
    let loss_fn = CrossEntropyLoss::new();
    let (inputdataclass, inputlabels) = inputdata(pathname, label).unwrap();
    let batch_size = inputdataclass.len();
    let seq_len = inputdataclass[0].len();
    let flatclass: Vec<f32> = inputdataclass.iter().flat_map(|s| baseidx(s)).collect();
    let input = Variable::new(
        Tensor::<f32>::from_vec(flatclass, &[batch_size, seq_len]).unwrap(),
        false,
    );
    let targets = Variable::new(
        Tensor::<f32>::from_vec(
            inputlabels
                .iter()
                .map(|x| x.to_string().parse::<f32>().unwrap())
                .collect::<Vec<_>>(),
            &[batch_size],
        )
        .unwrap(),
        false,
    );
    for epoch in 0..epochs.parse::<usize>().unwrap() {
        let output = model.forward(&input);
        let loss = loss_fn.compute(&output, &targets);
        optimizer.zero_grad();
        loss.backward();
        optimizer.step();
        println!("epoch: {epoch}, Loss => {:?}", loss.data().to_vec());
    }
    Ok("The axonml training has finished".to_string())
}
#[tokio::main]
pub async fn inputdata(
    pathname: &str,
    label: &str,
) -> Result<(Vec<String>, Vec<usize>), Box<dyn Error>> {
    let file = File::open(pathname).expect("file not found");
    let fileread = BufReader::new(file);
    let mut stringvec: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if !line.starts_with(">") {
            stringvec.push(line);
        }
    }
    let mut labels: Vec<usize> = Vec::new();
    let labelopen = File::open(label).expect("labels not found");
    let labelread = BufReader::new(labelopen);
    for i in labelread.lines() {
        let line = i.expect("line not found");
        labels.push(line.parse::<usize>().unwrap())
    }
    Ok((stringvec, labels))
}
