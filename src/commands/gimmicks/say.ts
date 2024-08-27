import { CommandInteraction, CommandInteractionOptionResolver, EmbedBuilder, SlashCommandIntegerOption, SlashCommandStringOption } from "discord.js";
import { AppSlashCommandBuilder } from "../../builders/commands.js";

const command = new AppSlashCommandBuilder()
  .setName("say")
  .setDescription("Make me say something.")
  .setCooldown(1000)
  .setCallback(async function(interaction: CommandInteraction) {
    const options = interaction.options as CommandInteractionOptionResolver;

    const text = options.getString("text", true);
    const responseTimeout = options.getInteger("delete-after", false);

    // The Lawliet format for now
    // might change later
    // will definitely change later
    const embed = new EmbedBuilder()
      .setColor(0xf2f3f4)
      .setDescription(text)
      .setFooter({
        text: `Written by ${interaction.user.displayName}`,
      });

    await interaction.reply({
      embeds: [embed],
    }).then(response => {
      if (responseTimeout)
        setTimeout(() => response.delete(), responseTimeout);
    }).catch(console.error);
  })
  .addStringOption(
    new SlashCommandStringOption()
      .setName("text")
      .setDescription("Something that you want me to say")
      .setRequired(true)
  )
  .addIntegerOption(
    new SlashCommandIntegerOption()
      .setName("delete-after")
      .setDescription("Delete this after some time (ms)")
      .setRequired(false)
  );

export default command;