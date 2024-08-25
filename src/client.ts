import { Client, ClientOptions } from "discord.js";
import { CommandsHandler } from "./handlers/commands.js";
import { EventsHandler } from "./handlers/events.js";
import FileSystem from "./auxiliaries/filesystem.js";
import { MappedType } from "./auxiliaries/reflection.js";

export class AppClient extends Client {
  private readonly handlers;

  public constructor(options: ClientOptions) {
    super(options);

    this.handlers = {
      commands: new CommandsHandler(this),
      events: new EventsHandler(this),
    };
  }

  async loadModules(options: FileSystem.Options, dirNames: MappedType<typeof this.handlers, string>) {
    const promises = new Array();

    for (const key of Object.keys(this.handlers)) {
      if (!Reflect.has(this.handlers, key))
        continue;

      const handler = Reflect.get(this.handlers, key);
      const dirName = Reflect.get(dirNames, key);
      promises.push(handler.loadModules(options, dirName));
    }

    await Promise.all(promises);
  }
};