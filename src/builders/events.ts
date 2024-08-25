import { Events } from "discord.js";
import { Property } from "../auxiliaries/reflection.js";

export class AppEventBuilder {
  public readonly name!: Events;
  public readonly type!: "off" | "on" | "once";
  public readonly callback!: (...args: any) => void;

  public setName(name: typeof this.name): typeof this {
    Reflect.set(this, Property.toString(() => this.name)!, name);
    return this;
  }
  
  public setType(type: typeof this.type): typeof this {
    Reflect.set(this, Property.toString(() => this.type)!, type);
    return this;
  }
  
  public setCallback(callback: typeof this.callback): typeof this {
    Reflect.set(this, Property.toString(() => this.callback)!, callback);
    return this;
  }
};