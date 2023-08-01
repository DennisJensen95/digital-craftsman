import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TechSkillsComponent } from './tech-skills.component';

describe('TechSkillsComponent', () => {
  let component: TechSkillsComponent;
  let fixture: ComponentFixture<TechSkillsComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [TechSkillsComponent]
    });
    fixture = TestBed.createComponent(TechSkillsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
