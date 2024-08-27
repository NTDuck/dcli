import { CommandInteraction } from "discord.js";
import { AppSlashCommandBuilder } from "../../builders/commands.js";

const command = new AppSlashCommandBuilder()
.setName("ping")
.setDescription("You ping, I pong.")
.setCallback(async (interaction: CommandInteraction) => {
  await interaction.reply("pong");
});

export default command;