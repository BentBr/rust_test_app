<script setup>
import {toDoItemStore} from '../stores/ToDoItem';

const itemStore = toDoItemStore();
const props = defineProps(["uuid", "title"]);

</script>

<template>
    <div>
        <a :title="props.title" :href="'/todos/' + props.uuid">{{ props.title }}</a>
    </div>
</template>

<style scoped>
</style>