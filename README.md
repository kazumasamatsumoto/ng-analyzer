# ng-analyzer

Rust で構築された高性能な Angular プロジェクト分析ツールです。爆速のパフォーマンスと包括的な分析を提供します。

## 機能

- 🚀 **高性能**: Rust で構築された最高速度と効率性
- 🔍 **包括的分析**: コンポーネント、サービス、依存関係、パフォーマンスパターンの分析
- 🎯 **複数のアナライザー**: コンポーネント、依存関係、状態管理、パフォーマンス分析
- 📊 **豊富なレポート**: JSON、HTML、テーブル形式での出力
- ⚡ **並列処理**: Rust の並行性を活用した大規模プロジェクトの高速分析
- 🛠️ **設定可能なルール**: カスタマイズ可能な分析ルールと重要度レベル
- 📈 **詳細なメトリクス**: プロジェクト統計と複雑度測定

## インストール

### 前提条件

- Rust 1.70 以上
- Cargo（Rust のパッケージマネージャー）

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/your-org/ng-analyzer.git
cd ng-analyzer

# プロジェクトをビルド
cargo build --release

# バイナリが生成される場所
./target/release/ng-analyzer --help
```

### ローカルインストール（推奨）

```bash
# プロジェクトディレクトリ内でローカルインストール
cargo install --path .

# どこからでもコマンドを実行可能
ng-analyzer --help
```

### 開発モードでの実行

```bash
# 開発中はcargo runを使用
cargo run -- --help
cargo run -- component ./src
```

## 使用方法

### 基本的なコマンド構造

```bash
ng-analyzer [OPTIONS] <COMMAND> [ARGS]
```

### 利用可能なオプション

- `--verbose`: 詳細な出力を表示
- `--quiet`: 最小限の出力のみ表示
- `--help`: ヘルプメッセージを表示

## コマンドリファレンス

### 1. コンポーネント分析

Angular コンポーネントの詳細な分析を実行します。

```bash
# 基本的なコンポーネント分析
ng-analyzer component ./src

# 複雑度の閾値を設定
ng-analyzer component ./src --max-complexity 5

# 分析の深さを制限
ng-analyzer component ./src --depth 3

# HTML形式で出力
ng-analyzer component ./src --output html

# エラーのみを表示
ng-analyzer component ./src --errors-only
```

**出力例:**

```
🔍 Starting Angular project analysis...
📁 Analyzing path: ./src
📊 Found 12 components, 5 services, 3 modules

📈 Analysis Summary:
   Total issues found: 8
   Issues shown: 8
   ⚠️  Warnings: 8

   💡 Recommendations: 3
   🎯 Suggestions: 2
```

### 2. 依存関係分析

プロジェクトの依存関係を分析します。

```bash
# 基本的な依存関係分析
ng-analyzer deps ./src

# JSON形式で出力
ng-analyzer deps ./src --format json

# テーブル形式で出力
ng-analyzer deps ./src --format table
```

### 3. 状態管理分析

状態管理パターンとサブスクリプション管理を分析します。

```bash
# 状態管理分析
ng-analyzer state ./src

# JSON形式で出力
ng-analyzer state ./src --format json
```

### 4. パフォーマンス分析

パフォーマンスに影響する要因を分析します。

```bash
# パフォーマンス分析
ng-analyzer performance ./src

# JSON形式で出力
ng-analyzer performance ./src --format json
```

### 5. 包括的監査

全てのアナライザーを実行します。

```bash
# 基本的な監査
ng-analyzer audit ./src

# 全機能を有効にした監査
ng-analyzer audit ./src --full

# 複数形式での出力
ng-analyzer audit ./src --formats json,html,table --output-dir ./reports

# 特定のアナライザーのみ実行
ng-analyzer audit ./src --analyzers component,deps

# 重要度レベルを設定
ng-analyzer audit ./src --severity warning
```

### 6. 設定初期化

プロジェクト設定ファイルを作成します。

```bash
# 推奨設定で初期化
ng-analyzer init --profile recommended

# 厳格な設定で初期化
ng-analyzer init --profile strict

# 設定ファイルの出力先を指定
ng-analyzer init --output ./ng-analyzer.json
```

### 7. アナライザー一覧

利用可能なアナライザーを表示します。

```bash
# アナライザー一覧を表示
ng-analyzer list

# 詳細情報を表示
ng-analyzer list --details

# 特定のカテゴリのみ表示
ng-analyzer list --category component
```

### 8. 検索機能

プロジェクト内のコードを高度な検索機能で検索します。

#### 基本的な検索

```bash
# 基本的な検索
ng-analyzer search ./src --keyword "Component"

# 特定のファイルタイプのみ検索
ng-analyzer search ./src --keyword "Injectable" --file-type ts

# 大文字小文字を区別する検索
ng-analyzer search ./src --keyword "ngOnInit" --case-sensitive

# 行番号を表示
ng-analyzer search ./src --keyword "subscribe" --line-numbers

# コンテキストを含めて表示
ng-analyzer search ./src --keyword "unsubscribe" --context 3
```

#### 高度な検索機能

**HTML クラス名検索** - HTML の class 属性内でクラス名を検索

```bash
# 特定のCSSクラスの使用箇所を検索
ng-analyzer search ./src --keyword "btn-primary" --html-class

# Bootstrapクラスの使用状況を調査
ng-analyzer search ./src --keyword "col-md-" --html-class --file-type html

# 複数のクラスが設定されている要素を検索
ng-analyzer search ./src --keyword "active" --html-class --context 1
```

**HTML テキスト検索** - HTML タグ内のテキストコンテンツを検索

```bash
# ボタンのテキストを検索
ng-analyzer search ./src --keyword "Click here" --html-text

# エラーメッセージの検索
ng-analyzer search ./src --keyword "Error" --html-text --context 2

# 特定の表示テキストの使用箇所を確認
ng-analyzer search ./src --keyword "ログイン" --html-text --file-type html
```

**関数名検索** - TypeScript/JavaScript の関数定義を検索

```bash
# 関数定義を検索
ng-analyzer search ./src --keyword "getUserData" --function-name

# ライフサイクルフックの実装を検索
ng-analyzer search ./src --keyword "ngOnInit" --function-name --file-type ts

# 非同期関数を検索
ng-analyzer search ./src --keyword "loadData" --function-name --context 3

# アロー関数を検索
ng-analyzer search ./src --keyword "handleClick" --function-name
```

**正規表現検索** - 複雑なパターンマッチング

```bash
# Angularディレクティブを検索
ng-analyzer search ./src --keyword "ng[A-Z][a-zA-Z]+" --regex

# メールアドレスパターンを検索
ng-analyzer search ./src --keyword "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" --regex

# HTTPメソッドの使用を検索
ng-analyzer search ./src --keyword "(get|post|put|delete)\(" --regex --file-type ts

# コンポーネントセレクターを検索
ng-analyzer search ./src --keyword "selector:\s*['\"][^'\"]+['\"]" --regex
```

**構造的検索** - 複雑なコードパターンを検索

```bash
# Componentデコレーターの定義を検索
ng-analyzer search ./src --structural "@Component.*selector.*template" --file-type ts

# サービスの注入パターンを検索
ng-analyzer search ./src --structural "constructor.*[a-zA-Z]+Service" --file-type ts

# インターフェース実装を検索
ng-analyzer search ./src --structural "class.*implements.*OnInit" --file-type ts

# 特定のImportパターンを検索
ng-analyzer search ./src --structural "import.*from.*@angular" --file-type ts
```

#### 検索出力フォーマット

**シンプル形式（デフォルト）**

```bash
ng-analyzer search ./src --keyword "ngOnInit" --function-name

# 出力例:
# 📄 src/app/user/user.component.ts
#    2 matches found
#    45:
#    → export class UserComponent implements OnInit {
#         [ngOnInit]() { (function_name)
#
#         console.log('Component initialized');
#    }
```

**テーブル形式**

```bash
ng-analyzer search ./src --keyword "btn-primary" --html-class --output table

# 出力例:
# File                                     Line   Type            Content
# ---------------------------------------------------------------------------------
# src/app/login/login.component.html       15     html_class      <button class="btn btn-primary">Login</button>
# src/app/header/header.component.html     8      html_class      <a class="btn btn-primary btn-sm">Sign Up</a>
```

**JSON 形式**

```bash
ng-analyzer search ./src --keyword "ngOnInit" --function-name --output json

# 出力例:
# [
#   {
#     "file_path": "src/app/user/user.component.ts",
#     "total_matches": 1,
#     "matches": [
#       {
#         "line_number": 45,
#         "line_content": "  ngOnInit() {",
#         "match_start": 2,
#         "match_end": 10,
#         "context_before": [],
#         "context_after": [],
#         "match_type": "function_name"
#       }
#     ]
#   }
# ]
```

#### 実用的な検索例

**1. Angular アプリケーションの監査**

```bash
# 未使用のコンポーネントを探す
ng-analyzer search ./src --keyword "Component" --function-name --output json > components.json

# 古いAngular APIの使用を検索
ng-analyzer search ./src --keyword "ActivatedRoute" --regex --file-type ts

# サブスクリプションの管理状況を確認
ng-analyzer search ./src --keyword "subscribe" --regex --context 3
```

**2. CSS クラスの使用状況調査**

```bash
# 特定のデザインシステムのクラス使用状況
ng-analyzer search ./src --keyword "mat-" --html-class --file-type html

# カスタムCSSクラスの使用箇所
ng-analyzer search ./src --keyword "custom-btn" --html-class --output table

# 未使用の可能性があるクラスを検索
ng-analyzer search ./src --keyword "deprecated" --html-class
```

**3. セキュリティ監査**

```bash
# innerHTML使用箇所の確認
ng-analyzer search ./src --keyword "innerHTML" --regex --file-type ts

# 外部URLの使用を検索
ng-analyzer search ./src --keyword "http[s]?://" --regex --context 2

# 機密情報が含まれる可能性のあるコメントを検索
ng-analyzer search ./src --keyword "TODO.*password|TODO.*secret" --regex
```

**4. リファクタリング支援**

```bash
# 特定のメソッド名の使用箇所を検索
ng-analyzer search ./src --keyword "oldMethodName" --function-name

# 非推奨APIの使用を検索
ng-analyzer search ./src --keyword "@deprecated" --regex --context 1

# 特定のライブラリのimportを検索
ng-analyzer search ./src --structural "import.*from.*old-library" --file-type ts
```

**5. コードベースの理解**

```bash
# すべてのライフサイクルフックの実装を確認
ng-analyzer search ./src --keyword "ng(OnInit|OnDestroy|OnChanges)" --regex --function-name

# エラーハンドリングの実装を検索
ng-analyzer search ./src --keyword "catch|error" --regex --context 2

# 環境設定の使用箇所を検索
ng-analyzer search ./src --keyword "environment\." --regex --file-type ts
```

#### 検索のベストプラクティス

**パフォーマンス最適化**

- `--file-type` を指定して検索範囲を限定
- 大規模プロジェクトでは `--file-pattern` を活用
- 複雑な正規表現は事前にテスト

**効果的な検索**

- 検索キーワードは具体的に指定
- `--context` オプションで周辺コードも確認
- 複数の検索タイプを組み合わせて使用

**結果の活用**

- JSON 出力でスクリプト処理
- テーブル出力で一覧表示
- シンプル出力で詳細確認

## 出力フォーマット

### JSON 出力

```bash
ng-analyzer component ./src --output json
```

JSON 形式での出力例：

```json
{
  "project": {
    "root_path": "./src",
    "components": [...],
    "services": [...],
    "modules": [...]
  },
  "issues": [
    {
      "rule_name": "component-complexity",
      "severity": "Warning",
      "message": "Component complexity (12) exceeds threshold (10)",
      "file_path": "./src/app/complex-component.ts",
      "line_number": 15
    }
  ],
  "recommendations": [...]
}
```

### HTML 出力

```bash
ng-analyzer audit ./src --formats html --output-dir ./reports
```

HTML 形式では、ブラウザで見やすい形式で分析結果が表示されます。

### テーブル出力

```bash
ng-analyzer deps ./src --format table
```

## 実際の使用例

### 1. 新しい Angular プロジェクトの分析

```bash
# プロジェクトの基本的な健全性チェック
ng-analyzer audit ./src --full --formats json,html --output-dir ./reports

# 結果をブラウザで確認
# reports/analysis-report.html を開く
```

### 2. 継続的統合（CI）での使用

```bash
# エラーのみを表示し、問題があればプロセスを終了
ng-analyzer audit ./src --severity error --format json > analysis.json

# 結果をパース
if grep -q '"severity": "Error"' analysis.json; then
  echo "分析でエラーが発見されました"
  exit 1
fi
```

### 3. 大規模プロジェクトのパフォーマンス分析

```bash
# 詳細なパフォーマンス分析
ng-analyzer performance ./src --format html --output-dir ./perf-reports

# 特定のメトリクスのみ確認
ng-analyzer component ./src --max-complexity 5 --errors-only
```

## トラブルシューティング

### よくある問題

1. **ビルドエラー**

   ```bash
   # 依存関係を更新
   cargo update

   # クリーンビルド
   cargo clean
   cargo build --release
   ```

2. **メモリ不足**

   ```bash
   # スタックサイズを増やす
   export RUST_MIN_STACK=8388608
   ng-analyzer audit ./src
   ```

3. **パースエラー**
   ```bash
   # 詳細なエラー情報を表示
   ng-analyzer component ./src --verbose
   ```

### パフォーマンスの最適化

- 大規模プロジェクトでは`--depth`オプションで分析の深さを制限
- 必要なアナライザーのみを実行（`--analyzers`オプション）
- 並列処理を活用するため、十分なメモリを確保

## 設定

### 設定ファイル

設定ファイルを作成するには：

```bash
ng-analyzer init --profile recommended
```

設定例：

```json
{
  "profiles": {
    "recommended": {
      "name": "Recommended",
      "description": "ほとんどのプロジェクトに適したバランスの取れたルール",
      "rules": {
        "component-complexity": {
          "enabled": true,
          "severity": "warning",
          "options": {
            "max_complexity": 10
          }
        },
        "too-many-inputs": {
          "enabled": true,
          "severity": "warning",
          "options": {
            "max_inputs": 8
          }
        }
      }
    }
  },
  "ignore": ["**/*.spec.ts", "**/node_modules/**", "**/dist/**"],
  "output": {
    "formats": ["json", "html"],
    "path": "./reports"
  }
}
```

### 利用可能なプロファイル

- **strict**: プロダクション対応コードのための厳格なルール
- **recommended**: ほとんどのプロジェクトに適したバランスの取れたルール
- **relaxed**: 高速開発のための最小限のルール

## ルール

### コンポーネントルール

- `component-complexity`: コンポーネントの複雑度をチェック
- `change-detection-strategy`: OnPush 戦略を提案
- `too-many-inputs`: 入力プロパティの数を制限
- `too-many-outputs`: 出力プロパティの数を制限
- `missing-cleanup-pattern`: 適切なクリーンアップをチェック

### 依存関係ルール

- `circular-dependency`: 循環依存関係を検出
- `unused-dependency`: 未使用の依存関係を識別
- `deep-dependency-chain`: 依存関係の深さをチェック

### 状態管理ルール

- `consider-state-management`: 一元的な状態管理を提案
- `missing-unsubscribe-pattern`: 適切なサブスクリプション解除をチェック

### パフォーマンスルール

- `high-default-change-detection`: デフォルトの変更検知について警告
- `consider-lazy-loading`: 遅延読み込みを提案
- `potential-memory-leak`: メモリリークのリスクを識別

## 開発者向け情報

### コントリビューション

1. フォークしてクローン
2. 新しいブランチを作成
3. 変更を加えてテスト
4. プルリクエストを送信

### テスト

```bash
# 単体テスト
cargo test

# 統合テスト
cargo test --test integration
```

### パフォーマンス測定

```bash
# リリースビルドでベンチマーク
cargo bench
```

## ライセンス

MIT License

## サポート

問題やフィードバックがあれば、GitHub の Issues ページで報告してください。
