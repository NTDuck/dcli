import { Client, Events } from "discord.js";
import Blueprints from "../helpers/blueprints.js";

const blueprint: Blueprints.EventBlueprint = {
  event: Events.ClientReady,
  listener: (client: Client) => {
    console.log(`Logged in as ${client.user?.tag}`);
  },
  registryMethod: Blueprints.EventRegistryMethod.once,
};

export default blueprint;