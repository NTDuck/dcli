import { CommandInteraction, SlashCommandBuilder } from "discord.js";
import { Property } from "../auxiliaries/reflection.js";

export class AppSlashCommandBuilder extends SlashCommandBuilder {
  public readonly cooldown?: number;
  public readonly callback!: (interaction: CommandInteraction) => Promise<void>;
  
  public setCooldown(cooldown: typeof this.cooldown): typeof this {
    Reflect.set(this, Property.toString(() => this.cooldown)!, cooldown);
    return this;
  }

  public setCallback(callback: typeof this.callback): typeof this {
    Reflect.set(this, Property.toString(() => this.callback)!, callback);
    return this;
  }
};