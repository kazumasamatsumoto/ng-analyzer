# Angular 分析 CLI ツール設計案

## 1. 全体設計思想

### アーキテクチャの特徴

- **モジュラー設計**: 各機能を独立したモジュールとして実装
- **プラグインシステム**: 機能の追加・拡張が容易
- **TypeScript First**: 型安全性と IntelliSense の活用
- **設定駆動**: プロジェクトごとのカスタマイズ対応

### 技術スタック

- **言語**: TypeScript (Node.js)
- **CLI フレームワーク**: Commander.js
- **AST 解析**: TypeScript Compiler API
- **HTML 解析**: Angular Compiler API
- **レポート生成**: 複数フォーマット対応 (JSON, HTML, PDF)

## 2. 機能モジュール一覧

### 2.1 コア分析モジュール

#### A. コンポーネント分析 (`component-analyzer`)

```typescript
// 機能例
ng-analyzer component --path ./src/app --depth 3 --output report.json
```

**分析項目:**

- コンポーネント階層構造の可視化
- 親子関係の依存グラフ
- Input/Output プロパティの使用状況
- ライフサイクルフックの使用パターン
- テンプレート内の複雑度計測

#### B. 依存関係分析 (`dependency-analyzer`)

```typescript
ng-analyzer deps --circular --unused --format graph
```

**分析項目:**

- 循環依存の検出
- 未使用依存関係の特定
- モジュール間の結合度分析
- サービス注入パターンの検証

#### C. 状態管理分析 (`state-analyzer`)

```typescript
ng-analyzer state --ngrx --akita --signals --performance
```

**分析項目:**

- NgRx: Action/Reducer/Effect/Selector の整合性
- 状態の正規化度合い
- 不変性違反の検出
- パフォーマンス影響分析

#### D. パフォーマンス分析 (`performance-analyzer`)

```typescript
ng-analyzer perf --bundle-size --change-detection --lazy-loading
```

**分析項目:**

- バンドルサイズの最適化提案
- 変更検知戦略の分析
- OnPush 戦略の適用可能性
- レイジーローディングの最適化

### 2.2 コード品質モジュール

#### E. アクセシビリティ分析 (`a11y-analyzer`)

```typescript
ng-analyzer a11y --wcag-level AA --template-check --aria-validation
```

**分析項目:**

- ARIA 属性の適切な使用
- キーボードナビゲーション対応
- カラーコントラスト比チェック
- セマンティック HTML 検証

#### F. セキュリティ分析 (`security-analyzer`)

```typescript
ng-analyzer security --xss --csrf --sanitization
```

**分析項目:**

- XSS 脆弱性の検出
- HTML サニタイゼーションの検証
- 信頼できないデータの流れ追跡
- CSP ポリシーの妥当性確認

#### G. テスト分析 (`test-analyzer`)

```typescript
ng-analyzer test --coverage --quality --patterns
```

**分析項目:**

- テストカバレッジの詳細分析
- テストパターンの適切性
- モック使用の最適化提案
- E2E テスト戦略の評価

### 2.3 アーキテクチャ分析モジュール

#### H. 設計パターン分析 (`pattern-analyzer`)

```typescript
ng-analyzer patterns --mvvm --facade --observer --strategy
```

**分析項目:**

- 設計パターンの使用状況
- アンチパターンの検出
- アーキテクチャ違反の特定
- リファクタリング提案

#### I. API 分析 (`api-analyzer`)

```typescript
ng-analyzer api --http --interceptors --error-handling
```

**分析項目:**

- HTTP 通信パターンの分析
- エラーハンドリング戦略
- レスポンスキャッシュの効率性
- API 呼び出しの最適化

## 3. 実行モード

### 3.1 単一機能実行

```bash
# 特定の機能のみ実行
ng-analyzer component --path ./src/app/features
ng-analyzer deps --circular-only
ng-analyzer state --ngrx --performance
```

### 3.2 複合分析実行

```bash
# 複数機能を組み合わせて実行
ng-analyzer audit --full  # 全機能実行
ng-analyzer audit --performance --security --a11y  # 特定の組み合わせ
ng-analyzer audit --custom-profile production  # カスタムプロファイル
```

### 3.3 継続的統合モード

```bash
# CI/CDパイプライン用
ng-analyzer ci --threshold-file .ng-analyzer.json --fail-on-error
```

## 4. 設定システム

### 4.1 設定ファイル例 (`.ng-analyzer.json`)

```json
{
  "profiles": {
    "development": {
      "rules": {
        "component": {
          "maxDepth": 5,
          "maxComplexity": 10
        },
        "performance": {
          "bundleSizeLimit": "2MB",
          "changeDetectionStrategy": "OnPush"
        }
      }
    },
    "production": {
      "rules": {
        "security": {
          "strictMode": true,
          "xssProtection": true
        },
        "performance": {
          "bundleSizeLimit": "1MB",
          "treeShaking": true
        }
      }
    }
  },
  "ignore": ["**/*.spec.ts", "**/node_modules/**"],
  "output": {
    "format": ["json", "html"],
    "path": "./reports"
  }
}
```

## 5. 出力・レポート機能

### 5.1 対話型 HTML レポート

- **ダッシュボード**: 全体的な健全性スコア
- **ドリルダウン**: 詳細な問題箇所への遷移
- **可視化**: 依存関係グラフ、メトリクス推移
- **推奨事項**: 具体的な改善提案

### 5.2 JSON API

```json
{
  "summary": {
    "overallScore": 85,
    "issues": {
      "critical": 2,
      "warning": 15,
      "info": 8
    }
  },
  "modules": {
    "component": {
      "score": 90,
      "details": [...],
      "recommendations": [...]
    }
  }
}
```

### 5.3 IDE 統合

- **VS Code 拡張**: リアルタイム分析結果表示
- **問題パネル**: 検出された問題の一覧
- **クイックフィックス**: 自動修正提案

## 6. 拡張性・カスタマイズ

### 6.1 プラグインシステム

```typescript
// カスタムプラグイン例
export class CustomRulePlugin implements AnalyzerPlugin {
  name = "custom-naming-convention";

  analyze(project: NgProject): AnalysisResult {
    // カスタム分析ロジック
  }
}
```

### 6.2 ルールエンジン

```typescript
// カスタムルール定義
export const customRules: RuleDefinition[] = [
  {
    name: "component-naming",
    severity: "warning",
    check: (node) => node.name.endsWith("Component"),
    message: 'Component class should end with "Component"',
  },
];
```

## 7. 学習・教育機能

### 7.1 知識習得クイズ

```bash
ng-analyzer learn --topic state-management --level intermediate
```

### 7.2 ベストプラクティス提案

- コードレビュー時の自動提案
- 設計パターンの適用提案
- パフォーマンス最適化のヒント

## 8. 実装優先度

### フェーズ 1 (MVP)

1. コンポーネント分析
2. 依存関係分析
3. 基本的なレポート生成

### フェーズ 2

1. 状態管理分析
2. パフォーマンス分析
3. 設定システム

### フェーズ 3

1. セキュリティ分析
2. アクセシビリティ分析
3. IDE 統合

### フェーズ 4

1. プラグインシステム
2. 学習機能
3. 高度な可視化

## 9. 技術的な実装検討事項

### パフォーマンス最適化

- 大規模プロジェクトでの高速化
- 増分分析による効率化
- 並列処理の活用

### 精度向上

- 型情報の活用
- 動的解析との組み合わせ
- 実行時情報の収集

### 保守性

- モジュールの疎結合
- テスト可能な設計
- 明確な API インターフェース

# Rust 製 Angular 分析 CLI ツール設計案

## 1. Rust アーキテクチャの利点

### パフォーマンス優位性

- **ゼロコスト抽象化**: 高レベルな抽象化でも実行時オーバーヘッドなし
- **メモリ安全性**: ガベージコレクションなしでのメモリ安全
- **並行処理**: 所有権システムによる安全な並行処理
- **高速ファイル I/O**: 大規模プロジェクトでの高速解析

### 開発体験

- **型安全性**: TypeScript プロジェクトを型安全に解析
- **エラーハンドリング**: Result 型による堅牢なエラー処理
- **クロスプラットフォーム**: 単一バイナリでの配布

## 2. 技術スタック

### コア依存関係

```toml
[dependencies]
# CLI フレームワーク
clap = { version = "4.0", features = ["derive"] }
# 非同期処理
tokio = { version = "1.0", features = ["full"] }
# JSON/設定ファイル
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# ファイル操作
walkdir = "2.3"
ignore = "0.4"
# TypeScript/JavaScript解析
swc_ecma_parser = "0.140"
swc_ecma_ast = "0.110"
swc_common = "0.31"
# HTML解析
html5ever = "0.26"
# レポート生成
tabled = "0.12"
# 並行処理
rayon = "1.7"
# エラーハンドリング
anyhow = "1.0"
thiserror = "1.0"
```

### アーキテクチャ構造

```
ng-analyzer/
├── src/
│   ├── main.rs                 # エントリーポイント
│   ├── cli/                    # CLI関連
│   │   ├── mod.rs
│   │   ├── commands.rs         # コマンド定義
│   │   └── args.rs            # 引数解析
│   ├── analyzers/             # 分析モジュール
│   │   ├── mod.rs
│   │   ├── component.rs       # コンポーネント分析
│   │   ├── dependency.rs      # 依存関係分析
│   │   ├── state.rs           # 状態管理分析
│   │   └── performance.rs     # パフォーマンス分析
│   ├── parsers/               # パーサー
│   │   ├── mod.rs
│   │   ├── typescript.rs      # TypeScript解析
│   │   └── html.rs            # HTML解析
│   ├── ast/                   # AST操作
│   │   ├── mod.rs
│   │   ├── visitor.rs         # AST訪問者
│   │   └── analyzer.rs        # AST分析
│   ├── config/                # 設定管理
│   │   ├── mod.rs
│   │   └── rules.rs           # ルール定義
│   └── output/                # 出力・レポート
│       ├── mod.rs
│       ├── json.rs
│       └── html.rs
└── tests/
    ├── fixtures/              # テスト用プロジェクト
    └── integration/           # 統合テスト
```

## 3. 主要な型定義

### 3.1 コア型定義

```rust
// src/ast/mod.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgProject {
    pub root_path: PathBuf,
    pub components: Vec<NgComponent>,
    pub services: Vec<NgService>,
    pub modules: Vec<NgModule>,
    pub pipes: Vec<NgPipe>,
    pub directives: Vec<NgDirective>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgComponent {
    pub name: String,
    pub file_path: PathBuf,
    pub selector: Option<String>,
    pub template_url: Option<String>,
    pub template: Option<String>,
    pub style_urls: Vec<String>,
    pub inputs: Vec<NgInput>,
    pub outputs: Vec<NgOutput>,
    pub lifecycle_hooks: Vec<String>,
    pub dependencies: Vec<String>,
    pub change_detection: ChangeDetectionStrategy,
    pub complexity_score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgService {
    pub name: String,
    pub file_path: PathBuf,
    pub provided_in: Option<String>,
    pub injectable: bool,
    pub dependencies: Vec<String>,
    pub methods: Vec<NgMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeDetectionStrategy {
    Default,
    OnPush,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub project: NgProject,
    pub issues: Vec<Issue>,
    pub metrics: ProjectMetrics,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: Severity,
    pub rule: String,
    pub message: String,
    pub file_path: PathBuf,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}
```

### 3.2 分析器トレイト

```rust
// src/analyzers/mod.rs
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait Analyzer: Send + Sync {
    async fn analyze(&self, project: &NgProject) -> Result<AnalysisResult>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

#[async_trait]
pub trait Rule: Send + Sync {
    async fn check(&self, node: &AstNode) -> Result<Vec<Issue>>;
    fn severity(&self) -> Severity;
    fn name(&self) -> &'static str;
}
```

## 4. 主要機能の実装

### 4.1 TypeScript 解析器

```rust
// src/parsers/typescript.rs
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_ast::*;
use swc_common::SourceMap;
use std::sync::Arc;

pub struct TypeScriptParser {
    source_map: Arc<SourceMap>,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        Self {
            source_map: Arc::new(SourceMap::default()),
        }
    }

    pub fn parse_file(&self, content: &str) -> Result<Module, anyhow::Error> {
        let input = StringInput::new(content, BytePos(0), BytePos(content.len() as u32));
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                ..Default::default()
            }),
            EsVersion::Es2020,
            input,
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module()
            .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

        Ok(module)
    }
}
```

### 4.2 コンポーネント分析器

```rust
// src/analyzers/component.rs
use super::{Analyzer, AnalysisResult};
use crate::ast::{NgProject, NgComponent, Issue, Severity};
use async_trait::async_trait;
use anyhow::Result;

pub struct ComponentAnalyzer {
    max_complexity: u32,
    max_depth: u32,
}

impl ComponentAnalyzer {
    pub fn new() -> Self {
        Self {
            max_complexity: 10,
            max_depth: 5,
        }
    }

    fn analyze_component(&self, component: &NgComponent) -> Vec<Issue> {
        let mut issues = Vec::new();

        // 複雑度チェック
        if component.complexity_score > self.max_complexity {
            issues.push(Issue {
                severity: Severity::Warning,
                rule: "component-complexity".to_string(),
                message: format!(
                    "Component complexity ({}) exceeds threshold ({})",
                    component.complexity_score, self.max_complexity
                ),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        // 変更検知戦略チェック
        if matches!(component.change_detection, ChangeDetectionStrategy::Default) {
            issues.push(Issue {
                severity: Severity::Info,
                rule: "change-detection-strategy".to_string(),
                message: "Consider using OnPush change detection strategy".to_string(),
                file_path: component.file_path.clone(),
                line: None,
                column: None,
            });
        }

        issues
    }
}

#[async_trait]
impl Analyzer for ComponentAnalyzer {
    async fn analyze(&self, project: &NgProject) -> Result<AnalysisResult> {
        let mut all_issues = Vec::new();

        // 並列処理でコンポーネントを分析
        let issues: Vec<_> = project.components
            .iter()
            .map(|component| self.analyze_component(component))
            .collect();

        for issue_set in issues {
            all_issues.extend(issue_set);
        }

        Ok(AnalysisResult {
            project: project.clone(),
            issues: all_issues,
            metrics: ProjectMetrics::default(),
            recommendations: Vec::new(),
        })
    }

    fn name(&self) -> &'static str {
        "component"
    }

    fn description(&self) -> &'static str {
        "Analyzes Angular components for best practices and performance"
    }
}
```

### 4.3 並行処理での高速化

```rust
// src/analyzers/mod.rs
use rayon::prelude::*;
use std::collections::HashMap;

pub struct AnalysisEngine {
    analyzers: HashMap<String, Box<dyn Analyzer>>,
}

impl AnalysisEngine {
    pub fn new() -> Self {
        let mut analyzers: HashMap<String, Box<dyn Analyzer>> = HashMap::new();

        analyzers.insert("component".to_string(), Box::new(ComponentAnalyzer::new()));
        analyzers.insert("dependency".to_string(), Box::new(DependencyAnalyzer::new()));
        analyzers.insert("state".to_string(), Box::new(StateAnalyzer::new()));

        Self { analyzers }
    }

    pub async fn run_analysis(&self, project: &NgProject, analyzer_names: &[String]) -> Result<Vec<AnalysisResult>> {
        let results: Result<Vec<_>, _> = analyzer_names
            .par_iter()
            .map(|name| {
                let analyzer = self.analyzers.get(name)
                    .ok_or_else(|| anyhow::anyhow!("Unknown analyzer: {}", name))?;

                // 非同期処理をブロッキングで実行（並行処理内で）
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(analyzer.analyze(project))
                })
            })
            .collect();

        results
    }
}
```

## 5. CLI 実装

### 5.1 コマンド定義

```rust
// src/cli/commands.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ng-analyzer")]
#[command(about = "A powerful Angular project analyzer built with Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze Angular components
    Component {
        /// Path to analyze
        #[arg(short, long, default_value = "./src")]
        path: PathBuf,
        /// Maximum analysis depth
        #[arg(short, long, default_value = "5")]
        depth: u32,
        /// Output format
        #[arg(short, long, default_value = "json")]
        output: String,
    },
    /// Analyze dependencies
    Deps {
        /// Check for circular dependencies
        #[arg(long)]
        circular: bool,
        /// Find unused dependencies
        #[arg(long)]
        unused: bool,
        /// Output format
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    /// Analyze state management
    State {
        /// Analyze NgRx patterns
        #[arg(long)]
        ngrx: bool,
        /// Check performance implications
        #[arg(long)]
        performance: bool,
    },
    /// Run comprehensive audit
    Audit {
        /// Run all analyzers
        #[arg(long)]
        full: bool,
        /// Specific analyzers to run
        #[arg(long, value_delimiter = ',')]
        analyzers: Option<Vec<String>>,
        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}
```

### 5.2 メイン実装

```rust
// src/main.rs
use clap::Parser;
use ng_analyzer::cli::{Cli, Commands};
use ng_analyzer::analyzers::AnalysisEngine;
use ng_analyzer::parsers::ProjectParser;
use ng_analyzer::output::OutputFormatter;
use anyhow::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Component { path, depth, output } => {
            let project = ProjectParser::new().parse_project(&path).await?;
            let engine = AnalysisEngine::new();
            let results = engine.run_analysis(&project, &["component".to_string()]).await?;

            let formatter = OutputFormatter::new(&output);
            formatter.format_results(&results)?;
        }
        Commands::Audit { full, analyzers, config } => {
            let project = ProjectParser::new().parse_project(&std::env::current_dir()?).await?;
            let engine = AnalysisEngine::new();

            let analyzer_list = if full {
                vec!["component".to_string(), "dependency".to_string(), "state".to_string()]
            } else {
                analyzers.unwrap_or_else(|| vec!["component".to_string()])
            };

            let results = engine.run_analysis(&project, &analyzer_list).await?;

            let formatter = OutputFormatter::new("html");
            formatter.format_results(&results)?;
        }
        _ => {
            println!("Command not yet implemented");
        }
    }

    Ok(())
}
```

## 6. 設定システム

### 6.1 設定構造体

```rust
// src/config/mod.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub ignore: Vec<String>,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub rules: HashMap<String, RuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    pub enabled: bool,
    pub severity: String,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: Vec<String>,
    pub path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profiles: HashMap::new(),
            ignore: vec![
                "**/*.spec.ts".to_string(),
                "**/node_modules/**".to_string(),
            ],
            output: OutputConfig {
                format: vec!["json".to_string()],
                path: PathBuf::from("./reports"),
            },
        }
    }
}
```

## 7. パフォーマンス最適化

### 7.1 メモリ効率化

```rust
// ファイル読み込みの最適化
use std::fs::File;
use std::io::{BufRead, BufReader};
use memmap2::MmapOptions;

pub struct OptimizedFileReader;

impl OptimizedFileReader {
    pub fn read_large_file(path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;

        if metadata.len() > 1024 * 1024 { // 1MB以上
            // メモリマップを使用
            let mmap = unsafe { MmapOptions::new().map(&file)? };
            Ok(String::from_utf8_lossy(&mmap).to_string())
        } else {
            // 通常の読み込み
            let mut reader = BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content)?;
            Ok(content)
        }
    }
}
```

### 7.2 並行処理の活用

```rust
// 複数ファイルの並行解析
use rayon::prelude::*;
use std::sync::Arc;

pub async fn analyze_files_parallel(files: &[PathBuf]) -> Result<Vec<AnalysisResult>> {
    let parser = Arc::new(TypeScriptParser::new());

    let results: Result<Vec<_>, _> = files
        .par_iter()
        .map(|file| -> Result<AnalysisResult> {
            let content = std::fs::read_to_string(file)?;
            let ast = parser.parse_file(&content)?;
            // 解析処理
            Ok(AnalysisResult::default())
        })
        .collect();

    results
}
```

## 8. テスト戦略

### 8.1 単体テスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_component_analysis() {
        let temp_dir = TempDir::new().unwrap();
        let component_file = temp_dir.path().join("test.component.ts");

        std::fs::write(&component_file, r#"
            @Component({
                selector: 'app-test',
                template: '<div>Test</div>'
            })
            export class TestComponent {}
        "#).unwrap();

        let analyzer = ComponentAnalyzer::new();
        let project = create_test_project(temp_dir.path());
        let result = analyzer.analyze(&project).await.unwrap();

        assert!(!result.issues.is_empty());
    }
}
```

## 9. 配布・インストール

### 9.1 バイナリ配布

```bash
# GitHub Releasesでの配布
cargo install ng-analyzer

# 特定バージョンの指定
cargo install ng-analyzer --version 1.0.0

# ソースからのビルド
git clone https://github.com/your-org/ng-analyzer.git
cd ng-analyzer
cargo build --release
```

### 9.2 Docker 対応

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/ng-analyzer /usr/local/bin/
ENTRYPOINT ["ng-analyzer"]
```

## 10. 知識習得クイズ

**Q1**: Rust の所有権システムが、この Angular 分析ツールの並行処理でどのような安全性を提供しますか？

**Q2**: `swc_ecma_parser`を使用することで、従来の JavaScript/TypeScript パーサーと比べてどのような利点がありますか？

**Q3**: Rayon を使った並行処理と、Tokio を使った非同期処理の使い分けは何を基準に行うべきですか？

**正解は後でお答えします！**

Rust での実装により、従来の Node.js 製ツールと比べて大幅な性能向上が期待できます。特に大規模な Angular プロジェクトでの分析において、その差は顕著に現れるでしょう。
