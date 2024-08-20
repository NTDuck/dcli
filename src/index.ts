import "dotenv/config";
import { DClient } from "./client.js";
import { GatewayIntentBits } from "discord.js";

const client = new DClient({
  intents: [
    GatewayIntentBits.Guilds,
  ],
});

await client.start({
  commandsDirName: "commands",
  eventsDirName: "events",
  options: { exts: [".js"] },
});

const TOKEN = process.env.TOKEN;
await client.login(TOKEN);