import { Component } from '@angular/core';
import { MarkdownService } from 'ngx-markdown';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-resume',
  templateUrl: './resume.component.html',
  styleUrls: ['./resume.component.scss']
})
export class ResumeComponent {
  positionsContents: string[] = [];
  projectsContents: string[] = [];

  positionsAndProjects = [
    {
      position: {
        url: '/markdown-files/resume/position_software_engineer_in_data.md',
        content: ''
      },
      projects: [
        {
          url: '/markdown-files/resume/project_aim.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_qio.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_tprm.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_mlops_platform.md',
          content: ''
        }
      ]
    },
    {
      position: {
        url: '/markdown-files/resume/position_software_engineer_iot.md',
        content: ''
      },
      projects: [
        {
          url: '/markdown-files/resume/project_software_engineer_iot.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_software_engineer_iot_devops.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_frontend_software_engineer_iot.md',
          content: ''
        }
      ]
    },
    {
      position: {
        url: '/markdown-files/resume/position_student_software_engineer_iot.md',
        content: ''
      },
      projects: [
      ]
    },
    {
      position: {
        url: '/markdown-files/resume/position_test_automation_engineer.md',
        content: ''
      },
      projects: [
      ]
    },
    {
      position: {
        url: '/markdown-files/resume/position_bolig_portal.md',
        content: ''
      },
      projects: [
      ]
    },
    {
      position: {
        url: "/markdown-files/resume/position_personal_projects.md",
        content: ''
      },
      projects: [
        {
          url: '/markdown-files/resume/project_digital_craftsman.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_coverage_scope.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_ler_gml_converter.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_subplan.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_facebook_scraping_notifier.md',
          content: ''
        },
        {
          url: '/markdown-files/resume/project_energinet.md',
          content: ''
        }
      ]
    }
  ]


  constructor(private markdownService: MarkdownService, private http: HttpClient) {
    for (let item of this.positionsAndProjects) {
      this.http.get(item.position.url, { responseType: 'text' }).subscribe(data => {
        item.position.content = this.markdownService.parse(data);
      });

      for (let project of item.projects) {
        this.http.get(project.url, { responseType: 'text' }).subscribe(data => {
          project.content = this.markdownService.parse(data);
          console.log(project.content);
        });
      }
    }
  }

}
