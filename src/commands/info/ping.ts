import { Interaction, SlashCommandBuilder } from "discord.js";
import { SlashCommandRegistry } from "../../helpers/registry.js";

const registry: SlashCommandRegistry = {
  data: new SlashCommandBuilder()
    .setName("ping")
    .setDescription("ping => pong"),

  callback: async (interaction: Interaction) => {
    if (!interaction.isChatInputCommand())
      return;

    await interaction.reply("pong :3");
  },
};

export default registry;