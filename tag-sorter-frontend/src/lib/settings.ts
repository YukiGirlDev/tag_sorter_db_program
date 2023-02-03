import { browser } from '$app/environment';
import { writable, type Writable } from 'svelte/store';

export const IP = 'http://localhost:8000';

export const items_per_page_default = 20;
export let items_per_page: Writable<number>;
if (browser) {
	items_per_page = writable(
		parseInt(localStorage.getItem('items_per_page') || items_per_page_default.toString())
	);
	items_per_page.subscribe((value) => {
		localStorage.setItem('items_per_page', value.toString());
	});
}
