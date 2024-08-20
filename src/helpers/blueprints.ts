import { ClientEvents, Interaction, SlashCommandBuilder } from "discord.js";

export namespace Blueprints {
  export interface SlashCommandBlueprint {
    readonly data: SlashCommandBuilder;
    readonly callback: (interaction: Interaction) => Promise<void>;
  }
  
  export enum EventRegistryMethod {
    off, on, once,
  }
  
  export interface EventBlueprint {
    readonly event: keyof ClientEvents;
    readonly listener: Function;
    readonly method: EventRegistryMethod;
  }
}

export default Blueprints;