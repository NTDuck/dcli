import { CacheType, CommandInteraction, CommandInteractionOptionResolver, SlashCommandIntegerOption } from "discord.js";
import { AppSlashCommandBuilder } from "../../builders/commands.js";

const defaultMessageCount: number = 1;
const responseTimeout: number = 2222;

const command = new AppSlashCommandBuilder()
  .setName("delete")
  .setDescription("Delete messages in the current text channel.")
  .setCallback(async (interaction: CommandInteraction) => {
    const options = interaction.options as CommandInteractionOptionResolver<CacheType>;

    const messageCount = options.getInteger("message-count") ?? defaultMessageCount;

    // Should add:
    // 1. filter by user/word/...
    // 2. nuke
    await interaction.channel?.messages.fetch({
      limit: messageCount,
    }).then(messages => {
      // Best way to make Discord unhappy
      messages.forEach(message => message.delete());
      return messages;
    }).then(messages => {
      return interaction.reply({
        content: `Deleted ${messages.size} messages from channel ${interaction.channel}.`
      });
    }).then(response => setTimeout(() => response.delete(), responseTimeout))
      .catch(console.error);
  })
  .addIntegerOption(
    new SlashCommandIntegerOption()
      .setName("message-count")
      .setDescription("The number of messages to delete.")
      .setRequired(false)
      .setMinValue(1)
      .setMaxValue(100)   // Due to Discord API rules
  );

export default command;