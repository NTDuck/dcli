import { Interaction, SlashCommandBuilder } from "discord.js";
import { Blueprints } from "../../helpers/blueprints.js";

const blueprint: Blueprints.SlashCommandBlueprint = {
  data: new SlashCommandBuilder()
    .setName("ping")
    .setDescription("ping => pong"),
  callback: async (interaction: Interaction) => {
    if (!interaction.isChatInputCommand())
      return;

    await interaction.reply("pong :3");
  },
};

export default blueprint;