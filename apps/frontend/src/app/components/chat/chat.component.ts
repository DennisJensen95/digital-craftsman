import { Component } from '@angular/core';

@Component({
  selector: 'app-chat',
  templateUrl: './chat.component.html',
  styleUrls: ['./chat.component.scss']
})
export class ChatComponent {
  messages: { sender: string, text: string }[] = [];
  inputMessage: string = '';
  loading: boolean = false;

  constructor() { }

  sendMessage() {
    if (this.inputMessage.trim() === '') return;

    this.messages.push({ sender: 'You asked', text: this.inputMessage });

    this.messages.push({ sender: 'Digital Craftsman\'s Assistant', text: '' });
    let message_index = this.messages.length - 1;

    this.loading = true;

    // Capture this context
    const self = this;

    fetch('/chat', {
      method: 'POST',
      body: JSON.stringify({ question: this.inputMessage }),
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(res => {
      if (res.body == null) {
        return;
      }

      self.loading = false;

      const reader = res.body.getReader();
      const decoder = new TextDecoder('utf-8');

      return new ReadableStream({
        start(controller) {
          const read = (): Promise<void> => {
            return reader.read().then(({ done, value }) => {
              if (done) {
                controller.close();
                return;
              }

              const chunk = decoder.decode(value);
              self.messages[message_index].text += chunk;

              controller.enqueue(value);
              return read();
            });
          };

          return read();
        }
      });
    });
  }


  resetChat() {
    this.messages = [];
  }
}
