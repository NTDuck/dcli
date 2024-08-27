"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Property = void 0;
var Property;
(function (Property) {
    /**
     * @returns A substring of `getter.toString()`
     * from the last invalid character (if any)
     * to the next invalid character.
     * A valid character is any character that
     * when appended to a valid identifier
     * does not change its validity.
     */
    Property.toString = function (getter) {
        var match = getter.toString().match(/[^$\w\u00A0-\uFFFF]+([$\w\u00A0-\uFFFF]+)[^$\w\u00A0-\uFFFF]*$/);
        // Temporary debug, will remove later
        if (!match)
            console.error(getter.toString());
        return match ? match[1] : null;
    };
})(Property || (exports.Property = Property = {}));
