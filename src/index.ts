import "dotenv/config";
import { DClient } from "./client.js";
import { GatewayIntentBits } from "discord.js";

const client = new DClient({
  intents: [
    GatewayIntentBits.Guilds,
  ],
});

const TOKEN = process.env.TOKEN;
client.login(TOKEN);