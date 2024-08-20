import { Client, ClientOptions, Collection } from "discord.js";
import { EventRegistry, EventRegistryMethod, SlashCommandRegistry } from "./helpers/registry.js";
import { FileSystem } from "./helpers/filesystem.js";
import { pathToFileURL } from "node:url";

export class DClient extends Client {
  public readonly commands: Collection<string, SlashCommandRegistry> = new Collection();

  public constructor(options: ClientOptions) {
    super(options);

    this.loadCommands({ exts: [".js"] });
    this.loadEvents({ exts: [".js"] });
  }

  public static async load(func: Function, options: FileSystem.Options, dirName: string) {
    const dirPaths = FileSystem.findDir(dirName);

    for (const dirPath of dirPaths) {
      const filePaths = FileSystem.findFile(options, dirPath)
        .map(filePath => pathToFileURL(filePath).href);
        // Prevent ERR_UNSUPPORTED_ESM_URL_SCHEME on Windows
    
      for (const filePath of filePaths)
        func((await import(filePath)).default);
    }
  }

  private async loadCommands(options: FileSystem.Options, dirName: string = "commands") {
    this.commands.clear();
    DClient.load((command: SlashCommandRegistry) => {
      this.commands.set(command.data.name, command);
    }, options, dirName);
  }

  private async loadEvents(options: FileSystem.Options, dirName: string = "events") {
    const registryMethodMap = {
      [EventRegistryMethod.off]: this.off,
      [EventRegistryMethod.on]: this.on,
      [EventRegistryMethod.once]: this.once,
    };

    this.removeAllListeners();
    DClient.load((event: EventRegistry) => {
      registryMethodMap[event.method].call(this, event.event, (...args) => event.listener(...args));
    }, options, dirName);
  }
};