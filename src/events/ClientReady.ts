import { Client, Events } from "discord.js";
import { AppEventBuilder } from "../builders/events.js";

const event = new AppEventBuilder()
  .setName(Events.ClientReady)
  .setType("once")
  .setCallback((client: Client) => {
    console.log(`Logged in as ${client.user?.tag}`);
  });

export default event;