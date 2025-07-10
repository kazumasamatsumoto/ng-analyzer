use ng_analyzer::ast::*;
use ng_analyzer::analyzers::*;
use ng_analyzer::parsers::*;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_component_analysis_integration() {
    let temp_dir = TempDir::new().unwrap();
    let component_path = temp_dir.path().join("test.component.ts");
    
    let component_content = r#"
import { Component, Input, Output, EventEmitter } from '@angular/core';

@Component({
  selector: 'app-test',
  template: '<div>{{ message }}</div>',
  styleUrls: ['./test.component.css']
})
export class TestComponent {
  @Input() message: string = '';
  @Output() messageChange = new EventEmitter<string>();
  
  private complexMethod() {
    // This is a complex method with multiple branches
    if (this.message.length > 0) {
      if (this.message.includes('test')) {
        if (this.message.startsWith('hello')) {
          return 'complex result';
        }
      }
    }
    return 'simple result';
  }
}
"#;

    std::fs::write(&component_path, component_content).unwrap();

    let parser = ProjectParser::new();
    let project = parser.parse_project(&temp_dir.path().to_path_buf()).await.unwrap();

    assert_eq!(project.components.len(), 1);
    let component = &project.components[0];
    assert_eq!(component.name, "TestComponent");
    assert_eq!(component.selector, Some("app-test".to_string()));
    assert_eq!(component.inputs.len(), 1);
    assert_eq!(component.outputs.len(), 1);

    let analyzer = component::ComponentAnalyzer::new();
    let result = analyzer.analyze(&project).await.unwrap();

    assert!(!result.issues.is_empty());
    assert_eq!(result.metrics.total_components, 1);
}

#[tokio::test]
async fn test_service_analysis_integration() {
    let temp_dir = TempDir::new().unwrap();
    let service_path = temp_dir.path().join("test.service.ts");
    
    let service_content = r#"
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class TestService {
  private data: string[] = [];
  
  getData() {
    return this.data;
  }
  
  setData(value: string) {
    this.data.push(value);
  }
}
"#;

    std::fs::write(&service_path, service_content).unwrap();

    let parser = ProjectParser::new();
    let project = parser.parse_project(&temp_dir.path().to_path_buf()).await.unwrap();

    assert_eq!(project.services.len(), 1);
    let service = &project.services[0];
    assert_eq!(service.name, "TestService");
    assert_eq!(service.provided_in, Some("root".to_string()));
    assert!(service.injectable);
}

#[tokio::test]
async fn test_dependency_analysis_integration() {
    let temp_dir = TempDir::new().unwrap();
    
    let service_a_path = temp_dir.path().join("service-a.service.ts");
    let service_a_content = r#"
import { Injectable } from '@angular/core';
import { ServiceB } from './service-b.service';

@Injectable({
  providedIn: 'root'
})
export class ServiceA {
  constructor(private serviceB: ServiceB) {}
  
  doSomething() {
    return this.serviceB.process();
  }
}
"#;

    let service_b_path = temp_dir.path().join("service-b.service.ts");
    let service_b_content = r#"
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ServiceB {
  process() {
    return 'processed';
  }
}
"#;

    std::fs::write(&service_a_path, service_a_content).unwrap();
    std::fs::write(&service_b_path, service_b_content).unwrap();

    let parser = ProjectParser::new();
    let project = parser.parse_project(&temp_dir.path().to_path_buf()).await.unwrap();

    assert_eq!(project.services.len(), 2);

    let analyzer = dependency::DependencyAnalyzer::new();
    let result = analyzer.analyze(&project).await.unwrap();

    assert!(!result.recommendations.is_empty());
}

#[tokio::test]
async fn test_performance_analysis_integration() {
    let temp_dir = TempDir::new().unwrap();
    let component_path = temp_dir.path().join("heavy.component.ts");
    
    let component_content = r#"
import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-heavy',
  template: `
    <div>
      <div *ngFor="let item of items">{{ item }}</div>
      <div *ngFor="let user of users">{{ user.name }}</div>
    </div>
  `,
  changeDetection: ChangeDetectionStrategy.Default
})
export class HeavyComponent {
  items = Array.from({length: 1000}, (_, i) => `Item ${i}`);
  users = Array.from({length: 1000}, (_, i) => ({name: `User ${i}`}));
}
"#;

    std::fs::write(&component_path, component_content).unwrap();

    let parser = ProjectParser::new();
    let project = parser.parse_project(&temp_dir.path().to_path_buf()).await.unwrap();

    let analyzer = performance::PerformanceAnalyzer::new();
    let result = analyzer.analyze(&project).await.unwrap();

    assert!(!result.recommendations.is_empty());
    let has_onpush_recommendation = result.recommendations.iter()
        .any(|r| r.title.contains("OnPush"));
    assert!(has_onpush_recommendation);
}

#[tokio::test]
async fn test_state_analysis_integration() {
    let temp_dir = TempDir::new().unwrap();
    let state_service_path = temp_dir.path().join("state.service.ts");
    
    let state_service_content = r#"
import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class StateService {
  private stateSubject = new BehaviorSubject<any>({});
  state$ = this.stateSubject.asObservable();
  
  updateState(newState: any) {
    this.stateSubject.next(newState);
  }
  
  getState() {
    return this.stateSubject.value;
  }
}
"#;

    std::fs::write(&state_service_path, state_service_content).unwrap();

    let parser = ProjectParser::new();
    let project = parser.parse_project(&temp_dir.path().to_path_buf()).await.unwrap();

    let analyzer = state::StateAnalyzer::new();
    let result = analyzer.analyze(&project).await.unwrap();

    assert!(!result.recommendations.is_empty());
}