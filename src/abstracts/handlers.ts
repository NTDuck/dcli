import { Client } from "discord.js";
import FileSystem from "../auxiliaries/filesystem.js";
import { pathToFileURL } from "node:url";

export abstract class AbstractHandler<Client_ extends Client, Module> {
  protected client: Client_;
  
  constructor(client: typeof this.client) {
    this.client = client;
    this.loadModule = this.loadModule.bind(this);
  }

  public async loadModules(options: FileSystem.Options, dirName: string): Promise<void> {
    await this.preload?.();
    await AbstractHandler.loadModules(this.loadModule, options, dirName);
    await this.postload?.();
  }

  protected abstract loadModule(module: Module): void | Promise<void>;
  protected preload?(): void | Promise<void>;
  protected postload?(): void | Promise<void>;

  public static async loadModules<Module>(moduleLoader: (module: Module) => void | Promise<void>, options: FileSystem.Options, dirName: string) : Promise<void> {
    const dirPaths = FileSystem.findDir(dirName);
  
    await Promise.all(
      dirPaths.map(async (dirPath) => {
        const filePaths = FileSystem.findFile(options, dirPath)
          .map(filePath => pathToFileURL(filePath).href);
          // Prevent ERR_UNSUPPORTED_ESM_URL_SCHEME on Windows

        await Promise.all(
          filePaths.map(
            filePath => import(filePath)
              .then(blueprint => blueprint.default)
              .then(blueprint => moduleLoader(blueprint))
          )
        );
      })
    );
  }
};