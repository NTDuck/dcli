import { Client, Events } from "discord.js";
import { Blueprints } from "../helpers/blueprints.js";

const registry: Blueprints.EventBlueprint = {
  event: Events.ClientReady,
  listener: (client: Client) => {
    console.log(`Logged in as ${client.user?.tag}`);
  },
  method: Blueprints.EventRegistryMethod.once,
};

export default registry;