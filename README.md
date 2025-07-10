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
- 🔍 **高度な検索**: 正規表現、HTML クラス、関数名、構造的検索をサポート
- 🔧 **クロスプラットフォーム**: Windows、macOS、Linux で統一されたファイルパス表示

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
ng-analyzer [OPTIONS] <COMMAND> <PATH> [ARGS]
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

**実際の出力例:**

```
🔍 Starting Angular project analysis...
📁 Analyzing path: ./src
📊 Found 16 components, 0 services, 0 modules

📈 Analysis Summary:
   Total issues found: 44
   Issues shown: 44
   ❌ Errors: 16
   ⚠️  Warnings: 12
   💡 Recommendations: 2
Analysis completed in 0.09s
```

### 2. 依存関係分析

プロジェクトの依存関係を分析します。

```bash
# 基本的な依存関係分析
ng-analyzer deps ./src

# 循環依存関係をチェック
ng-analyzer deps ./src --circular

# 未使用依存関係を検出
ng-analyzer deps ./src --unused

# 依存関係の深度を分析
ng-analyzer deps ./src --depth

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

# NgRx パターンを分析
ng-analyzer state ./src --ngrx

# サブスクリプション管理をチェック
ng-analyzer state ./src --subscriptions

# 変更検知への影響を分析
ng-analyzer state ./src --change-detection

# JSON形式で出力
ng-analyzer state ./src --format json
```

### 4. パフォーマンス分析

パフォーマンスに影響する要因を分析します。

```bash
# パフォーマンス分析
ng-analyzer performance ./src

# バンドルサイズへの影響をチェック
ng-analyzer performance ./src --bundle-size

# 遅延読み込み機会を分析
ng-analyzer performance ./src --lazy-loading

# メモリリークリスクをチェック
ng-analyzer performance ./src --memory-leaks

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

# 設定ファイルを指定
ng-analyzer audit ./src --config ./custom-config.json
```

### 6. 設定初期化

プロジェクト設定ファイルを作成します。

```bash
# 推奨設定で初期化
ng-analyzer init --profile recommended

# 厳格な設定で初期化
ng-analyzer init --profile strict

# リラックスした設定で初期化
ng-analyzer init --profile relaxed

# 設定ファイルの出力先を指定
ng-analyzer init --output ./ng-analyzer.json
```

### 7. アナライザー一覧

利用可能なアナライザーとルールを表示します。

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

# ファイルパターンを指定
ng-analyzer search ./src --keyword "service" --file-pattern "*.service.ts"
```

#### 高度な検索機能

**HTML クラス名検索** - HTML の class 属性内でクラス名を検索

```bash
# 特定のCSSクラスの使用箇所を検索
ng-analyzer search ./src --keyword "btn-primary" --html-class

# Bootstrapクラスの使用状況を調査
ng-analyzer search ./src --keyword "col-md-" --html-class --file-type html

# Angular Materialクラスの使用を検索
ng-analyzer search ./src --keyword "mat-" --html-class --output table

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

# フォームラベルのテキストを検索
ng-analyzer search ./src --keyword "Username" --html-text --output json
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
ng-analyzer search ./src --keyword "handleClick" --function-name --output table

# コンストラクタを検索
ng-analyzer search ./src --keyword "constructor" --function-name --line-numbers
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

# TypeScript型注釈を検索
ng-analyzer search ./src --keyword ":\s*(string|number|boolean)\[\]" --regex
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

# Observable パターンを検索
ng-analyzer search ./src --structural "Observable<.*>" --file-type ts
```

#### 検索出力フォーマット

**シンプル形式（デフォルト）**

```bash
ng-analyzer search ./src --keyword "Component" --function-name

# 実際の出力例:
📄 ./src/app/advanced/advanced/advanced.component.ts
   3 matches found
   → import { [Component], OnInit, OnDestroy } from '@angular/core'; (simple)
   → @[Component]({ (simple)
   → export class Advanced[Component] implements OnInit, OnDestroy { (simple)

📄 ./src/app/navigation/navigation.component.ts
   3 matches found
   → import { [Component] } from '@angular/core'; (simple)
   → @[Component]({ (simple)
   → export class Navigation[Component] { (simple)

🔍 Search Summary:
   Files with matches: 17
   Total matches: 67
```

**テーブル形式**

```bash
ng-analyzer search ./src --keyword "Component" --html-class --output table

# 出力例:
File                                     Line   Type            Content
---------------------------------------------------------------------------------
src/app/login/login.component.html       15     html_class      <button class="btn btn-primary">Login</button>
src/app/header/header.component.html     8      html_class      <a class="btn btn-primary btn-sm">Sign Up</a>
```

**JSON 形式**

```bash
ng-analyzer search ./src --keyword "ngOnInit" --function-name --output json

# 出力例:
[
  {
    "file_path": "./src/app/advanced/advanced/advanced.component.ts",
    "total_matches": 1,
    "matches": [
      {
        "line_number": 45,
        "line_content": "  ngOnInit() {",
        "match_start": 2,
        "match_end": 10,
        "context_before": [],
        "context_after": [],
        "match_type": "function_name"
      }
    ]
  }
]
```

#### 実用的な検索例

**1. Angular アプリケーションの監査**

```bash
# すべてのコンポーネントを検索
ng-analyzer search ./src --keyword "Component" --regex --output json > components.json

# 古いAngular APIの使用を検索
ng-analyzer search ./src --keyword "Http[^C]" --regex --file-type ts

# サブスクリプションの管理状況を確認
ng-analyzer search ./src --keyword "subscribe" --regex --context 3

# OnDestroy 実装の確認
ng-analyzer search ./src --keyword "OnDestroy" --regex --function-name
```

**2. CSS クラスの使用状況調査**

```bash
# Material UI クラスの使用状況
ng-analyzer search ./src --keyword "mat-" --html-class --file-type html

# Bootstrap クラスの使用状況
ng-analyzer search ./src --keyword "btn-" --html-class --output table

# カスタムCSSクラスの使用箇所
ng-analyzer search ./src --keyword "custom-" --html-class

# 未使用の可能性があるクラスを検索
ng-analyzer search ./src --keyword "deprecated" --html-class --context 2
```

**3. セキュリティ監査**

```bash
# innerHTML使用箇所の確認
ng-analyzer search ./src --keyword "innerHTML" --regex --file-type ts

# 外部URLの使用を検索
ng-analyzer search ./src --keyword "http[s]?://" --regex --context 2

# ローカルストレージの使用を検索
ng-analyzer search ./src --keyword "localStorage" --regex --file-type ts

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

# Any型の使用箇所を検索
ng-analyzer search ./src --keyword ":\s*any" --regex --file-type ts
```

**5. コードベースの理解**

```bash
# すべてのライフサイクルフックの実装を確認
ng-analyzer search ./src --keyword "ng(OnInit|OnDestroy|OnChanges)" --regex --function-name

# エラーハンドリングの実装を検索
ng-analyzer search ./src --keyword "catch|error" --regex --context 2

# 環境設定の使用箇所を検索
ng-analyzer search ./src --keyword "environment\." --regex --file-type ts

# ルーティング設定を検索
ng-analyzer search ./src --keyword "RouterModule|Routes" --regex --file-type ts
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

JSON 形式での出力例（修正されたファイルパス表示）：

```json
{
  "project": {
    "root_path": "./src",
    "components": [
      {
        "name": "AdvancedComponent",
        "file_path": "./src/app/advanced/advanced/advanced.component.ts",
        "selector": "app-advanced",
        "template_url": null,
        "template": null,
        "style_urls": [],
        "inputs": [],
        "outputs": [],
        "lifecycle_hooks": ["ngOnInit", "ngOnDestroy"],
        "dependencies": [],
        "change_detection": "Default",
        "complexity_score": 13
      }
    ],
    "services": [],
    "modules": []
  },
  "issues": [
    {
      "severity": "Warning",
      "rule": "component-complexity",
      "message": "Component complexity (13) exceeds threshold (10). Consider breaking down into smaller components.",
      "file_path": "./src/app/advanced/advanced/advanced.component.ts",
      "line": null,
      "column": null
    },
    {
      "severity": "Error",
      "rule": "missing-template",
      "message": "Component must have either a template or templateUrl",
      "file_path": "./src/app/advanced/advanced/advanced.component.ts",
      "line": null,
      "column": null
    }
  ],
  "recommendations": [
    {
      "category": "Performance",
      "title": "Optimize Change Detection",
      "description": "Consider implementing OnPush change detection strategy for 16 components to improve performance",
      "priority": "Medium",
      "file_path": null
    }
  ]
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

# 高度な検索でパフォーマンス問題を特定
ng-analyzer search ./src --keyword "ngFor.*track" --regex --file-type html
```

### 4. レガシーコードのリファクタリング

```bash
# 古い API の使用を検索
ng-analyzer search ./src --keyword "Http[^C]" --regex --file-type ts

# 未使用のコンポーネントを特定
ng-analyzer search ./src --keyword "Component" --function-name --output json > components.json

# 複雑なコンポーネントを特定
ng-analyzer component ./src --max-complexity 8 --output table
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

2. **コマンドライン引数エラー**

   **問題**: `ng-analyzer component --path ./src` でエラーが発生

   **解決**: 新しい構文では `path` は位置引数です

   ```bash
   # ❌ 古い書き方
   ng-analyzer component --path ./src

   # ✅ 正しい書き方
   ng-analyzer component ./src
   ```

3. **ファイルパス表示の問題**

   **問題**: Windows でファイルパスに `\\` が表示される

   **解決**: 最新版では自動的に `/` に正規化されます

   ```bash
   # 修正前: "./src\\app\\component.ts"
   # 修正後: "./src/app/component.ts"
   ```

4. **メモリ不足**

   ```bash
   # スタックサイズを増やす
   export RUST_MIN_STACK=8388608
   ng-analyzer audit ./src
   ```

5. **パースエラー**

   ```bash
   # 詳細なエラー情報を表示
   ng-analyzer component ./src --verbose
   ```

6. **検索が遅い場合**

   ```bash
   # ファイルタイプを限定
   ng-analyzer search ./src --keyword "Component" --file-type ts

   # パターンを限定
   ng-analyzer search ./src --keyword "service" --file-pattern "*.service.ts"
   ```

### パフォーマンスの最適化

- 大規模プロジェクトでは`--depth`オプションで分析の深さを制限
- 必要なアナライザーのみを実行（`--analyzers`オプション）
- 検索時は`--file-type`で範囲を限定
- 並列処理を活用するため、十分なメモリを確保

### デバッグ方法

```bash
# 詳細ログを有効にする
ng-analyzer component ./src --verbose

# 静かな出力でエラーのみ表示
ng-analyzer component ./src --quiet

# JSON 出力で結果をパース
ng-analyzer component ./src --output json | jq '.issues[] | select(.severity == "Error")'
```

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
        },
        "missing-template": {
          "enabled": true,
          "severity": "error"
        },
        "change-detection-strategy": {
          "enabled": true,
          "severity": "info"
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

- `component-complexity`: コンポーネントの複雑度をチェック（デフォルト: 10）
- `change-detection-strategy`: OnPush 戦略を提案
- `too-many-inputs`: 入力プロパティの数を制限（デフォルト: 8）
- `too-many-outputs`: 出力プロパティの数を制限（デフォルト: 5）
- `missing-cleanup-pattern`: 適切なクリーンアップをチェック
- `missing-template`: テンプレートまたは templateUrl の存在をチェック
- `template-conflict`: inline template と templateUrl の競合をチェック
- `inline-template-too-large`: 大きなインラインテンプレートを警告

### 依存関係ルール

- `circular-dependency`: 循環依存関係を検出
- `unused-dependency`: 未使用の依存関係を識別
- `deep-dependency-chain`: 依存関係の深さをチェック（デフォルト: 5）

### 状態管理ルール

- `consider-state-management`: 一元的な状態管理を提案
- `missing-unsubscribe-pattern`: 適切なサブスクリプション解除をチェック
- `complex-state-components`: 複雑な状態管理を持つコンポーネントを警告

### パフォーマンスルール

- `high-default-change-detection`: デフォルトの変更検知について警告
- `consider-lazy-loading`: 遅延読み込みを提案
- `potential-memory-leak`: メモリリークのリスクを識別
- `feature-module-organization`: フィーチャーモジュールの組織化を提案

## 開発者向け情報

### アーキテクチャ

```
ng-analyzer/
├── src/
│   ├── analyzers/          # 分析エンジン
│   │   ├── component.rs    # コンポーネント分析
│   │   ├── dependency.rs   # 依存関係分析
│   │   ├── state.rs        # 状態管理分析
│   │   └── performance.rs  # パフォーマンス分析
│   ├── ast/                # AST 定義
│   ├── cli/                # CLI インターフェース
│   ├── config/             # 設定管理
│   ├── output/             # 出力フォーマッター
│   ├── parsers/            # パーサー（TypeScript、HTML）
│   └── search/             # 検索エンジン
└── tests/                  # テストファイル
```

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

# 特定のテストのみ実行
cargo test component_analysis
```

### パフォーマンス測定

```bash
# リリースビルドでベンチマーク
cargo bench

# プロファイリング
cargo run --release -- component ./large-project --verbose
```

### 新しいアナライザーの追加

1. `src/analyzers/` に新しいファイルを作成
2. `Analyzer` トレイトを実装
3. `AnalysisEngine` に追加
4. ルール定義を `src/config/rules.rs` に追加
5. テストを作成

## 変更履歴

### v0.1.0 (最新)

**新機能:**

- ✅ CLI 引数の改善（path を位置引数に変更）
- ✅ 高度な検索機能（正規表現、HTML クラス、関数名、構造的検索）
- ✅ クロスプラットフォーム対応のファイルパス表示
- ✅ 16 種類のコンポーネント分析ルール
- ✅ 実際の Angular プロジェクトでのテスト済み

**修正:**

- ✅ Windows でのファイルパス二重バックスラッシュ問題を解決
- ✅ SWC ライブラリの互換性問題を修正
- ✅ TypeScript パーサーの API 変更に対応

**パフォーマンス:**

- ✅ 大規模プロジェクトの分析時間を大幅短縮
- ✅ 並列処理による検索の高速化

## ライセンス

MIT License

## サポート

問題やフィードバックがあれば、GitHub の Issues ページで報告してください。

**よくある質問:**

- **Q**: コマンドが `unexpected argument` エラーを出します
- **A**: 最新版では `ng-analyzer component ./src` のように path を直接指定してください

- **Q**: ファイルパスが `\\` で表示されます
- **A**: 最新版では自動的に `/` に正規化されます

- **Q**: 検索機能の使い方がわかりません
- **A**: `ng-analyzer search --help` で詳細なヘルプを確認してください
