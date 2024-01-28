import {ref} from 'vue'
import {defineStore} from 'pinia'

export const toDoItemStore = defineStore('itemStore', () => {
    let title = ref('')
    let creationDate = ref('')
    let status = ref('')

    async function get(title) {
        console.log(title + 'in store we are');
        const item = await fetch("http://localhost:9095/v1/item/get/" + title).then(res => res.json());

        this.title = item.title
        this.creationDate = item.creation_date
        this.status = item.status
    }

    async function create(title) {
        const item = await fetch(
            "http://localhost:9095/v1/item/create" + title,
            {
                method: "POST"
            }
        ).then(res => res.json());

        this.title = item.title
        this.creationDate = item.creation_date
        this.status = item.status
    }

    async function edit(title) {
        const item = await fetch(
            "http://localhost:9095/v1/item/edit",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    "title": title,
                    "creation_date": '',
                    "status": ''
                })
            }
        ).then(res => res.json());

        this.title = item.title
        this.creationDate = item.creation_date
        this.status = item.status
    }

    return {
        title,
        creationDate,
        status,
        get,
        create,
        edit
    }
})
