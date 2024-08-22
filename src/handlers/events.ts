import { BaseHandler } from "../handlers/base.js";
import { Client } from "discord.js";
import Blueprints from "../helpers/blueprints.js";

export class EventsHandler<ClientType extends Client> extends BaseHandler<ClientType> {
  private readonly eventRegistryMethodMap = {
    [Blueprints.EventRegistryMethod.off]: this.client.off,
    [Blueprints.EventRegistryMethod.on]: this.client.on,
    [Blueprints.EventRegistryMethod.once]: this.client.once,
  };

  constructor(client: ClientType) {
    super(client);
  }

  protected override loadBlueprint(eventModule: Blueprints.EventBlueprint) {
    const eventRegistryMethod = this.eventRegistryMethodMap[eventModule.registryMethod];
    if (eventRegistryMethod)
      eventRegistryMethod.call(this.client, eventModule.event, (...args: any) => eventModule.listener(...args));
  }
  
  protected override preloadBlueprints(): void | Promise<void> {
    this.client.removeAllListeners();
  }
};