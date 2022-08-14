const MAX_TEXT = 165;

export function format_description(description: string): string {
    if (description.length > MAX_TEXT) {
        return description.slice(0, MAX_TEXT - 3) + "...";
    }

    return description;
}