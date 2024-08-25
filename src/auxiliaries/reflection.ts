export namespace Property {
  /**
   * @returns A substring of `getter.toString()`
   * from the last invalid character (if any)
   * to the next invalid character.
   * A valid character is any character that
   * when appended to a valid identifier
   * does not change its validity.
   */
  export const toString = (getter: () => any) => {
    const match = getter.toString().match(/[^$\w\u00A0-\uFFFF]+([$\w\u00A0-\uFFFF]+)[^$\w\u00A0-\uFFFF]*$/);
    return match ? match[1] : null;
  }
}

export type MappedType<Source, Dest, Key extends string | number | symbol = string> = {
  [key in keyof Source as Key]: Dest;
};