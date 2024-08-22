import { Events, Interaction } from "discord.js";
import Blueprints from "../helpers/blueprints.js";

const blueprint : Blueprints.EventBlueprint = {
  event: Events.InteractionCreate,
  listener: async (interaction: Interaction) => {
    if (!interaction.isChatInputCommand())
      return;

    // @ts-ignore
    // "commands" property defined by CommandsHandler
    const command = interaction.client.commands.get(interaction.commandName);

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
  registryMethod: Blueprints.EventRegistryMethod.on,
};

export default blueprint;