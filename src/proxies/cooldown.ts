import { Client, Collection, CommandInteraction, Interaction } from "discord.js";
import { AbstractProxy } from "../abstracts/proxies.js";
import { AppSlashCommandBuilder } from "../builders/commands.js";

/**
 * For now, enforce cooldown on slash commands only.
 */
export class CommandCooldownProxy extends AbstractProxy {
  private readonly cooldowns: Collection<string, Collection<string, number>>;
  private readonly defaultCooldown: number = 0;

  constructor() {
    super();
    this.cooldowns = new Collection();
  }

  protected override check(interaction: Interaction, cooldown?: number): boolean | Promise<boolean> {
    const commandInteraction = interaction as CommandInteraction;

    // If the command hasn't been registered,
    // register it
    if (!this.cooldowns.has(commandInteraction.commandName))
      this.cooldowns.set(commandInteraction.commandName, new Collection());
    
    const timestamps = this.cooldowns.get(commandInteraction.commandName)!;
    const interactionTimestamp = commandInteraction.createdTimestamp;

    // Set cooldown according to priority:
    // 1. the one provided by parameter
    // 2. the one set by `AppSlashCommandBuilder`
    // 3. the default one
    cooldown = cooldown ?? (commandInteraction.client as Client & { commands: Collection<string, AppSlashCommandBuilder> }).commands.get(commandInteraction.commandName)?.cooldown ?? this.defaultCooldown;

    // If the user has never used the command
    // (since the app was on)
    if (!timestamps.has(commandInteraction.user.id)) {
      // Add the user
      timestamps.set(commandInteraction.user.id, interactionTimestamp);

      // Remove user after cooldown
      setTimeout(() => timestamps.delete(commandInteraction.user.id), cooldown);

      return false;
    }
    
    const offCooldownTimestamp = timestamps.get(commandInteraction.user.id)! + cooldown;

    if (interactionTimestamp < offCooldownTimestamp)
      return true;

    // If off cooldown already,
    // remove user
    timestamps.delete(commandInteraction.user.id);
    return false;
  }

  protected override onInterceptSuccess(interaction: Interaction): void | Promise<void> {
    const commandInteraction = interaction as CommandInteraction;
    commandInteraction.reply({
      content: "Please wait, you are on cooldown.",
      ephemeral: true,
    });
  }
};