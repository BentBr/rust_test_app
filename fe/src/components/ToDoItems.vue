<script setup>
import {onMounted} from 'vue';
import {toDoItemsStore} from '../stores/ToDoItems';
import UpdateToDoItem from "./UpdateButton.vue";
import ToDoItemLink from "./ToDoItemLink.vue";
import CreateToDoItem from "./CreateToDoItem.vue";

const itemsStore = toDoItemsStore();

onMounted(async () => {
    // Use the title to fetch data from the store
    await itemsStore.update();
});
</script>

<template>
    <div>

        <create-to-do-item @reload-list="itemsStore.update()"></create-to-do-item>

        <hr>

        <div v-if="itemsStore.openItemsCount === 1">
            <h2>You have {{ itemsStore.openItemsCount }} open item</h2>
        </div>
        <div v-else>
            <h2>You have {{ itemsStore.openItemsCount }} open items</h2>
        </div>

        <!-- list to do items -->
        <ul>
            <li v-for="item in itemsStore.openItems">
                <ToDoItemLink :title="item.title"></ToDoItemLink>
                <UpdateToDoItem :title="item.title" :status="item.status"></UpdateToDoItem>
            </li>
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