import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-chat',
  templateUrl: './chat.component.html',
  styleUrls: ['./chat.component.scss']
})
export class ChatComponent {
  messages: { sender: string, text: string }[] = [];
  inputMessage: string = '';

  constructor(private http: HttpClient) { }

  loading: boolean = false;

  async sendMessage() {
    if (this.inputMessage.trim() === '') return;

    // Add the user's message to the chat
    this.messages.push({ sender: 'You asked', text: this.inputMessage });
    this.loading = true; // Set loading state for this message

    try {
      const data = await this.http.post('/chat', { question: this.inputMessage }).toPromise();
      let parsedData = JSON.parse(JSON.stringify(data));
      this.messages.push({ sender: 'Digital Craftsman\'s Assistant', text: parsedData.response });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    this.loading = false; // Unset loading state for this message
    this.inputMessage = ''; // Clear input for new questions
  }

  resetChat() {
    this.messages = [];
  }
}
