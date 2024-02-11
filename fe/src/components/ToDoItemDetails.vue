<script setup>
import {ref, onMounted} from 'vue';
import {useRoute} from 'vue-router';
import {toDoItemStore} from '../stores/ToDoItem';
import UpdateButton from "./UpdateButton.vue";
import DeleteButton from "./DeleteButton.vue";

const route = useRoute();
const title = ref(route.params.title); // Access the title parameter from the route

const itemStore = toDoItemStore();

onMounted(async () => {
    // Use the title to fetch data from the store
    await itemStore.get(title.value);
});
</script>

<template>
    <div v-if="itemStore.status !== ''">
        <div class="actions">
            Actions:

            <UpdateButton :title="itemStore.title" :status="itemStore.status"></UpdateButton>

            <div v-if="itemStore.status === 'Done'">
                <DeleteButton :title="itemStore.title" :deleted="false"></DeleteButton>
            </div>
        </div>
    </div>
    <div>

        <hr>

        <div class="content">
            <p><strong>This is your title: </strong>{{ itemStore.title }}</p>
            <p><strong>Status: </strong>{{ itemStore.status }}</p>
            <p><strong>Creation date: </strong>{{ itemStore.creationDate }}</p>
        </div>
    </div>
</template>

<style scoped>
.actions {
    padding: 10px;
}
.content {
    padding: 10px;
}
hr {
    margin-bottom: 10px;
    margin-top: 10px;
}
</style>