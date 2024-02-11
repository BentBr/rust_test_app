<script setup>
    import {toDoItemStore} from '../stores/ToDoItem';
    import {ref, watch, defineEmits} from 'vue';

    const emit = defineEmits(['reload-list']);

    const itemStore = toDoItemStore();
    const title = ref('');
    const createBool = ref(false);

    const handleCreate = async () => {
        await itemStore.create(title.value);
        createBool.value = false;
        emit('reload-list');
    };

    watch(() => createBool.value, () => {
        title.value = ''; // Reset title when createBool changes
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
            <input v-model="title" placeholder="Enter a title for your task" />

            <button class="create" @click="handleCreate">
                Create
            </button>
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
input {
    font: inherit;
    margin: 0.2em 0.5em;
}
</style>