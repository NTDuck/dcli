import { Client } from "discord.js";
import { pathToFileURL } from "node:url";
import FileSystem from "../helpers/filesystem.js";

export abstract class BaseHandler<ClientType extends Client> {
  protected readonly client: ClientType;
  
  constructor(client: ClientType) {
    this.client = client;

    // Prevent undefined behaviour
    this.loadBlueprint = this.loadBlueprint.bind(this);
  }

  protected abstract loadBlueprint(blueprint: any): void | Promise<void>;
  protected preloadBlueprints(): void | Promise<void> {}
  protected postloadBlueprints(): void | Promise<void> {}

  public async loadBlueprints(options: FileSystem.FindFileOptions, dirName: string) {
    await this.preloadBlueprints();
    await BaseHandler.loadBlueprints(this.loadBlueprint, options, dirName);
    await this.postloadBlueprints();
  }

  public static async loadBlueprints(loadBlueprint: (blueprint: any) => void | Promise<void>, options: FileSystem.FindFileOptions, dirName: string) {
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
              .then(blueprint => loadBlueprint(blueprint))
          )
        );
      })
    );
  }
};