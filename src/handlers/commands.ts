import { Client, Collection } from "discord.js";
import { AppSlashCommandBuilder } from "../builders/commands.js";
import { AbstractHandler } from "../abstracts/handlers.js";
import { Property } from "../auxiliaries/reflection.js";

export class CommandsHandler<Client_ extends Client> extends AbstractHandler<Client_, AppSlashCommandBuilder> {
  private readonly commands: Collection<string, AppSlashCommandBuilder>;

  constructor(client: Client_, commandsPropertyKey?: string) {
    super(client);

    this.commands = new Collection();

    commandsPropertyKey = commandsPropertyKey ?? Property.toString(() => this.commands)!;

    // Make `commands` property accessible
    // via `interaction.client.commands`
    Reflect.defineProperty(this.client, commandsPropertyKey, {
      get: () => this.commands,
      configurable: false,
      enumerable: true,
    });
  }

  protected override loadModule(command: AppSlashCommandBuilder): void | Promise<void> {
    this.commands.set(command.name, command);
  }

  protected override preload(): void | Promise<void> {
    this.commands.clear();
  }
};