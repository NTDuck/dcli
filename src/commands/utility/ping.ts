import { Interaction, InteractionType } from "discord.js";
import { AppSlashCommandBuilder } from "../../builders/commands.js";

const command = new AppSlashCommandBuilder()
.setName("ping")
.setDescription("You ping, I pong.")
.setCallback(async (interaction: Interaction) => {
  if (interaction.type !== InteractionType.ApplicationCommand)
    return;
  
  await interaction.reply("pong");
});

export default command;