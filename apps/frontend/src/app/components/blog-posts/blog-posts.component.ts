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

  handlePostClick(event: Event, post: any, index: number) {
    // If the clicked element is a link, do nothing (let the browser handle the navigation)
    if ((event.target as HTMLElement).tagName === 'A') {
        return;
    }
    
    // If there's a link, navigate to it
    if (post.link) {
        window.open(post.link, '_blank');
    } else {
        // Otherwise, toggle the content
        this.toggleContent(index);
    }
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
