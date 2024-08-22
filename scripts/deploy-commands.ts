import "dotenv/config";

const TOKEN = process.env["TOKEN"];
const CLIENT_ID = process.env["CLIENT_ID"];
// const GUILD_ID = process.env["GUILD_ID"];

if (!TOKEN)
	throw new Error(`Missing required field "TOKEN" in .env`);

if (!CLIENT_ID)
	throw new Error(`Missing required field "CLIENT_ID" in .env`);

import { Blueprints } from "../src/helpers/blueprints.js";
import { REST, Routes } from "discord.js";
import { BaseHandler } from "../src/handlers/base.js";

let commands = new Array();
await BaseHandler.loadBlueprints((command: Blueprints.SlashCommandBlueprint) => {
	commands.push(command.data.toJSON());
}, { exts: [".js"] }, "commands");

const rest = new REST().setToken(TOKEN);
(async () => {
	try {
		console.log(`Started refreshing ${commands.length} application (/) commands.`);

		const data: any = await rest.put(
			// Routes.applicationGuildCommands(CLIENT_ID, GUILD_ID),
      Routes.applicationCommands(CLIENT_ID),
			{ body: commands },
		);

		console.log(`Successfully reloaded ${data.length} application (/) commands.`);
	} catch (error) {
		console.error(error);
	}
})();