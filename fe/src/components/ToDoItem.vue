<script setup>
import {ref, onMounted} from 'vue';
import {useRoute} from 'vue-router';
import {toDoItemStore} from '../stores/ToDoItem';

const route = useRoute();
const title = ref(route.params.title); // Access the title parameter from the route

const itemStore = toDoItemStore();

onMounted(async () => {
    // Use the title to fetch data from the store
    await itemStore.get(title.value);
});
</script>


<template>
    <div>

        <hr>

        <div>
            <p><strong>This is your title: </strong>{{ itemStore.title }}</p>
            <p><strong>Status: </strong>{{ itemStore.status }}</p>
            <p><strong>Creation date: </strong>{{ itemStore.creationDate }}</p>
        </div>
    </div>
</template>

<style scoped>
</style>