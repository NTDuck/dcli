import { Client, Events } from "discord.js";
import { EventRegistry, EventRegistryMethod } from "../helpers/registry.js";

const registry: EventRegistry = {
  event: Events.ClientReady,
  listener: (client: Client) => {
    console.log(`Logged in as ${client.user?.tag}`);
  },
  method: EventRegistryMethod.once,
};

export default registry;