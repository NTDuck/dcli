import { Events, Interaction } from "discord.js";
import { EventRegistry, EventRegistryMethod } from "../helpers/registry.js";
import { DClient } from "../client.js";

const registry : EventRegistry = {
  event: Events.InteractionCreate,
  listener: async (interaction: Interaction) => {
    if (!interaction.isChatInputCommand())
      return;

    const client = interaction.client as DClient;
    const command = client.commands.get(interaction.commandName);

    if (!command) {
      console.error(`No command matching ${interaction.commandName} was found.`);
      return;
    }

    try {
      await command.callback(interaction);
    } catch (err) {
      console.error(err);
      if (interaction.replied || interaction.deferred) {
        await interaction.followUp({ content: 'There was an error while executing this command!', ephemeral: true });
      } else {
        await interaction.reply({ content: 'There was an error while executing this command!', ephemeral: true });
      }
    } finally {
      // console.log(interaction);
    }
  },
  method: EventRegistryMethod.on,
};

export default registry;