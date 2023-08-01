import { Component } from '@angular/core';
import { MarkdownService } from 'ngx-markdown';
import { HttpClient } from '@angular/common/http';


interface BlogPost {
  title: string;
  link: string;
  content: string;
}

interface MarkdownBlog {
  path: string;
  title: string;
}


@Component({
  selector: 'app-blog-posts',
  templateUrl: './blog-posts.component.html',
  styleUrls: ['./blog-posts.component.scss']
})
export class BlogPostsComponent {

  expandedPostIndex: number | null = null;

  toggleContent(index: number) {
    this.expandedPostIndex = this.expandedPostIndex === index ? null : index;
  }

  // Make a dictionary with title, link and markdown content

  blogPosts: BlogPost[] = [
    {title: 'Lightweight code coverage quality gate', link: 'https://medium.com/@darbj95/lightweight-code-coverage-quality-gate-bc595d18bf1', content: ''},
  ];
  blogs: MarkdownBlog[] = [
    {title: 'The power of unit testing', path: '/markdown-files/blog-posts/power-of-unit-testing.md'}
  ]

  constructor(private markdownService: MarkdownService, private http: HttpClient) {
    for (let markdownBlog of this.blogs) {
      this.http.get(markdownBlog.path, { responseType: 'text' }).subscribe(data => {
        let parsedData = this.markdownService.parse(data);
        let blogPost: BlogPost = {title: markdownBlog.title, link: '', content: parsedData};
        this.blogPosts.push(blogPost);
      });
    }
  }
  

}
