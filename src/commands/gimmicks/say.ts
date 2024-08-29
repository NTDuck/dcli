import { CacheType, ColorResolvable, CommandInteraction, CommandInteractionOptionResolver, EmbedBuilder, GuildMember, PermissionFlagsBits, SlashCommandIntegerOption, SlashCommandStringOption } from "discord.js";
import { AppSlashCommandBuilder } from "../../builders/commands.js";

const defaultEmbedColor: ColorResolvable = 0xf2f3f4;

const command = new AppSlashCommandBuilder()
  .setName("say")
  .setDescription("Make me say something.")
  .setCooldown(1000)
  .setCallback(async (interaction: CommandInteraction) => {
    const options = interaction.options as CommandInteractionOptionResolver<CacheType>;

    const content = options.getString("content", true);
    const responseTimeout = options.getInteger("delete-after", false);

    await interaction.reply({
      embeds: [
        new EmbedBuilder()
          .setColor((interaction.member as GuildMember)?.displayColor ?? defaultEmbedColor)
          .setDescription(content)
          .setFooter({
            text: `written by ${interaction.user.username}`,
          })
      ]
    }).then(response => {
      if (responseTimeout)
        setTimeout(() => response.delete(), responseTimeout);
    }).catch(console.error);
  })
  .addStringOption(
    new SlashCommandStringOption()
      .setName("content")
      .setDescription("Something that you want me to say")
      .setRequired(true)
  )
  .addIntegerOption(
    new SlashCommandIntegerOption()
      .setName("delete-after")
      .setDescription("Delete this after some time (ms)")
      .setRequired(false)
  )
  .setDefaultMemberPermissions(
      PermissionFlagsBits.SendMessages
    | PermissionFlagsBits.ViewChannel
  );

export default command;