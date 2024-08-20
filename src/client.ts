import { Client, ClientOptions, Collection } from "discord.js";
import { Blueprints } from "./helpers/blueprints.js";
import { FileSystem } from "./helpers/filesystem.js";
import { pathToFileURL } from "node:url";

export class DClient extends Client {
  public readonly commands: Collection<string, Blueprints.SlashCommandBlueprint>;

  public constructor(options: ClientOptions) {
    super(options);

    this.commands = new Collection();
  }
  
  public async start(options: {
    commandsDirName: string;
    eventsDirName: string;
    options: FileSystem.Options;
  }) {
    await Promise.all([
      this.loadCommands(options.options, options.commandsDirName),
      this.loadEvents(options.options, options.eventsDirName),
    ]);
  }

  public static async load(func: (module: any) => void, options: FileSystem.Options, dirName: string) {
    const dirPaths = FileSystem.findDir(dirName);
  
    await Promise.all(
      dirPaths.map(async (dirPath) => {
        const filePaths = FileSystem.findFile(options, dirPath)
          .map(filePath => pathToFileURL(filePath).href);
          // Prevent ERR_UNSUPPORTED_ESM_URL_SCHEME on Windows

        const modules = await Promise.all(filePaths.map(
          filePath => import(filePath)
          .then(module => module.default)
        ));
  
        modules.forEach(module => func(module));
      })
    );
  }

  private async loadCommands(options: FileSystem.Options, dirName: string) {
    this.commands.clear();
    await DClient.load((command: Blueprints.SlashCommandBlueprint) => {
      this.commands.set(command.data.name, command);
    }, options, dirName);
  }

  private async loadEvents(options: FileSystem.Options, dirName: string) {
    const registryMethodMap = {
      [Blueprints.EventRegistryMethod.off]: this.off,
      [Blueprints.EventRegistryMethod.on]: this.on,
      [Blueprints.EventRegistryMethod.once]: this.once,
    };

    this.removeAllListeners();
    await DClient.load((event: Blueprints.EventBlueprint) => {
      registryMethodMap[event.method].call(this, event.event, (...args) => event.listener(...args));
    }, options, dirName);
  }
};