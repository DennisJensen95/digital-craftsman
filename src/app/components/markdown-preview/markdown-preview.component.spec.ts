import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MarkdownPreviewComponent } from './markdown-preview.component';

describe('MarkdownPreviewComponent', () => {
  let component: MarkdownPreviewComponent;
  let fixture: ComponentFixture<MarkdownPreviewComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [MarkdownPreviewComponent]
    });
    fixture = TestBed.createComponent(MarkdownPreviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
