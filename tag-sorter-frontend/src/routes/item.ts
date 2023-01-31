export interface Item {
    id: number;
    path: string;
    year: number;
    month: number;
    day: number;
    tags: string[];
}
export function split_with_blank(input: string) {
    if (input.trim() == "") {
        return [];
    } else {
        return input.split(",").map((x) => x.trim());
    }
}
