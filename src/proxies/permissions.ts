import { Colors, CommandInteraction, EmbedBuilder, inlineCode, Interaction, PermissionResolvable } from "discord.js";
import { AppSlashCommandBuilder } from "../builders/commands.js";
import { AbstractProxy } from "../abstracts/proxies.js";

export class CommandAppPermissionProxy extends AbstractProxy {
  protected override check(interaction: Interaction, command: AppSlashCommandBuilder) {
    const appPermissions = interaction.appPermissions;

    return appPermissions
      ? !appPermissions.has(command.default_member_permissions as PermissionResolvable)
      : false;
  }

  protected override onInterceptSuccess(interaction: Interaction): void | Promise<void> {
    const commandInteraction = interaction as CommandInteraction;
    commandInteraction.reply({
      embeds: [
        new EmbedBuilder()
          .setColor(Colors.Red)
          .setTitle(`/${commandInteraction.commandName} is a no-no`)
          .setDescription(`I have insufficient permissions to use the command ${inlineCode(`/${commandInteraction.commandName}`)}.`)
      ],
      ephemeral: true,
    });
  }
};

export class CommandMemberPermissionProxy extends AbstractProxy {
  protected override check(interaction: Interaction, command: AppSlashCommandBuilder) {
    const memberPermissions = interaction.memberPermissions;

    return memberPermissions
      ? !memberPermissions.has(command.default_member_permissions as PermissionResolvable)
      : false;
  }

  protected override onInterceptSuccess(interaction: Interaction): void | Promise<void> {
    const commandInteraction = interaction as CommandInteraction;
    commandInteraction.reply({
      embeds: [
        new EmbedBuilder()
          .setColor(Colors.Red)
          .setTitle(`/${commandInteraction.commandName} is a no-no`)
          .setDescription(`User ${commandInteraction.user.username} has insufficient permissions to use the command ${inlineCode(`/${commandInteraction.commandName}`)}.`)
      ],
      ephemeral: true,
    });
  }
};