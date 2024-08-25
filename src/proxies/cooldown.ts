import { Client, Collection, CommandInteraction, Interaction, InteractionType } from "discord.js";
import { AbstractProxy } from "../abstracts/proxies.js";
import { AppSlashCommandBuilder } from "../builders/commands.js";

/**
 * For now, enforce cooldown on slash commands only.
 */
export class CooldownProxy extends AbstractProxy {
  private readonly cooldowns: Collection<string, Collection<string, number>>;
  private readonly defaultCooldown: number = 0;

  constructor() {
    super();
    this.cooldowns = new Collection();
  }

  protected override check(interaction: Interaction, cooldown?: number): boolean | Promise<boolean> {
    if (interaction.type !== InteractionType.ApplicationCommand)
      return false;

    // If the command hasn't been registered,
    // register it
    if (!this.cooldowns.has(interaction.commandName))
      this.cooldowns.set(interaction.commandName, new Collection());
    
    const timestamps = this.cooldowns.get(interaction.commandName)!;
    const interactionTimestamp = interaction.createdTimestamp;

    // Set cooldown according to priority:
    // 1. the one provided by parameter
    // 2. the one set by `AppSlashCommandBuilder`
    // 3. the default one
    cooldown = cooldown ?? (interaction.client as Client & { commands: Collection<string, AppSlashCommandBuilder> }).commands.get(interaction.commandName)?.cooldown ?? this.defaultCooldown;

    // If the user has never used the command
    // (since the app was on)
    if (!timestamps.has(interaction.user.id)) {
      // Add the user
      timestamps.set(interaction.user.id, interactionTimestamp);

      // Remove user after cooldown
      setTimeout(() => timestamps.delete(interaction.user.id), cooldown);

      return false;
    }
    
    const offCooldownTimestamp = timestamps.get(interaction.user.id)! + cooldown;

    if (interactionTimestamp < offCooldownTimestamp)
      return true;

    // If off cooldown already,
    // remove user
    timestamps.delete(interaction.user.id);
    return false;
  }

  protected override onInterceptSuccess(interaction: Interaction): void | Promise<void> {
    (interaction as CommandInteraction).reply({
      content: "Please wait, you are on cooldown.",
      ephemeral: true,
    });
  }
};