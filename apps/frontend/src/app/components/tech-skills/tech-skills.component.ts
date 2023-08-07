import { Component } from '@angular/core';
import { MarkdownService } from 'ngx-markdown';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-tech-skills',
  templateUrl: './tech-skills.component.html',
  styleUrls: ['./tech-skills.component.scss']
})
export class TechSkillsComponent {
  skillsContents: string[] = [];
  skills: string[] = [
    '/markdown-files/tech-skills/roles.md',
    '/markdown-files/tech-skills/industries.md',
    '/markdown-files/tech-skills/technologies.md',
    '/markdown-files/tech-skills/frameworks.md',
    '/markdown-files/tech-skills/infra.md',
    '/markdown-files/tech-skills/devops.md',
    '/markdown-files/tech-skills/databases.md',
    '/markdown-files/tech-skills/conceptual.md',
  ];

  constructor(private markdownService: MarkdownService, private http: HttpClient) {
    for (let file of this.skills) {
      this.http.get(file, { responseType: 'text' }).subscribe(data => {
        let parsedData = this.markdownService.parse(data);
        this.skillsContents.push(parsedData);
      });
    }
  }

}
