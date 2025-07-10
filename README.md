# ng-analyzer

Rustで構築された高性能なAngularプロジェクト分析ツールです。爆速のパフォーマンスと包括的な分析を提供します。

## 機能

- 🚀 **高性能**: Rustで構築された最高速度と効率性
- 🔍 **包括的分析**: コンポーネント、サービス、依存関係、パフォーマンスパターンの分析
- 🎯 **複数のアナライザー**: コンポーネント、依存関係、状態管理、パフォーマンス分析
- 📊 **豊富なレポート**: JSON、HTML、テーブル形式での出力
- ⚡ **並列処理**: Rustの並行性を活用した大規模プロジェクトの高速分析
- 🛠️ **設定可能なルール**: カスタマイズ可能な分析ルールと重要度レベル
- 📈 **詳細なメトリクス**: プロジェクト統計と複雑度測定

## インストール

### ソースからビルド

```bash
git clone https://github.com/your-org/ng-analyzer.git
cd ng-analyzer
cargo build --release
```

### Cargoを使用

```bash
cargo install ng-analyzer
```

## クイックスタート

### コンポーネント分析

```bash
ng-analyzer component --path ./src
```

### 依存関係分析

```bash
ng-analyzer deps --path ./src --circular --unused
```

### 完全監査実行

```bash
ng-analyzer audit --path ./src --full --formats json,html,table
```

### 設定初期化

```bash
ng-analyzer init --profile recommended
```

## アナライザー

### コンポーネントアナライザー

Angularコンポーネントの以下の項目を分析します：
- 複雑度スコア
- 変更検知戦略
- 入力/出力プロパティ
- ライフサイクルフックの使用
- テンプレートとスタイルの構成

```bash
ng-analyzer component --max-complexity 8 --output html
```

### 依存関係アナライザー

プロジェクトの依存関係を分析します：
- 循環依存関係
- 未使用の依存関係
- 深い依存関係チェーン
- アーキテクチャパターン

```bash
ng-analyzer deps --circular --depth --format table
```

### 状態管理アナライザー

状態管理パターンを分析します：
- 状態サービスの識別
- NgRxパターンの検出
- サブスクリプション管理
- メモリリークのリスク

```bash
ng-analyzer state --subscriptions --change-detection
```

### パフォーマンスアナライザー

パフォーマンスへの影響を分析します：
- バンドルサイズへの影響
- 変更検知のパフォーマンス
- 遅延読み込みの機会
- メモリリークの検出

```bash
ng-analyzer performance --bundle-size --lazy-loading --memory-leaks
```

## 出力フォーマット

### JSON出力

```bash
ng-analyzer component --output json > analysis.json
```

### HTMLレポート

```bash
ng-analyzer audit --formats html --output-dir ./reports
```

### テーブル出力

```bash
ng-analyzer deps --format table
```

## 設定

### 設定ファイル

設定ファイルを作成するには：

```bash
ng-analyzer init --profile strict
```

設定例：

```json
{
  "profiles": {
    "strict": {
      "name": "Strict",
      "description": "プロダクション対応コードのための厳格なルール",
      "rules": {
        "component-complexity": {
          "enabled": true,
          "severity": "error",
          "options": {
            "max_complexity": 8
          }
        }
      }
    }
  },
  "ignore": [
    "**/*.spec.ts",
    "**/node_modules/**"
  ],
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
- `change-detection-strategy`: OnPush戦略を提案
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

## CLIリファレンス

### グローバルオプション

- `--verbose`: 詳細な出力を有効化
- `--quiet`: 必須でない出力を抑制

### コンポーネントコマンド

```bash
ng-analyzer component [OPTIONS]
```

- `--path <PATH>`: 分析するパス (デフォルト: ./src)
- `--max-complexity <NUM>`: 最大複雑度しきい値 (デフォルト: 10)
- `--depth <NUM>`: 最大分析深度 (デフォルト: 5)
- `--output <FORMAT>`: 出力フォーマット (json, table, html)
- `--errors-only`: エラーと警告のみを表示

### 監査コマンド

```bash
ng-analyzer audit [OPTIONS]
```

- `--path <PATH>`: 分析するパス
- `--full`: すべてのアナライザーを実行
- `--analyzers <LIST>`: 実行する特定のアナライザー
- `--config <FILE>`: 設定ファイルのパス
- `--output-dir <DIR>`: レポートの出力ディレクトリ
- `--formats <LIST>`: 出力フォーマット（カンマ区切り）
- `--severity <LEVEL>`: 重要度しきい値 (error, warning, info)

## パフォーマンス

ng-analyzerはパフォーマンスを重視して構築されています：

- **並列分析**: Rayonを使用した並列処理
- **メモリ効率**: ゼロコピー解析による最適化されたメモリ使用
- **高速TypeScript解析**: SWCを使用した超高速TypeScript解析
- **並行I/O**: より良いパフォーマンスのための非同期ファイル操作

## 貢献

1. リポジトリをフォーク
2. 機能ブランチを作成
3. 変更を加える
4. テストを追加
5. プルリクエストを送信

## ライセンス

MIT License - 詳細はLICENSEファイルを参照してください。

## 変更履歴

### v0.1.0

- 初回リリース
- コンポーネントアナライザー
- 依存関係アナライザー
- 状態管理アナライザー
- パフォーマンスアナライザー
- JSON、HTML、テーブル出力フォーマット
- 設定可能なルールとプロファイル