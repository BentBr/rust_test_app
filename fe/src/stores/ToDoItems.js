import {reactive, ref} from 'vue'
import {defineStore} from 'pinia'

export const toDoItemsStore = defineStore('itemsStore', () => {
    let openItems = reactive([]);
    let doneItems = reactive([]);
    let inProgressItems = reactive([]);

    let openItemsCount = ref(0);
    let doneItemsCount = ref(0);
    let inProgressItemsCount = ref(0);

    async function update() {
        const items = await fetch("http://localhost:9095/v1/task/get").then(res => res.json());

        openItems.splice(0, openItems.length, ...items.data.open_items);
        doneItems.splice(0, doneItems.length, ...items.data.done_items);
        inProgressItems.splice(0, inProgressItems.length, ...items.data.in_progress_items);

        openItemsCount.value = items.data.open_items_count;
        doneItemsCount.value = items.data.done_items_count;
        inProgressItemsCount.value = items.data.in_progress_items_count;
    }

    return {
        openItems,
        doneItems,
        inProgressItems,
        openItemsCount,
        doneItemsCount,
        inProgressItemsCount,
        update
    }
})
