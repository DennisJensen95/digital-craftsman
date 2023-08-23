import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AboutComponent } from './components/about/about.component';
import { ResumeComponent } from './components/resume/resume.component';
import { TechSkillsComponent } from './components/tech-skills/tech-skills.component';
import { BlogPostsComponent } from './components/blog-posts/blog-posts.component';
import { BrowserModule } from '@angular/platform-browser';


const routes: Routes = [
  {
    path: 'about',
    component: AboutComponent
  },
  {
    path: 'resume',
    component: ResumeComponent
  },
  {
    path: 'tech-skills',
    component: TechSkillsComponent
  },
  {
    path: 'blog',
    component: BlogPostsComponent
  },
  { path: '', redirectTo: '/about', pathMatch: 'full' },
];

@NgModule({
  imports: [
    BrowserModule,
    RouterModule.forRoot(routes)
  ],
  exports: [RouterModule],
})
export class AppRoutingModule {


}
