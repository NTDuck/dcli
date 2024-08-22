import "dotenv/config";
import { AppClient } from "./client.js";
import { GatewayIntentBits } from "discord.js";

const client = new AppClient({
  intents: [
    GatewayIntentBits.Guilds,
  ],
});

await client.loadBlueprints(
  { exts: [".js"] }, {
    commandsDirName: "commands",
    eventsDirName: "events",
  }
);
  
const TOKEN = process.env["TOKEN"];
await client.login(TOKEN);