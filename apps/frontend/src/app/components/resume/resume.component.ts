import { Component } from '@angular/core';
import { MarkdownService } from 'ngx-markdown';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-resume',
  templateUrl: './resume.component.html',
  styleUrls: ['./resume.component.scss']
})
export class ResumeComponent {
  // Drop downs
  togglePositions = false;
  toggleProjects = false;

  positionsContents: string[] = [];
  projectsContents: string[] = [];

  positions: string[] = [
    '/markdown-files/resume/position_software_engineer_in_data.md',
    '/markdown-files/resume/position_software_engineer_iot.md',
    '/markdown-files/resume/position_student_software_engineer_iot.md',
    '/markdown-files/resume/position_test_automation_engineer.md',
    '/markdown-files/resume/position_bolig_portal.md', 
  ];
  projects: string[] = [
    '/markdown-files/resume/project_energinet.md',
    '/markdown-files/resume/project_coverage_scope.md',
    '/markdown-files/resume/project_ler_gml_converter.md',
    '/markdown-files/resume/project_water_infrastructure_service.md',
    '/markdown-files/resume/project_subplan.md',
    '/markdown-files/resume/project_facebook_scraping_notifier.md',
    '/markdown-files/resume/project_thesis.md',
  ];

  constructor(private markdownService: MarkdownService, private http: HttpClient) {
    for (let file of this.positions) {
      this.http.get(file, { responseType: 'text' }).subscribe(data => {
        let parsedData = this.markdownService.parse(data);
        this.positionsContents.push(parsedData);
      });
    }

    for (let file of this.projects) {
      this.http.get(file, { responseType: 'text' }).subscribe(data => {
        let parsedData = this.markdownService.parse(data);
        this.projectsContents.push(parsedData);
      });
    }
  }
  
}
