import "dotenv/config";
import { AppClient } from "./client.js";
import { GatewayIntentBits } from "discord.js";

const client = new AppClient({
  intents: [
    GatewayIntentBits.Guilds,
  ],
});

await client.loadModules(
  { exts: [".js"] }, {
    commands: "commands",
    events: "events",
  },
);
  
const TOKEN = process.env["TOKEN"];
await client.login(TOKEN);