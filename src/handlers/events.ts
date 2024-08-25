import { Client, ClientEvents } from "discord.js";
import { AbstractHandler } from "../abstracts/handlers.js";
import { AppEventBuilder } from "../builders/events.js";

export class EventsHandler<Client_ extends Client> extends AbstractHandler<Client_, AppEventBuilder> {
  protected override loadModule(event: AppEventBuilder): void | Promise<void> {
    if (!Reflect.has(this.client, event.type))
      return;

    Reflect.apply(this.client[event.type], this.client, [event.name as keyof ClientEvents, (...args: any) => event.callback(...args)]);
  }

  protected override preload(): void | Promise<void> {
    this.client.removeAllListeners();
  }
};