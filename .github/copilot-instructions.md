## コーディング AI 向けの Copilot 指示

### コードの設計プロセス

1. **関数の説明をまず考えさせる**

   - 関数が実行する処理を人間が理解できる言葉で説明する。
   - 例: 「まず A をして、B かどうかを検証し、C を行う」

2. **説明に一致する関数を作成する**

   - その説明が直接コードの構造として現れるように関数を定義する。
   - 抽象度の統一を意識し、適切に分割する。

3. **関数の中身を実装する**
   - それぞれの関数が具体的な処理を担当するようにする。
   - 低レイヤーの関数を作成し、上位の関数が簡潔に読めるようにする。

### 基本原則

- **関数の役割を明確にする**

  - 一つの関数が一つの明確な目的を持つ。
  - 関数の説明がそのままコードの構造になるようにする。

- **関数の抽象度を統一する**
  - 高レベルな関数と低レベルな関数を混在させない。
  - 条件分岐やデータ処理は適切なレイヤーに分離する。

### コーディングスタイルガイドライン

#### 関数の作り方

1. **関数の流れを明確にする**

   - 例（Rust）:

     ```rust
     fn handle_left_key() {
         if is_at_root() {
             // 何もしない
         } else {
             move_to_parent();
         }
     }

     fn is_at_root() -> bool {
         // ルートディレクトリかを判定
     }

     fn move_to_parent() {
         // 親ディレクトリへ移動
     }
     ```

   - `handle_left_key()` を見るだけで、処理の流れが把握できる。
   - `move_to_parent()` の詳細はその関数内で完結させる。

2. **逐次処理の表現を統一する**
   - 例（TypeScript）:
     ```typescript
     async function process(users: User[]) {
     	const activeUsers = filterActiveUsers(users);
     	const transformedUsers = transformUsers(activeUsers);
     	const aggregatedData = aggregateData(transformedUsers);
     	const formattedData = formatData(aggregatedData);
     	return await sendData(formattedData);
     }
     ```
   - 各処理がステップごとに分かれており、直感的に理解しやすい。

### 命名規則

- **処理を具体的に表す名前を付ける**
  - `process_left_key()` のような漠然とした名前ではなく、
    - `handle_left_key()` (左キーの処理)
    - `move_to_parent()` (親ディレクトリへ移動)
    - `is_at_root()` (ルートかどうか判定)
  - など、処理の内容を一目で分かる名前にする。

### アーキテクチャの考慮点

1. **合成メソッド（Composed Methods）**

   - 上位の関数は要約を行い、詳細な処理は下位の関数で実装する。
   - 例:
     ```typescript
     function executeTask() {
     	validateInput();
     	processData();
     	generateReport();
     	notifyUser();
     }
     ```
   - 関数名だけで流れが理解できるようにする。

2. **レイヤードアーキテクチャを意識する**
   - ドメインロジックは適切なレイヤーに分離する。
   - 例：UseCase 層が Infrastructure 層の処理を直接持たないようにする。

### 追加の考慮点

- **SQL クエリ**: 複雑なクエリを記述する場合は、適切なコメントを付与する。
- **宣言的 UI コンポーネント**: React のコンポーネントの責務を明確に分離する。
- **再帰関数**: 基底ケースと再帰ケースを明示し、関数の要件をわかりやすくする。

### 目指すコードの形

- **関数の説明 → 構造化された関数 → 詳細な実装**
- **関数名だけで処理がわかる**
- **抽象度を統一し、可読性を向上させる**

これらの原則に従うことで、可読性が高く、拡張性や保守性に優れたコードを生成できます。

## task.md

- task.md にはタスクを列挙する

### task の開始

- task.md の中から優先順位の高いタスクを選択する
- 基本的には一つ選択し、修正を行う
- ただし、関連の強いタスクがあり一緒に解決したほうが良い場合は、同時に行う

### task の終了後

- 行った task の冒頭に`[x]`を追加する
- 行ったタスクを commit する際の commit message (英語)を考える
- 更に追加の修正点などがあれば、task.md に追記する
