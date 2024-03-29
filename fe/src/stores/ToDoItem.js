import {ref} from 'vue'
import {defineStore} from 'pinia'

export const toDoItemStore = defineStore('itemStore', () => {
    let title = ref('')
    let description = ref('')
    let uuid = ref('')
    let creationDate = ref('')
    let status = ref('')

    function updateFullData(data) {
        title.value = data.title
        description.value = data.description
        uuid.value = data.uuid
        creationDate.value = data.creation_date
        status.value = data.status
    }

    async function get(uuid) {
        const item = await fetch("http://localhost:9095/v1/task/get/" + uuid).then(res => res.json());

        updateFullData(item.data)
    }

    async function create(title, description) {
        const item = await fetch(
            "http://localhost:9095/v1/task/create",
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

        updateFullData(item.data)
    }

    async function remove(uuid) {
        await fetch(
            "http://localhost:9095/v1/task/delete/" + uuid,
            {
                method: "DELETE"
            }
        ).then(res => res.json());

        title.value = ''
        description.value = ''
        uuid.value = ''
        creationDate.value = ''
        status.value = ''
    }

    async function edit(uuid, title, description, status) {
        const item = await fetch(
            "http://localhost:9095/v1/task/edit/" + uuid,
            {
                method: "PATCH",
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

        title.value = item.data.title
        description.value = item.data.description
        creationDate.value = item.data.creation_date
        status.value = item.data.status
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
