import { Client, Collection } from "discord.js";
import { BaseHandler } from "./base.js";
import Blueprints from "../helpers/blueprints.js";

export class CommandsHandler<ClientType extends Client> extends BaseHandler<ClientType> {
  private readonly commands = new Collection<string, Blueprints.SlashCommandBlueprint>();

  constructor(client: ClientType) {
    super(client);
    
    this.defineProperty();
  }

  protected override loadBlueprint(commandBlueprint: Blueprints.SlashCommandBlueprint) {
    this.commands.set(commandBlueprint.data.name, commandBlueprint);
  }
  
  protected override preloadBlueprints() {
    this.commands.clear();
  }

  private defineProperty(propertyName: string = "commands") {
    Object.defineProperty(this.client, propertyName, {
      get: () => this.commands,
      configurable: true,
      enumerable: true,
    });
  }
};