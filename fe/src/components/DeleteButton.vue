<script setup>
import {ref, watch} from 'vue';
import {toDoItemStore} from '../stores/ToDoItem';

const itemStore = toDoItemStore();
const props = defineProps(["uuid", "deleted"]);
const deleted = ref(props.deleted)

watch(() => props.deleted, (newDeleted) => {
    deleted.value = newDeleted;
});

const handleDelete = async () => {
    await itemStore.remove(props.uuid);
    deleted.value = true;
};

</script>

<template>
    <div>
        <div v-if="deleted !== true">
            <button @click="handleDelete">
                delete
            </button>
        </div>
    </div>
</template>

<style scoped>
button {
    background-color: hsl(351, 94%, 33%);
    border: none;
    color: black;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    cursor: pointer;
    margin: 0.2em 0.5em;
}
</style>