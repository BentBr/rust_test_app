<script setup>
import {onMounted} from 'vue';
import {toDoItemsStore} from '../stores/ToDoItems';
import TransitionToDoItem from "./TransitionButton.vue";
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
                <ToDoItemLink :uuid="item.uuid" :title="item.title"></ToDoItemLink>
                <div class="row">
                    <TransitionToDoItem :uuid="item.uuid" :status="item.status" new_status="In Progress" @reload-list="itemsStore.update()"></TransitionToDoItem>
                    <TransitionToDoItem :uuid="item.uuid" :status="item.status" new_status="Done" @reload-list="itemsStore.update()"></TransitionToDoItem>
                </div>

            </li>
        </ul>

        <hr>

        <div v-if="itemsStore.inProgressItemsCount === 1">
            <h2>You have {{ itemsStore.inProgressItemsCount }} item in progress</h2>
        </div>
        <div v-else>
            <h2>You have {{ itemsStore.inProgressItemsCount }} items in progress</h2>
        </div>

        <!-- list to do items -->
        <ul>
            <li v-for="item in itemsStore.inProgressItems">
                <ToDoItemLink :uuid="item.uuid" :title="item.title"></ToDoItemLink>
                <div class="row">
                    <TransitionToDoItem :uuid="item.uuid" :status="item.status" new_status="Open" @reload-list="itemsStore.update()"></TransitionToDoItem>
                    <TransitionToDoItem :uuid="item.uuid" :status="item.status" new_status="Done" @reload-list="itemsStore.update()"></TransitionToDoItem>
                </div>
            </li>
        </ul>

        <hr>

        <div v-if="itemsStore.doneItemsCount === 1">
            <h2>You have {{ itemsStore.doneItemsCount }} done item</h2>
        </div>
        <div v-else>
            <h2>You have {{ itemsStore.doneItemsCount }} done items</h2>
        </div>

        <!-- list to do items -->
        <ul>
            <li v-for="item in itemsStore.doneItems">
                <ToDoItemLink :uuid="item.uuid" :title="item.title"></ToDoItemLink>
                <div class="row">
                    <TransitionToDoItem :uuid="item.uuid" :status="item.status" new_status="Open" @reload-list="itemsStore.update()"></TransitionToDoItem>
                    <TransitionToDoItem :uuid="item.uuid" :status="item.status" new_status="In Progress" @reload-list="itemsStore.update()"></TransitionToDoItem>
                </div>
            </li>
        </ul>

    </div>
</template>

<style scoped>
hr {
    margin-bottom: 20px;
    margin-top: 40px;
}
</style>