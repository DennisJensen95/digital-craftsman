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
    '/markdown-files/tech-skills/coding.md',
    '/markdown-files/tech-skills/conceptual.md',
    '/markdown-files/tech-skills/devops-and-cloud.md',
    '/markdown-files/tech-skills/build-system.md',
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
