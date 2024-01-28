<script setup>
import {defineProps} from 'vue';
import {toDoItemStore} from '../stores/ToDoItem';

const itemStore = toDoItemStore();
const props = defineProps(["title"]);

</script>

<template>
    <div>
        <a :href="'/todos/' + props.title">{{ props.title }}</a>
    </div>
</template>

<style scoped>
</style>