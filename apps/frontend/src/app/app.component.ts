// app.component.ts
import { Component } from "@angular/core";
import { Router } from "@angular/router";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.scss"],
})
export class AppComponent {
  title = "digital-craftsman";

  constructor(private router: Router) {}

  isCV(): boolean {
    return this.router.url === "/cv";
  }
}
