<script setup>
import {ref, onMounted} from 'vue';
import {useRoute} from 'vue-router';
import {toDoItemStore} from '../stores/ToDoItem';
import UpdateButton from "./UpdateButton.vue";
import DeleteButton from "./DeleteButton.vue";
import UpdateToDoItem from "./UpdateButton.vue";

const route = useRoute();
const uuid = ref(route.params.uuid); // Access the uuid parameter from the route

const itemStore = toDoItemStore();

onMounted(async () => {
    // Use the title to fetch data from the store
    await itemStore.get(uuid.value);
});
</script>

<template>
    <div v-if="itemStore.status !== ''">
        <div class="actions">
            Actions:

            <UpdateToDoItem :uuid="itemStore.uuid" :title="itemStore.title" :description="itemStore.description" :status="itemStore.status"></UpdateToDoItem>


            <div v-if="itemStore.status === 'Done'">
                <DeleteButton :uuid="itemStore.uuid" :deleted="false"></DeleteButton>
            </div>
        </div>
    </div>
    <div>

        <hr>

        <div class="content">
            <p><strong>{{ itemStore.title }}</strong></p>
            <p>{{ itemStore.description }}</p>
            <p><span class="light">Status: </span>{{ itemStore.status }}</p>
            <p><span class="light">Creation date: </span>{{ itemStore.creationDate }}</p>
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