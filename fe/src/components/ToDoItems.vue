<script setup>
import {onMounted} from 'vue';
import {toDoItemsStore} from '../stores/ToDoItems';

const itemsStore = toDoItemsStore();

onMounted(async () => {
    // Use the title to fetch data from the store
    await itemsStore.update();
});
</script>

<template>
    <div>

        <hr>

        <div v-if="itemsStore.openItemsCount === 1">
            <h2>You have {{ itemsStore.openItemsCount }} open item</h2>
        </div>
        <div v-else>
            <h2>You have {{ itemsStore.openItemsCount }} open items</h2>
        </div>

        <!-- list to do items -->
        <ul>
            <li v-for="item in itemsStore.openItems"><a :href="'/todos/' + item.title">{{ item.title }}</a></li>
        </ul>

        <hr>

        <div v-if="itemsStore.doneItemsCount === 1">
            <h2>You have {{ itemsStore.doneItemsCount }} done item</h2>
        </div>
        <div v-else>
            <h2>You have {{ itemsStore.doneItemsCount }} done items</h2>
        </div>

    </div>
</template>

<style scoped>
hr {
    margin-bottom: 20px;
    margin-top: 40px;
}
</style>