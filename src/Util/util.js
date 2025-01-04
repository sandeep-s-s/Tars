export function truncateString(str, length) {
    if (typeof str === 'string' && str.length > length) {
        return str.substring(0, length) + "...";
    }
    return str;
}
