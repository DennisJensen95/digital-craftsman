import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";
import { FormsModule } from "@angular/forms";

import { AppRoutingModule } from "./app-routing.module";
import { AppComponent } from "./app.component";

import { MarkdownModule } from "ngx-markdown";
import { HttpClientModule } from "@angular/common/http";
import { AboutComponent } from "./components/about/about.component";
import { ResumeComponent } from "./components/resume/resume.component";
import { TechSkillsComponent } from "./components/tech-skills/tech-skills.component";
import { BlogPostsComponent } from "./components/blog-posts/blog-posts.component";
import { ChatComponent } from "./components/chat/chat.component";
import { CvComponent } from "./components/cv/cv.component";

@NgModule({
  declarations: [
    AppComponent,
    AboutComponent,
    ResumeComponent,
    TechSkillsComponent,
    BlogPostsComponent,
    ChatComponent,
    CvComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    MarkdownModule.forRoot(),
    FormsModule,
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
