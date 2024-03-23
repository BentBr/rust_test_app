import {ref} from 'vue'
import {defineStore} from 'pinia'

export const toDoItemStore = defineStore('itemStore', () => {
    let title = ref('')
    let description = ref('')
    let uuid = ref('')
    let creationDate = ref('')
    let status = ref('')

    async function get(uuid) {
        console.log(title + 'in store we are');
        const item = await fetch("http://localhost:9095/v1/task/get/" + uuid).then(res => res.json());

        this.title = item.title
        this.description = item.description
        this.uuid = item.uuid
        this.creationDate = item.creation_date
        this.status = item.status
    }

    async function create(title, description) {
        const item = await fetch(
            "http://localhost:9095/v1/task/create/",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    "title": title,
                    "description": description
                })
            }
        ).then(res => res.json());

        this.title = item.title
        this.description = item.description
        this.uuid = item.uuid
        this.creationDate = item.creation_date
        this.status = item.status
    }

    async function remove(uuid) {
        await fetch(
            "http://localhost:9095/v1/task/delete/" + uuid,
            {
                method: "DELETE"
            }
        ).then(res => res.json());

        this.title = ref('')
        this.description = ref('')
        this.uuid = ref('')
        this.creationDate = ref('')
        this.status = ref('')
    }

    async function edit(uuid, title, description, status) {
        const item = await fetch(
            "http://localhost:9095/v1/task/edit/" + uuid,
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    "title": title,
                    "description": description,
                    "uuid": uuid,
                    "status": status
                })
            }
        ).then(res => res.json());

        this.title = item.title
        this.description = item.description
        this.creationDate = item.creation_date
        this.status = item.status
    }

    return {
        title,
        description,
        uuid,
        creationDate,
        status,
        get,
        create,
        edit,
        remove
    }
})
