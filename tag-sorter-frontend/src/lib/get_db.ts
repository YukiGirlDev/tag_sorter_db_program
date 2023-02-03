import type { Item } from "src/routes/item";
import { writable, type Writable } from "svelte/store";
import { IP } from "./settings";

export let search_tag_recomendations: Writable<string[]> = writable([]);
export let items: Item[] = [];
export let page_count: Writable<number> = writable(0);

export interface SearchRequest {
    name: string,
    tags: string[],
    entering: string,
    start_date?: Date,
    end_data?: Date,
    page: number,
    items_per_page: number,
    sort: boolean,
}


function map<T, U>(val: T|undefined|null, func: (x: T) => U): U|undefined {
    if (typeof(val)!=='undefined'&&val!==null) {
        return func(val);
    }else {
        return undefined
    }
}
export async function get_db(
    request: SearchRequest,
    run: (new_items: Item[]) => void
) {
    
    const res = await fetch(IP + '/api', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            name: request.name,
            tags: request.tags,
            entering: request.entering,
            start_year: map(request.start_date, x => x.getFullYear()),
            end_year: map(request.end_data, x => x.getFullYear()),
            start_month: map(request.start_date, x => x.getMonth()),
            end_month: map(request.end_data, x => x.getMonth()),
            start_day:map(request.start_date, x => x.getDate()),
            end_day: map(request.end_data, x => x.getDate()),
            items_per_page: request.items_per_page,
            page: request.page,
            sort: request.sort
        })
    });
    const aitems = await res.json();
    if (res.ok) {
        let current_len = items.length;
        if (!aitems.is_ok) {
            throw new Error(aitems.error);
        }
        let response: { items: Item[]; tags: string[]; page_count: number } = aitems.response;
        run(response.items)
        items = response.items;
        
        search_tag_recomendations.set(response.tags);
        page_count.set(response.page_count);
        return items;
    } else {
        throw new Error(aitems);
    }
}