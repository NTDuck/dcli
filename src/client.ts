import { Client, ClientOptions } from "discord.js";
import { CommandsHandler } from "./handlers/commands.js";
import { EventsHandler } from "./handlers/events.js";
import FileSystem from "./helpers/filesystem.js";

export class AppClient extends Client {
  private readonly handlers = {
    commandsHandler: new CommandsHandler(this),
    eventsHandler: new EventsHandler(this),
  };

  public constructor(options: ClientOptions) {
    super(options);
  }

  public async loadBlueprints(options: FileSystem.FindFileOptions, dirNames: {
    commandsDirName: string,
    eventsDirName: string,
  }) {
    await Promise.all([
      this.handlers.commandsHandler.loadBlueprints(options, dirNames.commandsDirName),
      this.handlers.eventsHandler.loadBlueprints(options, dirNames.eventsDirName),
    ]);
  }
};