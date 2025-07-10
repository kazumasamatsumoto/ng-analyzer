import { Component, Input, Output, EventEmitter, OnInit, OnDestroy } from '@angular/core';

@Component({
  selector: 'app-sample',
  templateUrl: './sample.component.html',
  styleUrls: ['./sample.component.css']
})
export class SampleComponent implements OnInit, OnDestroy {
  @Input() title: string = '';
  @Input() data: any[] = [];
  @Output() itemSelected = new EventEmitter<any>();
  @Output() itemDeleted = new EventEmitter<string>();

  ngOnInit() {
    // Initialize component
  }

  ngOnDestroy() {
    // Cleanup subscriptions
  }

  onItemClick(item: any) {
    this.itemSelected.emit(item);
  }

  onDeleteItem(id: string) {
    this.itemDeleted.emit(id);
  }
}