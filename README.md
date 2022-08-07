セキュリティ・キャンプ 2022全国大会のLT「SNSをグラフで見てみる」で扱ったプログラムです。

This is a supplementary repository for experiment a social network on Twitter. [Slides](https://o0i.es/a/sc22).

I am using a dataset on [Kaggle](https://www.kaggle.com/datasets/mathurinache/twitter-edge-nodes?resource=download) by Mathurin Aché. This dataset is quite simple and under public domain, so I have included it in this repository for ease of use. You will need more 'formal' dataset when you conduct some serious research.

## Usage

Make sure Rust toolchain is installed on your environment (see [rustup.rs](https://rustup.rs/)). 

1. Clone this repository
2. Download the dataset from [Kaggle](https://www.kaggle.com/datasets/mathurinache/twitter-edge-nodes?resource=download) and put its `edges.csv` under `dataset/`
3. `cargo run`

It calculates some parameters (e.g. graph diameter) for the network represented in the dataset, then outputs the graph in Graphviz format at `/tmp/sc22.dot`. `$ dot -T pdf -O /tmp/sc22.dot` to visualize it.

Alternatively, you can create your own `edges.csv` with the following format: 

```
1,2 // User 1 follows User 2
1,3
2,4
3,1
4,2
...
```

Note: This program applies Warshall-Floyd algorithm (which is $O(N^3)$ where N is the number of nodes) on the entire graph to calculate its diameter, so it could take a long time and might be even `SIGKILL`ed. Adapt `main.rs` to meet with your needs and computer performances. Let me know if you have any ideas for optimization!