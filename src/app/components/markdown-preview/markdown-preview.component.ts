import { Component } from '@angular/core';
import { MarkdownService } from 'ngx-markdown';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-markdown-preview',
  templateUrl: './markdown-preview.component.html',
  styleUrls: ['./markdown-preview.component.scss']
})
export class MarkdownPreviewComponent {
  markdown: string;
  constructor(private mdService: MarkdownService, private http: HttpClient) {
    this.markdown = "";
  }

  async ngOnInit() {
    const markdownRaw = await this.http.get('/markdown-files/about.md',
      { responseType: 'text' }).toPromise();

    this.markdown = this.mdService.parse(markdownRaw!);
  }
}
