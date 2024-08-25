import { Interaction } from "discord.js";

export abstract class AbstractProxy {
  public async intercept(interaction: Interaction, ...args: any): Promise<boolean> {
    if (await this.check(interaction, ...args)) {
      await this.onInterceptSuccess?.(interaction);
      return true;
    } else {
      await this.onInterceptFailure?.(interaction);
      return false;
    }
  }

  protected abstract check(interaction: Interaction, ...args: any): boolean | Promise<boolean>;
  protected onInterceptSuccess?(interaction: Interaction): void | Promise<void>;
  protected onInterceptFailure?(interaction: Interaction): void | Promise<void>;
};