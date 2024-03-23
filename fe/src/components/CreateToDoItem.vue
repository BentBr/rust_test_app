<script setup>
    import {toDoItemStore} from '../stores/ToDoItem';
    import {ref, watch} from 'vue';

    const emit = defineEmits(['reload-list']);

    const itemStore = toDoItemStore();
    const title = ref('');
    const description = ref('');
    const createBool = ref(false);

    const handleCreate = async () => {
        await itemStore.create(title.value, description.value);
        createBool.value = false;
        emit('reload-list');
    };

    watch(() => createBool.value, () => {
        title.value = ''; // Reset title when createBool changes
        description.value = '';
    });
</script>

<template>
    <div>
        <div v-if="createBool === false">
            <button class="create" @click="createBool = true">
                Create
            </button>
        </div>
        <div v-if="createBool === true">
            <button class="cancel" @click="createBool = false">
                Cancel
            </button>
        </div>

        <div v-if="createBool === true">
            <div class="row">
                <input v-model="title" placeholder="Enter a title for your task" />
            </div>
            <div class="row">
                <textarea v-model="description" placeholder="Awesome task description" />
            </div>
            <div class="row">
                <button class="create" @click="handleCreate">
                    Create
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
button {
    border: none;
    color: black;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    cursor: pointer;
    margin: 0.2em 0.5em;
}
.create {
    background-color: hsl(203, 76%, 51%);
}
.cancel {
    background-color: hsl(351, 94%, 33%);
}
input,textarea {
    font: inherit;
    margin: 0.2em 0.5em;
}
button {
    margin: 0.2em 0.5em;
}
</style>