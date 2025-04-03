# Burn: データグループ別モデル学習（メタモデル方式）

RustのBurnフレームワークを使用して、データグループに応じて異なるモデルで学習を行うための「メタモデル方式」についての手順とポイントをまとめます。

**前提条件:** 各データグループに対応するモデルは、すべて**同じ入力・出力インターフェース（テンソルの形状）**を持っている必要があります。

## 概要

この方式では、すべてのグループに対応するサブモデルを内部に保持する「メタモデル」を定義します。単一の学習パイプライン（`Learner`）を使用し、データローダーが提供するグループ識別子に基づいて、メタモデル内で適切なサブモデルを呼び出して学習を進めます。

## 手順

### 1. データ準備とデータローダー拡張

*   **データセットアイテム定義:**
    各データアイテムに、入力データ、ターゲットデータに加え、**グループ識別子**を含めます。
    ```rust
    use burn::prelude::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Enum は Copy が便利
    pub enum DataGroup {
        GroupA,
        GroupB,
        // 他のグループ...
    }

    #[derive(Clone, Debug)] // Dataset Item は Clone が必要
    pub struct MyBatch<B: Backend> {
        pub inputs: Tensor<B, /* Input Dim */>,
        pub targets: Tensor<B, /* Target Dim */>,
        // バッチ内の各サンプルに対応するグループIDリスト or 単一ID
        // Tensor<B, 1, Int> なども可能
        pub groups: Vec<DataGroup>,
    }
    ```
*   **データローダー実装:**
    Burnの `Dataset` トレイトを実装し、`get` メソッドや `collate` 関数が上記のグループ識別子を含む `MyBatch` (または個々のアイテム) を返すようにします。

### 2. サブモデルの定義

*   各データグループに対応するモデル（例: `ModelA`, `ModelB`）を、通常通り `burn::module::Module` トレイトを実装して定義します。
*   **重要:** すべてのサブモデルは、**同じ `forward` 関数のシグネチャ**を持ちます。
    ```rust
    #[derive(Module, Debug)]
    pub struct ModelA<B: Backend> { /* ... レイヤー ... */ }
    impl<B: Backend> ModelA<B> {
        pub fn new(device: &B::Device) -> Self { /* ... 初期化 ... */ todo!() }
        // forward のシグネチャを統一
        pub fn forward(&self, input: Tensor<B, /* Input Dim */>) -> Tensor<B, /* Target Dim */> {
            // ... Model A の処理 ...
            todo!()
        }
    }

    #[derive(Module, Debug)]
    pub struct ModelB<B: Backend> { /* ... レイヤー ... */ }
    impl<B: Backend> ModelB<B> {
         pub fn new(device: &B::Device) -> Self { /* ... 初期化 ... */ todo!() }
        // ModelA と同じシグネチャを持つ forward
        pub fn forward(&self, input: Tensor<B, /* Input Dim */>) -> Tensor<B, /* Target Dim */> {
            // ... Model B の処理 ...
            todo!()
        }
    }
    ```

### 3. メタモデルの定義

*   **構造:** すべてのサブモデルをフィールドとして持つ `MetaModel` 構造体を定義し、これも `burn::module::Module` トレイトを実装します。
    ```rust
    #[derive(Module, Debug)]
    pub struct MetaModel<B: Backend> {
        model_a: ModelA<B>,
        model_b: ModelB<B>,
        // 他のサブモデル...
    }

    impl<B: Backend> MetaModel<B> {
        pub fn new(device: &B::Device) -> Self {
            Self {
                model_a: ModelA::new(device),
                model_b: ModelB::new(device),
                // ...
            }
        }

        // 単一サンプル or グループ単位のバッチに対する forward (推論や学習ステップで利用)
        pub fn forward_single_group(&self, input: Tensor<B, /* Input Dim */>, group: DataGroup) -> Tensor<B, /* Target Dim */> {
            match group {
                DataGroup::GroupA => self.model_a.forward(input),
                DataGroup::GroupB => self.model_b.forward(input),
                // ...
            }
        }
    }
    ```

### 4. 学習ステップの実装 (`TrainStep`/`ValidStep`)

*   `MetaModel` に `burn::train::TrainStep` および `burn::train::ValidStep` トレイトを実装します。
*   `step` メソッド内でバッチデータ (`MyBatch`) を受け取ります。
*   **効率的な処理 (推奨):**
    1.  バッチデータをグループごとに分割します (`HashMap<DataGroup, Vec<usize>>` などを使用)。
    2.  各グループの入力データとターゲットデータを `Tensor::index_select` で抽出します。
    3.  対応するサブモデルの `forward` (または `forward_single_group`) を呼び出します。
    4.  得られた出力を、元のバッチ順序に対応するように `Tensor::index_select_assign` などを使って集約します。
    5.  集約した出力とターゲットを用いて**バッチ全体の損失**を計算します。
    ```rust
    use burn::train::{TrainStep, ValidStep, TrainOutput};
    use burn::tensor::backend::AutodiffBackend;
    use burn::tensor::{Tensor, Int}; // Int Backend を使うため
    use std::collections::HashMap;

    impl<B: AutodiffBackend> TrainStep<MyBatch<B>, Tensor<B, 1>> for MetaModel<B> {
        fn step(&self, batch: MyBatch<B>) -> TrainOutput<Tensor<B, 1>> {
            let device = batch.inputs.device();
            let batch_size = batch.inputs.dims()[0];

            // グループごとにインデックスを収集
            let mut group_indices: HashMap<DataGroup, Vec<usize>> = HashMap::new();
            for (i, group) in batch.groups.iter().enumerate() {
                group_indices.entry(*group).or_default().push(i);
            }

            // バッチ全体の出力を格納するテンソルを初期化 (ゼロ埋め)
            let mut all_outputs = Tensor::zeros_like(&batch.targets);
            // バッチ全体のターゲットを格納するテンソルを作成 (index_select_assign用)
            let mut final_targets = Tensor::zeros_like(&batch.targets);


            for (group, indices) in group_indices {
                // usize を i32 (Burn の Int Tensor のデフォルト) に変換
                let indices_vec: Vec<i32> = indices.iter().map(|&i| i as i32).collect();
                let indices_tensor = Tensor::<B, 1, Int>::from_ints(&indices_vec, &device);

                // 対応するグループの入力とターゲットを選択
                let group_inputs = batch.inputs.clone().index_select(0, indices_tensor.clone());
                let group_targets = batch.targets.clone().index_select(0, indices_tensor.clone());

                // 対応するサブモデルで forward
                let group_output = self.forward_single_group(group_inputs, group);

                // 計算結果 (出力とターゲット) を元の位置に戻す
                all_outputs = all_outputs.index_select_assign(0, indices_tensor.clone(), group_output);
                final_targets = final_targets.index_select_assign(0, indices_tensor, group_targets);
            }

            // --- バッチ全体の損失計算 ---
            // 例: MSE 損失
            let loss = burn::tensor::loss::mse_loss(all_outputs, final_targets);

            // 損失と勾配計算のための情報を TrainOutput で返す
            TrainOutput::new(self, loss.backward(), loss)
        }
    }

    // ValidStep も同様に実装 (loss.backward() は不要)
    impl<B: Backend> ValidStep<MyBatch<B>, Tensor<B, 1>> for MetaModel<B> {
         fn step(&self, batch: MyBatch<B>) -> Tensor<B, 1> {
             // TrainStep とほぼ同じロジックで出力を計算し、損失を返す
             // ... (forward と損失計算部分、backwardなし) ...
             // 以下の実装は TrainStep のロジックをコピーし、不要な部分を削除した例
             let device = batch.inputs.device();
             let batch_size = batch.inputs.dims()[0];
             let mut group_indices: HashMap<DataGroup, Vec<usize>> = HashMap::new();
             for (i, group) in batch.groups.iter().enumerate() {
                 group_indices.entry(*group).or_default().push(i);
             }
             let mut all_outputs = Tensor::zeros_like(&batch.targets);
             let mut final_targets = Tensor::zeros_like(&batch.targets);

             for (group, indices) in group_indices {
                 let indices_vec: Vec<i32> = indices.iter().map(|&i| i as i32).collect();
                 let indices_tensor = Tensor::<B, 1, Int>::from_ints(&indices_vec, &device);
                 let group_inputs = batch.inputs.clone().index_select(0, indices_tensor.clone());
                 let group_targets = batch.targets.clone().index_select(0, indices_tensor.clone());
                 let group_output = self.forward_single_group(group_inputs, group);
                 all_outputs = all_outputs.index_select_assign(0, indices_tensor.clone(), group_output);
                 final_targets = final_targets.index_select_assign(0, indices_tensor, group_targets);
             }
             let loss = burn::tensor::loss::mse_loss(all_outputs, final_targets);
             loss // ValidStep では損失テンソルのみを返す
        }
    }
    ```

### 5. 学習の実行

*   `LearnerBuilder` を使って `Learner` を構築します。
*   `build` メソッドに `MetaModel` のインスタンスとオプティマイザを渡します。
*   オプティマイザは `MetaModel` 全体に適用されますが、**Burnが自動微分時に計算に関与したパラメータ（= 使用されたサブモデルのパラメータ）の勾配のみを計算・更新**するため、意図通りに動作します。
    ```rust
    use burn::optim::AdamConfig;
    use burn::train::LearnerBuilder;
    use burn::data::dataloader::DataLoaderBuilder; // 例として追加

    fn run_training<B: AutodiffBackend>(device: &B::Device) {
        // データセットとデータローダー準備 (MyBatch を返すもの) の仮定
        // let dataset_train = MyDataset::new("train"); // 仮
        // let dataset_valid = MyDataset::new("valid"); // 仮
        // let dataloader_train = DataLoaderBuilder::new(dataset_train)
        //     .batch_size(32)
        //     .num_workers(4)
        //     .build();
        // let dataloader_valid = DataLoaderBuilder::new(dataset_valid)
        //     .batch_size(64) // 検証用は大きくても良い場合がある
        //     .num_workers(4)
        //     .build();

        let artifact_dir = "/tmp/burn-metamodel-example"; // 保存先ディレクトリ

        let model = MetaModel::<B>::new(device);
        let optim = AdamConfig::new().init(); // MetaModel 全体に適用

        let learner = LearnerBuilder::new(artifact_dir)
            .with_file_checkpointer(10) // 10エポックごとにチェックポイント保存
            .devices(vec![device.clone()]) // 使用するデバイス
            .num_epochs(20) // 学習エポック数
            .build(model, optim); // 損失関数は Step トレイト内で計算

        println!("Setup complete. Learner configured. Artifact dir: {}", artifact_dir);

        // 学習開始 (データローダーを渡す)
        // let model_trained = learner.fit(dataloader_train, dataloader_valid);
        // println!("Training finished.");
    }
    ```

## メリット

*   単一の学習プロセスで管理できる。
*   Burn の `Learner` などのエコシステム（チェックポイント、メトリクス、ロギング等）を最大限活用できる。
*   コードの共通化がしやすい。
*   勾配計算とパラメータ更新は Burn が自動で正しく処理してくれる（未使用のモデルパラメータの勾配はゼロになる）。

## デメリット

*   `TrainStep`/`ValidStep` の実装が、単純なモデルに比べて少し複雑になる（特に効率的なバッチ処理部分）。
*   すべてのサブモデルがメモリにロードされるため、モデルサイズが大きい場合やグループ数が多い場合にメモリ使用量が増加する可能性がある。

## 補足

*   `TrainStep`/`ValidStep` 内でのバッチ処理は、パフォーマンスに大きく影響します。ループ内で1サンプルずつ処理するのではなく、`index_select` や `index_select_assign` を使ってテンソル操作でグループごとに一括処理する方がGPU利用効率が高く、高速です。
*   グループ識別子は `enum` だけでなく、`usize` や `Tensor<B, 1, Int>` など、扱いやすい形式を選択できます。
*   `AutodiffBackend` が必要になるのは `TrainStep` と `Learner` を使う場合です。`ValidStep` や推論だけなら通常の `Backend` で十分です。
*   コード例中の `todo!()` やコメントアウトされた部分は、実際のデータやモデルに合わせて実装する必要があります。