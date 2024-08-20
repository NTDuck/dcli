import { ClientEvents, Interaction, SlashCommandBuilder } from "discord.js";

export interface SlashCommandRegistry {
  data: SlashCommandBuilder;
  callback: (interaction: Interaction) => Promise<void>;
}

export enum EventRegistryMethod {
  on,
  off,
  once,
}

export interface EventRegistry {
  event: keyof ClientEvents;
  listener: Function;
  method: EventRegistryMethod;
}