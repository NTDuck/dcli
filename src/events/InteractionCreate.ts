import { Collection, Events, Interaction, InteractionType } from "discord.js";
import { AppEventBuilder } from "../builders/events.js";
import { AppSlashCommandBuilder } from "../builders/commands.js";
import { CommandCooldownProxy } from "../proxies/cooldown.js";
import { CommandAppPermissionProxy, CommandMemberPermissionProxy } from "../proxies/permissions.js";

const proxies = {
  cooldown: new CommandCooldownProxy(),
  appPermissions: new CommandAppPermissionProxy(),
  memberPermission: new CommandMemberPermissionProxy(),
};

const event = new AppEventBuilder()
  .setName(Events.InteractionCreate)
  .setType("on")
  .setCallback(async (interaction: Interaction) => {
    if (interaction.type !== InteractionType.ApplicationCommand)
      return;

    const command = (interaction.client as typeof interaction.client & { commands: Collection<string, AppSlashCommandBuilder> }).commands.get(interaction.commandName);

    if (!command)
      return;

    // Check for cooldowns
    if (await proxies.cooldown.intercept(interaction))
      return;

    // Check for permissions also
    if (await proxies.appPermissions.intercept(interaction, command))
      return;

    if (await proxies.memberPermission.intercept(interaction, command))
      return;

    // Then handle
    await command.callback(interaction)
      .catch((error: Error) => {
        console.error(error);

        // Consider grouping
        if (interaction.replied || interaction.deferred)
          interaction.followUp({
            content: `Error occurred: ${error}`,
            ephemeral: true,
          });
        else
          interaction.reply({
            content: `Error occurred: ${error}`,
            ephemeral: true,
          });
      })
      // Do something afterwards
      // which is often not necessary
      // Perhaps could e.g. increment an user's command count?
      // but should that be done in the check section?
      .finally(() => {
        // console.log(interaction);
      });
  });

export default event;