<script setup lang="ts">
import ToDoItem from "../components/ToDoItemDetails.vue";

import {useRoute} from 'vue-router';

const route = useRoute();
const title = route.params.title;
</script>

<template>
    <div class="todo">
        <h1>{{ title }}</h1>

        <!-- Suspense component to show to do items  -->
        <Suspense>
            <template #fallback>
                <div>loading</div>
            </template>

            <ToDoItem/>
        </Suspense>
    </div>
</template>

<style scoped>

</style>