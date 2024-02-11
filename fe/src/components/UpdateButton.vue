<script setup>
import {defineProps, ref, watch} from 'vue';
import {toDoItemStore} from '../stores/ToDoItem';

const itemStore = toDoItemStore();
const props = defineProps(["title", "status"]);
const status = ref(props.status)

watch(() => props.status, (newStatus) => {
    status.value = newStatus;
});

const handleUpdate = async () => {
    await itemStore.edit(props.title);
    status.value = itemStore.status;
};

</script>

<template>
    <div>
        <button @click="handleUpdate">
            <div v-if="status === 'Open'">
                done
            </div>
            <div v-else>
                re-open
            </div>
        </button>
    </div>
</template>

<style scoped>
button {
    background-color: hsla(160, 100%, 37%, 1);
    border: none;
    color: black;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    cursor: pointer;
    margin: 0.2em 0.5em;
}
</style>