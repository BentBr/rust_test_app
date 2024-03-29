import {ref} from 'vue'
import {defineStore} from 'pinia'

export const toDoItemsStore = defineStore('itemsStore', () => {
    let openItems = []
    let doneItems = []
    let openItemsCount = ref(0)
    let doneItemsCount = ref(0)

    async function update() {
        const items = await fetch("http://localhost:9095/v1/task/get").then(res => res.json());

        this.openItems = items.data.open_items
        this.doneItems = items.data.done_items
        this.openItemsCount = items.data.open_items_count
        this.doneItemsCount = items.data.done_items_count
    }

    return {
        openItems,
        doneItems,
        openItemsCount,
        doneItemsCount,
        update
    }
})
