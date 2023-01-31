export interface Filter {
    id?: number,
    id_range?: FilterRange<number>,
    name?: string,
    start_name?: string,
    contains_name?: string,
    date?: FilterDate,
    date_range?: FilterRange<FilterDate>,
    tag?: string[],
    tag_sub?: string[],
    tag_super?: string[],
}
export interface FilterRange<T> {
    start: T,
    end: T,
}
export interface FilterDate {
    year: number,
    month: number,
    day: number,
}
